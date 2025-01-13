use guessing_game::guess::Guess;
use guessing_game::{generate_secret_number, compare_guess};
use std::io;

fn main() {
    println!("Welcome to the Guessing Game!");

    let secret_number = generate_secret_number();

    loop {
        println!("Please input your guess (between 1 and 100):");

        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        // Parse the input into an i32
        let guess: i32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a valid number!");
                continue;
            }
        };

        // Validate the guess using the `Guess` type
        let guess = match Guess::new(guess) {
            Ok(guess) => guess,
            Err(_) => {
                println!("Please enter a number between 1 and 100!");
                continue;
            }
        };

        // Compare the guess to the secret number
        match compare_guess(&guess, secret_number) {
            std::cmp::Ordering::Less => println!("Too small!"),
            std::cmp::Ordering::Greater => println!("Too big!"),
            std::cmp::Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}