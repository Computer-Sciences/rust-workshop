// 'static lifetime
// indicates references available for the entire duration of the program

// example: a string literal is stored in the program binary
// so it never goes away
// the data is available from start to finish
// all string literals have a static lifetime:
//  let literal: &'static str = "I am a string slice";

// will never become invalid and lives longer than other lifetimes

// 'static lifetime as a trait bound
// ensures the data type will only contain 'static elements
// so that the receiver can hold onto it
// and use it as long as they want knowing it will never become invalid:
// T: Display + 'static

// can be coerced to more restrictive lifetime

fn get_toto<'a>() -> &'a str {
    "toto here"
}

fn main() {
    let toto: &'static str = get_toto();
    println!("toto is {}", toto);
}
