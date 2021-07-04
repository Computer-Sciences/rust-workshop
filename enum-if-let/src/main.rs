// if let syntax
// use when matching only one variant

fn main() {
    let number = Some(1);

    // notice the syntax:
    // if let PATTERN = VALUE {}

    if let Some(1) = number {
        println!("You are the one");
    }
}
