extern crate rand;

use self::rand::{thread_rng, ThreadRng, Rng};

static LETTERS: [char; 14] = ['A', 'C', 'E', 'F', 'H', 'K', 'L', 'M', 'P', 'R', 'T', 'W', 'X', 'Y'];
static NUMBERS: [char; 5]  = ['3', '4', '6', '7', '9'];

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
    let mut rng = thread_rng();

    let code = code_format.chars().map( |character|
        match character {
            'B' => random_letter(&mut rng),
            '1' => random_number(&mut rng),
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
fn random_letter(rng: &mut ThreadRng) -> char {
    *rng.choose(&LETTERS).expect("Failed to select a random letter")
}

/// Returns a random letter from the available numbers in the codeset.
///
/// # Return value
///
/// A single `char`
fn random_number(rng: &mut ThreadRng) -> char {
    *rng.choose(&NUMBERS).expect("Failed to select a random number")
}
