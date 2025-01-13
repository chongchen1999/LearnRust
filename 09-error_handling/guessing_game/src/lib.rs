pub mod guess;

use rand::Rng;
use std::cmp::Ordering;
use crate::guess::Guess;

/// Generates a secret number between 1 and 100.
pub fn generate_secret_number() -> i32 {
    rand::thread_rng().gen_range(1..=100)
}

/// Compares the user's guess to the secret number and returns a result.
pub fn compare_guess(guess: &Guess, secret_number: i32) -> Ordering {
    guess.value().cmp(&secret_number)
}