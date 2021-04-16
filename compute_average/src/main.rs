fn main() {
    let a = 33;
    let b = 4.9;
    let c: f32 = 123.5;
    let average = (a as f32 + b as f32 + c) / 3.0;
    println!("average is {}", average);
    assert_eq!(average, 53.8);
    println!("test passed.");
}
