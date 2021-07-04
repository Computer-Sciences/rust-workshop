// recoverable errors

// errors that do not cause the program to fail
// and can be corrected

// Rust uses the Result<T,E> enum type to handle recoverable errors

// enum Result<T, E> {
//     Ok(T),
//     Err(E),
// }

// the Ok variant stores the value of the successful operation
// the Err variant stores the value of the error

// the Result enum is included in the prelude

use std::fs;
use std::io;

fn main() {
    // read_to_string() returns a Result<T, E> enum

    let content: Result<String, io::Error> = fs::read_to_string("file.txt");

    // avoid the unwrap() method
    // because will panic if this is the Err variant of the Result enum

    println!("content is {:?}", content.unwrap());

    // for custom error message, use expect()
    // not the best way to handle recoverable errorsn

    let content: Result<String, io::Error> = fs::read_to_string("file-x.txt");

    println!("content is {:?}", content.expect("failed reading file"));
}
