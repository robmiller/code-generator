use std::env;
use std::process;
use std::collections::HashSet;
use std::sync::mpsc::{channel, Sender, Receiver};
use std::thread::spawn;

mod codes;

/// When passed a code format and number of codes on the command line,
/// generates that many codes in that particular format.
///
/// Codes are guaranteed to be unique within the output.
///
/// # Examples
///
/// 	./code-generator 5 BB11BB11BB11
/// 	RC93AL64EW63
/// 	LA34YK47TE93
/// 	HR74AP94LT49
/// 	WX49XH39FR46
/// 	LA63KC47PW34
/// 
/// 	./code-generator 5 XX11BB11
/// 	XX73LH46
/// 	XX64HF77
/// 	XX44FE74
/// 	XX96YM39
/// 	XX37FF67
///
/// # Exit status
///
/// 1 if the command line arguments were invalid; 0 on success.
fn main() {
    let total_codes;
    let code_format;

    let usage = "Usage: code-generator NUMCODES CODEFORMAT";

    match parse_args() {
        (n, Some(c)) => {
            total_codes = n;
            code_format = c;
        },
        _ => {
            println!("{}", usage);
            process::exit(1);
        }
    }

    let (code_tx, code_rx)       = channel();
    let (exit_tx, exit_rx)       = channel();
    let (printer_tx, printer_rx) = channel();

    spawn(move || {
        code_generator(code_format, code_tx);
    });

    spawn(move || {
        code_exists_handler(total_codes, code_rx, printer_tx);
    });

    spawn(move || {
        print_handler(printer_rx, exit_tx);
    });

    // Once enough codes have been generated, the `code_exists_handler`
    // sends a message on the exit channel. The main thread will block
    // until it receives this.
    let _ = exit_rx.recv();
    return;
}

/// Spawned in a task; generates codes infinitely and without regard for
/// their uniqueness, sending each one to the code checking task.
///
/// Stops generating codes at the point that its `Sender` is dropped.
///
/// # Arguments
///
/// * `code_format` - The code format, ultimately passed to
///                   `codes::generate_code`.
/// * `tx` - A `Sender`; each code generated will be sent to this.
fn code_generator(code_format: String, tx: Sender<String>) {
    loop {
        let code = codes::generate_code(&code_format);
        match tx.send(code.clone()) {
            Ok(_) => {}
            Err(_) => {
                break;
            }
        }
    }
}

/// Spawned in a task; maintains a list of which codes have already been
/// generated. When passed a new code by `code_generator`, it checks
/// whether the code already exists; if it does, it does nothing; if it
/// doesn't, it passes it through to the `print_handler` to be outputted
/// to the screen.
///
/// Finally, this takes care of the total number of codes; once that
/// number of codes has been generated and passed to the print handler,
/// it will send the exit message to the main process, causing the whole
/// program to exit.
///
/// # Arguments
///
/// * `total_codes` - That total number of codes to generate.
/// * `rx` - A `Receiver` down which generated codes are passed; see
///          `code_generator`.
/// * `printer_tx` - A `Sender` down which unique codes are passed,
///                  presumably to be outputted to the screen.
fn code_exists_handler(total_codes: usize, rx: Receiver<String>, printer_tx: Sender<String>) {
    let mut existing_codes: HashSet<String> = HashSet::with_capacity(total_codes);

    loop {
        let code = rx.recv().unwrap();

        if !existing_codes.contains(&code) {
            existing_codes.insert(code.clone());
            let _ = printer_tx.send(code);
        }

        if existing_codes.len() >= total_codes {
            let _ = printer_tx.send("last-code".to_string());
            break;
        }
    }
}

/// Outputs codes to the screen.
///
/// # Arguments
///
/// * `rx` - A `Receiver` to which codes should be passed; each code
///          passed will be outputted verbatim.
/// * `exit_tx` - Once all codes have been generated, `true` will be
///               passed to this `Sender`.
fn print_handler(rx: Receiver<String>, exit_tx: Sender<bool>) {
    loop {
        match rx.recv() {
            Ok(code) => {
                if &code == "last-code" {
                    let _ = exit_tx.send(true);
                } else {
                    println!("{}", code);
                }
            },
            Err(_) => {
                break;
            }
        }
    }
}

/// Parses command-line arguments; currently, that means the number of
/// codes to generate and the format in which to generate them.
///
/// # Return value
///
/// Returns a tuple of two `Option`s; the first an `Option<usize>` for
/// the number of codes to generate, and the second an `Option<String>`
/// for the format to generate codes in.
fn parse_args() -> (usize, Option<String>) {
    let args: Vec<_> = env::args().collect();

    if args.len() < 3 {
        return (0, None);
    }

    let num_codes: usize = (&args[1]).trim().parse().unwrap_or(0);

    let code_format =
        if (&args[2]).len() < 1 {
            None
        } else {
            Some(args[2].clone())
        };

    (num_codes, code_format)
}

