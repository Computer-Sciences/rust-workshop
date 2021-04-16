fn main() {
    // unicode scalar value stored using 4 bytes (32 bits)
    // contrary to C like languages that storeit in 1 byte

    let letter: char = 'z';
    let number = '9';
    let finger = '\u{261D}';
    println!("letter is {}", letter);
    println!("number is {}", number);
    println!("finger is {}", finger);
}
