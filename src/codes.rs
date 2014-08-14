use std::rand::{task_rng, Rng};

pub fn generate_code(code_format: &str) -> String {
    let mut code = String::with_capacity(code_format.len());
    for character in code_format.chars() {
        code.push_char(match character {
            'B' => choose(['A', 'C', 'E', 'F', 'H', 'K', 'L',
                           'M', 'P', 'R', 'T', 'W', 'X', 'Y']),
            '1' => choose(['3', '4', '6', '7', '9']),
            other_char => other_char,
        });
    }

    code
}

fn choose(chars: &[char]) -> char {
    match task_rng().choose(chars) {
        Some(&c) => c,
        None => unreachable!(),
    }
}
