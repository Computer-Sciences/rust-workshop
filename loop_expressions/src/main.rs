fn main() {
    let mut count = 0;

    // infinite loop
    loop {
        if count == 10 {
            break;
        }
        count += 1;
        println!("count is {}", count);
    }

    println!("\nAfter first loop.\n");

    // returning a value from loop expression
    let result = loop {
        if count == 15 {
            break count * 20;
        }
        count += 1;
        println!("count is {}", count);
    };

    println!("\nAfter second loop, result is {}", result);
}
