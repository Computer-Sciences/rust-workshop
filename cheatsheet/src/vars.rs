pub fn run() {
    // variable are immutable by default

    let pc = "Inspirion XYZ";
    println!("pc is {}", pc);

    // mutable variable

    let mut age = 1;
    println!("age is {}", age);
    age = 2;
    println!("age is {}", age);

    // constants (must be uppercase and explicit type definition)

    const BRAND: &str = "Dell";
    println!("brand is {}", BRAND);

    // multiple assignment (tuple destructuring)

    let (status, code) = ("OK", 200);
    println!("status: {}, code: {}", status, code);
}
