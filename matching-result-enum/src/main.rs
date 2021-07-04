use std::fs;
use std::io;

fn main() {
    let result = fs::read_to_string("does-not-exist.txt");

    let content = match result {
        Ok(text) => text,

        // handle I/O error based on kind
        Err(error) => match error.kind() {
            io::ErrorKind::NotFound => String::from("File not found"),

            io::ErrorKind::PermissionDenied => String::from("Permission denied"),

            _ => error.to_string(),
        },
    };

    println!("content is \"{}\"", content);
}
