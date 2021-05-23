fn main() {
    // by default fractional values stored in f64
    let my_float = 12.345677890123456789012345;
    println!("my_float is: {}", my_float);

    let a_float: f32 = 9.9438535983578493758;
    println!("a_float is: {}", a_float);

    let min_f32 = std::f32::MIN;
    println!("min_f32 is: {}\n", min_f32);

    let max_f32 = std::f32::MAX;
    println!("max_f32 is: {}\n", max_f32);

    let min_f64 = std::f64::MIN;
    println!("min_f64 is: {}\n", min_f64);

    let max_f64 = std::f64::MAX;
    println!("max_f64 is: {}\n", max_f64);
}
