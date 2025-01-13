use guessing_game::Guess;
use rand::Rng;
use std::io;

fn main() {
    println!("Welcome to the guessing game!");
    println!("Guess the number between 1 and 100.");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Please input your guess:");

        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: i32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a valid number.");
                continue;
            }
        };

        // Directly call Guess::new without a match block
        let guess = Guess::new(guess);

        println!("You guessed: {}", guess.value());

        match guess.value().cmp(&secret_number) {
            std::cmp::Ordering::Less => println!("Too small!"),
            std::cmp::Ordering::Greater => println!("Too big!"),
            std::cmp::Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
