use std::fs;
use std::path::Path;

fn main() {
    let path = Path::new("./fruits.txt");
    let contents = fs::read_to_string(path).expect("Failed to read file.");
    println!("contents is: {}", contents);

    // process individual lines

    for line in contents.lines() {
        println!("line is {}", line);
    }

    // read file as bytes

    let contents: Vec<u8> = fs::read("fruits.txt").expect("Failed to read file.");
    println!("\ncontents is: {:?}", contents);
}
