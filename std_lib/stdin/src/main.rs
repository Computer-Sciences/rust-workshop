// read command line inputs
use std::io;
fn main() {
    let mut buffer = String::new();
    println!("Enter your name:");

    // ===== access the stdin stream =====

    // the read_line() function will update the buffer with the input string
    // read_line() blocks the execution of the program until a string is entered at the command line

    let read_line_result = io::stdin().read_line(&mut buffer);
    println!("read_line_result is {:?}", read_line_result);

    println!("Welcome to Rust, {}", buffer);

    // ===== parse input string ========

    // clear buffer from previous input
    buffer.clear();

    println!("Enter the year when you started learning Rust:");

    let read_line_result = io::stdin().read_line(&mut buffer);
    println!("read_line_result is {:?}", read_line_result);

    // need to trim the input string because contains a newline at the end
    // notice the turbofish operator (::<i32>) to indicate the type of data to parse from the input string (here an i32 integer)

    // parse() returns a Result enum
    // Result enum allow to handle errors (more on this later)
    // use the unwrap function to extract the value

    let start_year = buffer.trim().parse::<i32>().unwrap();
    println!("You started your Rust journey in {}", start_year);

    // without using the  turbofish operator, indicate the destination type as the variable type

    let start_year: i32 = buffer.trim().parse().unwrap();
    println!("You will be an expert in {}", start_year + 5);
}
