fn main() {
    let number = 123u8;

    // match expression returning values based on case
    // match arms are evaluated sequentially from top to bottom

    let result = match number {
        // match arm
        0 => "zero",
        // match arm
        1 => "one",
        // match arm
        2 => "two",
        // wildcard pattern
        // should be used last, otherwise the following arms will not be evaluated
        // notice the code block when more than one line
        _ => {
            println!("{} did not match any arm", number);
            "unmatched"
        }
    };

    println!("result is {}", result);
}
