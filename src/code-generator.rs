use std::rand::{task_rng, Rng};
use std::os;
use std::collections::hashmap::HashSet;

fn main() {
	let num_threads = 8u;

	let args = os::args();
	let usage = "Usage: code-generator NUMCODES CODEFORMAT";

	let num_codes: Option<uint> = from_str(args[1].as_slice().trim());
	let total_codes = match num_codes {
		Some(n) => n,
		None => {
			println!("{}", usage);
			return;
		}
	};

	let ref code_format = args[2].as_slice().trim();
	if code_format.len() < 1 {
		println!("{}", usage);
		return;
	}

	let mut generated_codes: HashSet<String> = HashSet::with_capacity(total_codes);

	for _ in range(0, total_codes) {
		println!("{}", generate_code(&mut generated_codes, *code_format));
	}
}

fn generate_code(existing_codes: &mut HashSet<String>, code_format: &str) -> String {
	let mut code = "".to_string();
	for character in code_format.chars() {
		let random_char = match character {
			'B' => random_letter(),
			'1' => random_number(),
			other_char => other_char
		};
		code.grow(1, random_char);
	}

	if existing_codes.contains(&code) {
		return generate_code(existing_codes, code_format);
	} else {
		existing_codes.insert(code.clone());
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
