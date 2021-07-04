#[derive(Debug)]
enum Shape {
    // storing data in enum variants by declaring parameter types
    Triangle(f64, f64, f64), // sides a, b, c
    Rectangle(f64, f64),     // width, height
    Circle(f64),             // radius
}

fn main() {
    let a_shape = Shape::Triangle(1.1, 2.2, 3.3);
    println!("a_shape is {:?}", a_shape);

    let a_shape = Shape::Rectangle(1.3, 5.7);
    println!("a_shape is {:?}", a_shape);

    let a_shape = Shape::Circle(1.13);
    println!("a_shape is {:?}", a_shape);

    // match operator
    // compares a value to a series of patterns to determine which code to execute
    // similar to switch statements in other languages

    // match expression used to control flow of program

    match a_shape {
        // enumerate all possible values

        // capture the stored data using a positional parameter name (a, b, c)
        Shape::Triangle(a, b, c) => println!(
            "a_shape is a triangle with sides a = {}, b = {}, c = {}",
            a, b, c
        ),
        Shape::Rectangle(width, height) => println!(
            "a_shape is a rectangle with width = {} and height = {}",
            width, height
        ),
        Shape::Circle(radius) => println!("a_shape is a circle with radius {}", radius),
    }
}
