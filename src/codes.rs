use std::rand::{task_rng, Rng};

pub fn generate_code(code_format: &str) -> String {
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
