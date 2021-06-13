use std::env;

fn main() {
    // check for required number of arguments
    if env::args().len() <= 2 {
        println!("this program requires at least two arguments");
        return;
    }

    // args() returns an iterator over arguments passed to the program executable binary file
    // use enumerate() to get input args and their index
    // first argument is the executable path (but not always, depending on systems)

    for (index, arg) in env::args().enumerate() {
        println!("argument {} is {}", index, arg);
    }

    // get argument at specific index

    let arg2 = env::args().nth(2).unwrap();
    println!("\narg2 is {}", arg2);
}
