fn main() {
    // can only do arithmetic operations on same type operands
    let a = 11;
    let b = 33;
    let c = a + b;
    println!("c is {}", c);

    let d = c - b;
    println!("d is {}", d);

    let e = a * d;
    println!("e is {}", e);

    // type casting (careful with precision loss and type compatibility)
    let f = c as f32 / 4.5;
    println!("f is {}", f);

    // operator precedence control
    let g = 43.5432 % (a as f64 * e as f64);
    println!("g is {}", g);
}
