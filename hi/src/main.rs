fn main() {
    // variables are immutable by default
    let x = 20;
    println!("x is {}",x);

    let mut local_var = 20;
    println!("local_var is {}",local_var);

    local_var = 50;
    println!("local_var is {}",local_var);
}
