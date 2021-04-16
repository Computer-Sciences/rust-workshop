fn main() {
    let mut count = 0;
    let letters: [char; 5] = ['a', 'b', 'c', 'd', 'e'];

    while count < letters.len() {
        println!("letter[{}] is {}", count, letters[count]);
        count += 1;
    }

    // contrary to loop expressions, the break statement in while loop cannot return a value
}
