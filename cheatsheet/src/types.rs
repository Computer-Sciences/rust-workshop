pub fn run() {
    // default integer numeric type is i32

    let num1 = 123;
    println!("{} - type: {}", num1, get_type(&num1));

    // default floating point numeric type is f64

    let num2 = 1.23;
    println!("{} - type: {}", num2, get_type(&num2));

    // explicit typing

    let num3: i8 = 23;
    println!("{} - type: {}", num3, get_type(&num3));

    // max values

    let max_i32 = std::i32::MAX;
    let max_i16 = std::i16::MAX;
    println!("max value for i32 is {}", max_i32);
    println!("max value for i16 is {}", max_i16);

    // boolean

    let is_rust_fun: bool = true;
    println!(
        "is_rust_fun is {} - type: {}",
        is_rust_fun,
        get_type(&is_rust_fun)
    );
    let is_greater = 23 > 5;
    println!(
        "is_greater is {} - type: {}",
        is_greater,
        get_type(&is_greater)
    );

    // characters (unicode - 4 bytes length)

    let smiley = '😈';
    println!("smiley is {} - type: {}", smiley, get_type(&smiley));
}

fn get_type<T>(_: &T) -> &str {
    std::any::type_name::<T>()
}
