fn main() {
    // slice: used to reference a contiguous section (subset) of a collection (aarray, etc.) WITHOUT taking ownership of these elements
    // commonly used: string slice dt type : &str
    // string literals are slices
    // the data is hard-coded in the executable binary and the program  uses a string slice to access it

    // create a string slice from string data

    // the sentence variable has a pointer to the beginning of the string data in memory as well, as info about its length and capacity

    let sentence = String::from("This is a sequence of words.");
    println!("sentence is {}", sentence);

    // the last_word variable has a pointer to the offset/start index of the section of the string data, as well as the length of the slice
    // as usual, the end index of the slice is excluded from the result (common in most programming languages when slicing collections)

    let last_word = &sentence[22..22 + 5]; // [start..end_excluded]
    println!("last_word is \"{}\"", last_word);

    // slice from offset index to end of the collection (here string data)

    let last_part: &str = &sentence[22..];
    println!("last_part is \"{}\"", last_part);

    // slice from beginning of collection up until end index

    let without_last_word = &sentence[..22];
    println!("without_last_word is \"{}\"", without_last_word);

    // the length of a string slice is in number of bytes (usize data type)
    // NOT in number of characters

    let slice_length: usize = last_part.len();
    println!("slice_length is {} bytes", slice_length);

    // when creating a string slice, the range indices must occur at valid UTF-8 character boundaries
    // remember that UTF-8 characters can occupy multiple bytes
    // all this to say that slice range indices must be character boundaries
    // if you index in the middle of a character, the program will panic
    // so be careful when creating string slices from strings with special characters or emojis
    
    // yes, this is some low-level stuff that we usually don't deal with in everyday Node.js
}
