use rand::Rng;
use std::io;

fn main() {
    println!("Guess a number");
    println!("Please enter your guess:");

    let secret_number = rand::thread_rng().gen_range(1, 101);
    println!("The secret number is {}", secret_number);

    // "::" is used for associated functions of a given type (equiv to static method in OOP)
    // String::new() creates an empty string of type String (growable UTF-8 encoded text)
    let mut guess = String::new();

    /*
        std::io::stdin, if you don't use the import at the top of file
        std::io::stdin() returns an instance of a std::io::Stdin type
    */
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guess: {}", guess);
}
