use std::rand::{task_rng, Rng};

pub fn generate_code(code_format: &str) -> String {
    let code = code_format.chars().map( |character|
        match character {
            'B' => random_letter(),
            '1' => random_number(),
            other_char => other_char
        }
    ).collect();

    code
}

fn random_letter() -> char {
    let mut rng = task_rng();
    let letters = ['A', 'C', 'E', 'F', 'H', 'K', 'L', 'M', 'P', 'R', 'T', 'W', 'X', 'Y'];
    *rng.choose(letters).expect("Failed to select a random letter")
}

fn random_number() -> char {
    let mut rng = task_rng();
    let numbers = ['3', '4', '6', '7', '9'];
    *rng.choose(numbers).expect("Failed to select a random number")
}
