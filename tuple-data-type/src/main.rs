fn main() {
    // can have n fixed number of mixed type values
    let a_tuple: (&str, u8, char) = ("ok", 0, 'd');
    let first_item = a_tuple.0;
    println!("first_item is {}", first_item);

    // mutate a tuple
    let mut b_tuple = ("ok", 0);
    b_tuple.0 = "ko";
    b_tuple.1 += 1;
    println!("b_tuple.1 is {}", b_tuple.1);

    // destructure a tuple
    let c_tuple = ("en", "US", 1);
    let (language, country, code) = c_tuple;
    println!(
        "language is: {}\ncountry is: {}\ncode is: {}",
        language, country, code
    )
}
