fn main() {
    let message = ['m', 'e', 's', 's', 'a', 'g', 'e'];

    /* Iterator
    - implements logic to iterate over each item in a collection
    - next() method returns the next item in a sequence
      */
    for item in message.iter() {
        println!("current item is {}", item);
    }

    println!("");

    // to also get the indexes when iterating
    // enumerate() return a tuple with index/item pairitem in the array not the actual item
    // to get the item use &item
    // the iterator gives a REFERENCE to an
    for (index, &item) in message.iter().enumerate() {
        println!("item {} is {}", index, item);
        if item == 'e' {
            break;
        }
    }

    println!("");

    // iterating over a range of numbers
    // excludes the end value of the range
    for number in 0..5 {
        println!("number is {}", number);
    }
}
