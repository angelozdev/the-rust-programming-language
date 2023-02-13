extern crate rand;

use rand::Rng;
use std::{cmp::Ordering, num::ParseIntError};

const MAX_ATTEMPS: u8 = 8;

fn main() {
    let secret_number = rand::thread_rng().gen_range(1..=100);
    let mut attemps: u8 = 0;

    loop {
        if attemps >= MAX_ATTEMPS {
            println!("You lose !");
            break;
        };

        println!("Guess the number (1-100):");
        let guess = match get_guessed_number() {
            Ok(guess) => guess,
            Err(error) => {
                println!("⚠️  {}", error);
                continue;
            }
        };

        attemps += 1;
        println!("You guessed: {}, attemps: {}", guess, attemps);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}

fn get_guessed_number() -> Result<u32, ParseIntError> {
    let mut guess = String::new();
    std::io::stdin()
        .read_line(&mut guess)
        .expect("Error on reading line");
    return guess.trim().parse::<u32>();
}
