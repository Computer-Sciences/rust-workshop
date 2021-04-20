fn main() {
    println!("Two types of string representation:");
    println!("- string literal: hard coded into the executable.\n\tthese are immutable and must be known before compilation");
    println!("- String type: allocated data on the heap, \n\tmutable and dynamically generated at runtime\n");

    // string literal stored on heap
    // String::from() creates a String type from a string literal
    // the sequence [m,a,r,s] will get stored on the heap
    // to access the string stored on heap, program holds a pointer to it on the stack (message variable)
    // that pointer on the stack includes first char memory address, length of string and the capacity so you how much mem allocated for it on the heap

    let mut message = String::from("Jupiter");
    println!("message is {}", message);

    // append string to original
    // if more memory need than capacity, pointer address updated as well as length and capacity to reflect new location in memory

    message.push_str(" is smoke and mirrors");
    println!("message is {}", message);
}
