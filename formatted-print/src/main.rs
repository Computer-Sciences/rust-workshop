fn main() {
    let x = 246.92385;
    let y = 24.69;
    let z = x / y;
    // print line macro with 3 decimal point precision
    println!("z is {:.3}", z);
    // 9: total character space the number to occupy (add pre padding if necessary)
    println!("z is {:9.3}", z);
    // 0: placeholder number for padding characters
    println!("z is {:09.3}", z);
    println!("z is {:09.3}\nx is {}", z, x);
    // print macro without new line
    print!("y is {:09.3}\n\tx is {}\n", y, x);

    // positional paramters
    println!("z is {0:05.1} and x is {1:.2}. \nx is also {1}", z, x)
}
