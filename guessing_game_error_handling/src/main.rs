use rand::prelude::*;
use std::io;
fn main() {
    guess_with_infinite_retry();
}
fn guess_with_infinite_retry() {
    let secret_number = thread_rng().gen_range(1..101);
    let mut invalid_numberretry_limit = 0;
    println!("Guess a number between 1 and 100:");
    loop {
        let mut buffer = String::new();

        let guess = match io::stdin().read_line(&mut buffer) {
            Ok(_) => match buffer.trim().parse::<u32>() {
                Ok(number) => number,
                Err(e) => {
                    invalid_numberretry_limit += 1;
                    if invalid_numberretry_limit == 3 {
                        panic!("Too many invalid numbers entered");
                    }
                    println!("Failed to parse guess: {}", e);
                    println!("Please enter a valid number:");
                    continue;
                }
            },
            Err(e) => {
                println!("Failed to read line: {}", e);
                continue;
            }
        };

        if guess < secret_number {
            println!("guess too low, guess higher:");
        } else if guess > secret_number {
            println!("guess too high, guess lower:");
        } else {
            println!("you found it!");
            break;
        }
    }
}
