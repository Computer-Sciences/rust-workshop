fn main() {
    be_polite();

    // infered types for y and z are the ones used as parameters of add()
    let y = 12;
    let z = 34;
    add(y, z);

    // passing later y and z to another fn with different param types will panic
    // guess_number(z) // -> expects a i32 not a u8
    // need for explicit cast:
    guess_number(y as i32)
}

fn be_polite() {
    println!("Greetings, pleased to meet you.");
    guess_number(25)
}

fn guess_number(number: i32) {
    println!("Indeed, {} is the correct answer", number)
}

fn add(a: u8, b: u8) {
    let sum = a + b;
    println!("sum is {}", sum)
}
