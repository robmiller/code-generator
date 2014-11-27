use std::rand::{task_rng, Rng};

/// Generates a single code in the specified format.
///
/// Given a code format, replaces any instance of the letter "B" with
/// a randomly selected letter, replaces any instance of the number "1"
/// with a randomly selected number, and leaves all other characters
/// intact.
///
/// No uniqueness checking is done, so repeated calls to `generate_code`
/// could produce the same string; it's the responsibility of the
/// calling code to ensure that codes are unique if this quality is
/// desired.
///
/// # Arguments
///
/// * `code_format` - The format to generate codes in, e.g.
///                   "BB11BB11BB11", "ZZ111111", etc.
///
/// # Return value
///
/// The generated code.
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

/// Returns a random letter from the available letters in the codeset.
///
/// # Return value
///
/// A single `char`
fn random_letter() -> char {
    let mut rng = task_rng();
    let letters = ['A', 'C', 'E', 'F', 'H', 'K', 'L', 'M', 'P', 'R', 'T', 'W', 'X', 'Y'];
    *rng.choose(&letters).expect("Failed to select a random letter")
}

/// Returns a random letter from the available numbers in the codeset.
///
/// # Return value
///
/// A single `char`
fn random_number() -> char {
    let mut rng = task_rng();
    let numbers = ['3', '4', '6', '7', '9'];
    *rng.choose(&numbers).expect("Failed to select a random number")
}
