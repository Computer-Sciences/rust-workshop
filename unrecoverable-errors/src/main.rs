// two types of runtime errors in Rust
// Recoverable (ex: file not found error)
// Unrecoverable (ex: index out of array bounds)

// most languages do not distinguish between these errors
// and use Exceptions to handle all of them

// Rust does not have traditional Exceptions
// Rust uses the Result<T, E> enum type for recoverable errors
// Rust uses panic for unrecoverable errors

fn main() {
    // panic! macro
    // immediately terminates the program and provides feedback to caller of program
    // panic!("Something went wrong.");

    let countdown: [i32; 4] = [3, 2, 1, 0];
    for left in countdown.iter() {
        println!("T-minus {}", left);

        // the program will panic when diving by zero
        let q = 1 / left;
        println!("q is {}", q);
    }
}
