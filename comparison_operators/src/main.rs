/*
can only compare values of same type
*/
fn main() {
    let a = 11;
    let b = 88;
    println!("a is {}\nb is {}", a, b);
    println!("a EQUAL TO b is {}", a == b);
    println!("a NOT EQUAL TO b is {}", a != b);
    println!("a GREATER THAN b is {}", a > b);
    println!("a GREATER THAN OR EQUAL TO b is {}", a >= b);
    println!("a LESS THAN b is {}", a < b);
    println!("a LESS THAN OR EQUAL TO b is {}", a <= b);

    let c = true;
    let d = false;

    println!("\nc is {}\nd is {}", c, d);
    println!("c EQUAL TO d is {}", c == d);
    println!("c NOT EQUAL TO d is {}", c != d);
    println!("c GREATER THAN d is {}", c > d);
    println!("c GREATER THAN OR EQUAL TO d is {}", c >= d);
    println!("c LESS THAN d is {}", c < d);
    println!("c LESS THAN OR EQUAL TO d is {}", c <= d);
}
