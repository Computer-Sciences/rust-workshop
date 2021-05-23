pub fn run() {
    println!("Hello, world!");

    // string interpolation

    println!("Adding {} and {} gives {}", 22, 33, 22 + 33);

    // positional arguments

    println!(
        "Ypur name is {0}. Welcome to {1}. Nice to meet you {0}",
        "Goto", "Rust"
    );

    // named arguments

    println!(
        "{language} is very popular. It was created in {year}",
        language = "Rust",
        year = 2010
    );

    // placeholder traits

    println!("Decimal: {0}\t Binary: {0:b}\t Hexadecimal: {0:x}", 11);

    // debug trait (very useful to print anything)
    // if you try to print the array directly, you will get an error
    // because an aray is not a string or number

    println!("{:?}", [11, 22, 33])
}
