use std::fs;
use std::io;

fn read_and_combine(f1: &str, f2: &str) -> Result<String, io::Error> {
    let mut s1 = match fs::read_to_string(f1) {
        Ok(s) => s,
        // propagating the error to caller
        Err(e) => return Err(e),
    };

    // Rust provides a shorthand syntax for propagating errors
    // by removing the match expression
    // and putting a question mark (?) after the Result enum
    // it is equivalent to the above syntax
    // the ? can only be used with functions that return a Result enum

    let s2 = fs::read_to_string(f2)?;

    s1.push('\n');
    s1.push_str(&s2);
    Ok(s1)
}

fn main() {
    match read_and_combine("file1.txt", "file2.txt") {
        Ok(text_result) => println!("result is:\n{}", text_result),
        Err(e) => println!("Got error: {}", e),
    };
}
