use std::mem;

// Box<T> used to store data on the heap instead of the stack

// it consists of a pointer on the stack
// which points to a memory chunk on the heap
// that has been allocated large enough to hold <T> data

// Boxes are considered smart pointers
// provide more functionality than references:

// Box<T> has ownership of the data it points to

// Box<T> deallocates the heap memory when it goes out of scope

// Box data type is used to store a type whose size cannot be known at compile time
// but a size is still required
// ex: recursive types
// ex of recursive type: a struct which includes another struct of the same type as one of its fields (think about Iterators - like in JavaScript)
// you don't know the depth of layers at compile time and therefore not know the size of the overall structure

// another use case for boxes is to transfer ownership of data rather copying it on the stack
// to avoid copying large amounts of stack data
// potential performance improvement by moving data to heap where can be more easily transferred (because of pointers)

#[derive(Debug)]
struct Team {
    name: String,
    size: u8,
    capacity: f64,
    domain: String,
}

fn main() {
    // remember the struct lives on the stack
    // except for mutable-length fields (ex: String) that leave on the heap

    let lions = Team {
        name: String::from("The lions"),
        size: 7,
        capacity: 11.5,
        domain: String::from("Authentication"),
    };

    println!("lions is {:?}", lions);

    // mem::size_of_val() gives the size in byte of what the given reference points to

    println!("lions size is on stack {} bytes", mem::size_of_val(&lions));

    // this is not a copy operation, ownership is moved
    // the lions variable looses ownership of the struct
    // the lions variable can no longer be used
    // the struct instance is transferred to to heap location allocated by the box
    // the boxed_lions variable becomes the new owner of the struct
    // through the box smart pointer that lives on the stack

    let boxed_lions: Box<Team> = Box::new(lions);

    println!("boxed_lions is {:?}", boxed_lions);

    // printing size in bytes of the box pointer

    println!(
        "boxed_lions size is on stack {} bytes",
        mem::size_of_val(&boxed_lions)
    );

    // printing size in byte of the struct instance
    // need to pass a reference to the dereferenced box pointer (WTF...) to mem::size_of_val()
    // using the dereference operator "*" before the box pointer
    // referencing a pointer will represent the pointed-to location / data:
    // &*boxed_lions is a reference to the data on the heap rather than the pointer on the stack

    println!(
        "boxed_lions team size is on heap {} bytes",
        mem::size_of_val(&*boxed_lions)
    );

    // moving back the struct instance on the stack
    // by dereferencing the box pointer

    let unboxed_lions: Team = *boxed_lions;

    println!("unboxed_lions is {:?}", unboxed_lions);

    println!(
        "unboxed_lions team size is on stack {} bytes",
        mem::size_of_val(&unboxed_lions)
    );
}
