fn main() {
    let a = true;
    let b = false;

    println!("a is {}\nb is {}", a, b);
    println!("NOT a is {}", !a);
    println!("a AND b is {}", a & b);
    println!("a OR b is {}", a | b);
    println!("a XOR b is {}", a ^ b);

    // boolean casted to integer begets 0 or 1
    println!("a XOR b is {}", (a ^ b) as i32); // 1

    let c = (a ^ b) | (a & b);
    println!("c is {}", c);

    // short-circuiting logical operations:
    // right operand not evaluated
    let d = true || (a & b);
    println!("d is {}", d);
    // the panic is not evaluated, so the process with 0
    let e = false && panic!();
    println!("e is {}", e);
}
