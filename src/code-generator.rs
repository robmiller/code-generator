use std::rand::{task_rng, Rng};
use std::os;
use std::collections::hashmap::HashSet;

fn main() {
	let total_codes: uint;
	let code_format: String;

	match parse_args() {
		(Some(n), Some(c)) => {
			total_codes = n;
			code_format = c;
		},
		_ => {
			return;
		}
	}

	let (tx1, rx1) = channel();
	let (tx2, rx2) = channel::<bool>();

	spawn(proc() {
		code_generator(total_codes, code_format, tx1, rx2);
	});

	spawn(proc() {
		code_exists_handler(total_codes, tx2, rx1);
	});
}

fn code_generator(total_codes: uint, code_format: String, tx: Sender<String>, rx: Receiver<bool>) {
	for _ in range(0, total_codes) {
		let mut code: String;
		loop {
			code = generate_code(code_format.as_slice());
			tx.send(code.clone());

			let exists = rx.recv();
			if !exists {
				break;
			}
		}
	}
}

fn code_exists_handler(total_codes: uint, tx: Sender<bool>, rx: Receiver<String>) {
	let mut existing_codes: HashSet<String> = HashSet::with_capacity(total_codes);

	loop {
		let code = rx.recv();

		if existing_codes.contains(&code) {
			tx.send(true);
		} else {
			existing_codes.insert(code.clone());
			println!("{}", code);
			tx.send(false);
		}
	}
}

fn generate_code(code_format: &str) -> String {
	let mut code = String::new();
	for character in code_format.chars() {
		let random_char = match character {
			'B' => random_letter(),
			'1' => random_number(),
			other_char => other_char
		};
		code.grow(1, random_char);
	}

	code
}

fn random_letter() -> char {
	let mut rng = task_rng();
	let letters = ['A', 'C', 'E', 'F', 'H', 'K', 'L', 'M', 'P', 'R', 'T', 'W', 'X', 'Y'];
	let i = rng.gen_range(0, letters.len());
	letters[i]
}

fn random_number() -> char {
	let mut rng = task_rng();
	let numbers = ['3', '4', '6', '7', '9'];
	let i = rng.gen_range(0, numbers.len());
	numbers[i]
}

fn parse_args() -> (Option<uint>, Option<String>) {
	let args = os::args();

	let usage = "Usage: code-generator NUMCODES CODEFORMAT";

	let num_codes: Option<uint> = from_str(args[1].as_slice().trim());
	let total_codes: Option<uint> = match num_codes {
		Some(n) => Some(n),
		None => {
			println!("{}", usage);
			os::set_exit_status(1);
			return (None, None);
		}
	};

	let code_format: Option<String> = Some(args[2].clone());
	if code_format.as_slice().len() < 1 {
		println!("{}", usage);
		os::set_exit_status(1);
		return (None, None);
	}

	(num_codes, code_format)
}
