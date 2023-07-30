extern crate rand;

use rand::random;
use std::io;

fn get_guess() -> u8 {
    loop {
        println!("Input guess:");
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read from stdin");

        match guess.trim().parse::<u8>() {
            Ok(v) => return v,
            Err(e) => println!("Could not understand input: {}", e),
        };
    }
}

fn handle_guess(guess: u8, correct: u8) -> bool {
    if guess < correct {
        println!("Too low!");
        false
    } else if guess > correct {
        println!("Too high!");
        false
    } else {
        println!("Correct!");
        true
    }
}

fn main() {
    println!("Welcome to the guessing game!");
    println!("I'm thinking of a number between 0 and 255");

    let correct_value = random::<u8>();
    let mut guess_count = 0;

    loop {
        if guess_count > 5 {
            println!("You lose!");
            break;
        }

        println!("Guesses remaining: {}", 5 - guess_count);
        let guess = get_guess();
        guess_count += 1;
        if handle_guess(guess, correct_value) {
            break;
        }
    }
}
