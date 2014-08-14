#![feature(macro_rules)]

use std::os;
use std::collections::hashmap::HashSet;

mod codes;

fn main() {
    let (total_codes, code_format) = match parse_args() {
        Some(args) => args,
        None => return,
    };

    let (code_tx, code_rx) = channel();
    let (printer_tx, printer_rx) = channel();

    spawn(proc() {
        code_generator(code_format, code_tx);
    });

    spawn(proc() {
        code_exists_handler(total_codes, code_rx, printer_tx);
    });

    spawn(proc() {
        print_handler(printer_rx);
    });
}

fn code_generator(code_format: String, tx: Sender<String>) {
    loop {
        let code = codes::generate_code(code_format.as_slice());
        if tx.send_opt(code).is_err() {
            break;
        }
    }
}

fn code_exists_handler(total_codes: uint, rx: Receiver<String>, printer_tx: Sender<String>) {
    let mut existing_codes = HashSet::with_capacity(total_codes);

    while existing_codes.len() < total_codes {
        let code = rx.recv();
        if existing_codes.insert(code.clone()) {
            printer_tx.send(code);
        }
    }
}

fn print_handler(rx: Receiver<String>) {
    for code in rx.iter() {
        println!("{}", code);
    }
}

fn parse_args() -> Option<(uint, String)> {
    macro_rules! usage {
        () => {{
            println!("Usage: code-generator NUMCODES CODEFORMAT");
            os::set_exit_status(1);
            return None;
        }}
    }
    let args = os::args();

    if args.len() < 3 {
        usage!()
    }

    let num_codes = match from_str::<uint>(args[1].as_slice().trim()) {
        Some(n) => n,
        None => usage!(),
    };

    let code_format = args[2].clone();
    if code_format.as_slice().len() < 1 {
        usage!()
    }

    Some((num_codes, code_format))
}
