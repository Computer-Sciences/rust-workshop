fn main() {
    // vectors = mutable size arrays

    let mut letters: Vec<char> = vec!['a', 'b', 'c'];
    println!("letters are {:?}", letters);

    let first_letter = letters[0];
    println!("first_letter is {}", first_letter);

    // add value to vector
    letters.push('d');
    letters.push('e');
    letters.push('f');
    println!("letters are {:?}", letters);

    // remove last value
    letters.pop();
    println!("letters are {:?}", letters);

    let mut numbers: Vec<i32> = vec![11, 22, 44];
    numbers[2] = 33;
    println!("numbers is {}", numbers[2]);

    let words: Vec<&str>;
    words = vec!["ok"; 2];

    println!("words is {:?}", words);

    let mut ints = vec![22, 33, 44, 55, 66, 77];
    let length: usize = ints.len();
    println!("length is {}", length);

    let mem_size_byte = std::mem::size_of_val(&ints);
    println!("mem_size_byte is {}", mem_size_byte);

    // slice from vector
    let mut slice: &[i32] = &ints;
    println!("slice is {:?}", slice);

    slice = &ints[2..5];
    println!("slice is {:?}", slice);

    // iterate over vector
    for it in ints.iter() {
        println!("it is {}", it);
    }

    // mutate vector items while iterating
    for it in ints.iter_mut() {
        // dereference the pointer to get and set value (*it)
        *it *= *it;
    }
    println!("ints is {:?}", ints);
}
