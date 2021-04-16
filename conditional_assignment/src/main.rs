fn main() {
    let is_x_odd = true;
    // in Rust, if conditional is an expression so it returns a value
    let x = if is_x_odd { 1 } else { 2 };

    println!("x is {}", x)
}
