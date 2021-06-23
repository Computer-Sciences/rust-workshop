// when working with generics we can use traits
// in order to stipulate which functionalities the concrete types must implement

// trait bounds require a generic type to implement specific traits

// trait bounds garantee a generic type will have the necessary behaviors

use core::fmt::Debug;
use std::any;

// To be able to print an item it must implement the Display trait
// but because not all type implement it, we will use the Debug trait
// which is more commonly implemented by default

fn print_type<T: Debug>(it: T) {
    // type_name() returns the name of a type as a string slice
    // by passing the type to the turbofish operator (::<MY_TYPE>)

    println!("{:?} is a {}", it, any::type_name::<T>());
}

fn main() {
    print_type(1);
    print_type(1.2);
    print_type([12, 34]);
}
