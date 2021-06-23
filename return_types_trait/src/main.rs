use std::fmt::Display;

// the return value must implement the Display trait

fn get_displayable() -> impl Display {
    1
}

fn main() {
    println!("Display: {}", get_displayable());
}
