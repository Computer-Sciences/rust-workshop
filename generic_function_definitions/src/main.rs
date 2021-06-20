// restrict the generic type
// to types that implement the Trait for values that can be compared for a sort-order (PartialOrd)
// PartialOrd trait is included in the std crate prelude (no need for full path)
// otherwise, how would the compiler know if the concrete types can be compared using > ?

fn get_biggest<T: PartialOrd>(n1: T, n2: T) -> T {
    if n1 > n2 {
        n1
    } else {
        n2
    }
}

fn main() {
    println!("biggest is {}", get_biggest(23u8, 121u8));
    println!("biggest is {}", get_biggest(12345, 121));
    println!("biggest is {}", get_biggest(123.45, 1.21));
}
