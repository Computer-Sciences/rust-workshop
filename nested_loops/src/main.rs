fn main() {
    let mut matrix = [[1, 2, 3], [4, 5, 6], [7, 8, 9]];

    // reading from matrix
    for row in matrix.iter() {
        for number in row.iter() {
            print!("{}\t", number);
        }
        println!("");
    }

    println!("=======================");

    // modifying values from mutable matrix
    // iter_mut() returns mutable references
    for row in matrix.iter_mut() {
        for number in row.iter_mut() {
            // dereference with asterisk to get the value itself
            *number += 20;
            print!("{}\t", number);
        }
        println!("");
    }
}
