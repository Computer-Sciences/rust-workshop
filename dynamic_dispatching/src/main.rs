use std::fmt::Display;

// dynamic dispatch

// when the retun type cannot be known until runtime
// notice the dyn keyword and the use of a Box pointers

// Returns a value that implements the Display trait,
// but we do not know which one at compile time.

fn get_dynamic_displayable(a: bool) -> Box<dyn Display> {
    if a {
        // returns u32
        Box::new(1)
    } else {
        // returns string slice
        Box::new("one")
    }
}

fn main() {
    println!("Display: {}", get_dynamic_displayable(false));
}
