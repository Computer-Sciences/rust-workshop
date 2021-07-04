#[derive(Debug)]
enum Shape {
    Triangle(f64, f64, f64), // sides a, b, c
    Rectangle(f64, f64),     // width, height
    Circle(f64),             // radius
}

impl Shape {
    fn get_perimeter(&self) -> f64 {
        // no need to dereference the self variable
        // the compiler will automatically reference/dereference patterns in match expressions
        // match *self {

        match self {
            Shape::Rectangle(width, height) => (width + height) * 2.0,
            Shape::Triangle(a, b, c) => a + b + c,
            Shape::Circle(radius) => radius * 2.0 * std::f64::consts::PI,
        }
    }
}

fn main() {
    let a_shape = Shape::Triangle(1.1, 2.2, 3.3);
    println!("a_shape is {:?}", a_shape);
    println!("a_shape perimeter is {}", a_shape.get_perimeter());

    let a_shape = Shape::Rectangle(1.3, 5.7);
    println!("a_shape is {:?}", a_shape);
    println!("a_shape perimeter is {}", a_shape.get_perimeter());

    let a_shape = Shape::Circle(1.13);
    println!("a_shape is {:?}", a_shape);
    println!("a_shape perimeter is {:.2}", a_shape.get_perimeter());
}
