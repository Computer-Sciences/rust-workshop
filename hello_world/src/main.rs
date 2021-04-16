fn main() {
    // variables are immutable by default
    let x = -20; // default inferred type is i32
    println!("x is {}", x);

    let mut local_var: u8 = 20;
    println!("local_var is {}", local_var);
    local_var = 50;
    println!("local_var is {}", local_var);

    let a_num: u16 = 2021;
    println!("a_num is {}", a_num);
}
