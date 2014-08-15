use std::os;
use std::collections::hashmap::HashSet;

mod codes;

fn main() {
    let total_codes;
    let code_format;

    match parse_args() {
        (Some(n), Some(c)) => {
            total_codes = n;
            code_format = c;
        },
        _ => {
            println!("{}", usage());
            os::set_exit_status(1);
            return;
        }
    }

    let (code_tx, code_rx)       = channel();
    let (exit_tx, exit_rx)       = channel();
    let (printer_tx, printer_rx) = channel();

    spawn(proc() {
        code_generator(code_format, code_tx);
    });

    spawn(proc() {
        code_exists_handler(total_codes, code_rx, printer_tx, exit_tx);
    });

    spawn(proc() {
        print_handler(printer_rx);
    });

    // Once enough codes have been generated, the `code_exists_handler`
    // sends a message on the exit channel. The main thread will block
    // until it receives this.
    exit_rx.recv();
    return;
}

fn code_generator(code_format: String, tx: Sender<String>) {
    loop {
        let code = codes::generate_code(code_format.as_slice());
        let send = tx.send_opt(code.clone());
        if send == Err(code) {
            break;
        }
    }
}

fn code_exists_handler(total_codes: uint, rx: Receiver<String>, printer_tx: Sender<String>, exit_tx: Sender<bool>) {
    let mut existing_codes: HashSet<String> = HashSet::with_capacity(total_codes);

    loop {
        let code = rx.recv();

        if !existing_codes.contains(&code) {
            existing_codes.insert(code.clone());
            printer_tx.send(code);
        }

        if existing_codes.len() >= total_codes {
            exit_tx.send(true);
            break;
        }
    }
}

fn print_handler(rx: Receiver<String>) {
    loop {
        match rx.recv_opt() {
            Ok(code) => {
                println!("{}", code);
            },
            Err(_) => {
                break;
            }
        }
    }
}

fn parse_args() -> (Option<uint>, Option<String>) {
    let args = os::args();

    if args.len() < 3 {
        return (None, None);
    }

    let num_codes: Option<uint> = from_str(args[1].as_slice().trim());

    let code_format =
        if args[2].as_slice().len() < 1 {
            None
        } else {
            Some(args[2].clone())
        };

    (num_codes, code_format)
}

fn usage() -> String {
    "Usage: code-generator NUMCODES CODEFORMAT".to_string()
}
