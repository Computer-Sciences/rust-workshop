fn main() {
    // fixed length and single typed
    // stored in contiguous memory locations

    let letters = ['a', 'b', 'c']; // type: [char; 3]
    let first_letter = letters[0];
    println!("first_letter is {}", first_letter);

    // to modify elements in array, it must be mutable
    let mut numbers = [11, 22, 44]; // type: [i32; 3]
    numbers[2] = 33;
    println!("numbers is {}", numbers[2]);

    // empty array declaration (memory allocated)
    let words: [&str; 2];
    words = ["ok"; 2]; // repeat expression, equivalent to ["ok", "ok"]
    println!("words is {}", words[1]);

    /*
    length of usize is based on number of bytes needed to reference memory in your target architecture:
    - for 32 bit compilation target -> usize is 4 bytes
    - for 64 bit compilation target -> usize is 8 bytes
    */
    let ints = [22; 5];
    let length: usize = ints.len();
    println!("length is {}", length);
}
