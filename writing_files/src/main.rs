use std::fs;
use std::path;
// import the Write trait of the io module prelude (more on traits later)
use std::io::prelude::*;

fn main() {
    let path = path::Path::new("./file.txt");

    let mut contents = String::new();
    contents.push_str("this is line 1\n");
    contents.push_str("this is line 2\n");
    contents.push_str("this is line 3\n");

    fs::write(path, contents).expect("Failed to write to file.");

    // append content to file

    let path = path::Path::new("./fruits.txt");

    // new() function of the OpenOptions struct in the fs module
    let mut file = fs::OpenOptions::new()
        .append(true)
        .open(path)
        .expect("Failed to open file.");

    // uses the imported Write trait
    // expects an array of bytes as argument
    // notice the b in front of the string to convert it to its byte representation

    file.write(b"\nolive\n").expect("Failed to write to file.");
}
