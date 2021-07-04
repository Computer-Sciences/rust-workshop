fn main() {
    // create an empty vector

    let mut presidents: Vec<String> = Vec::new();

    presidents.push(String::from("Chirac"));
    presidents.push(String::from("Sarkozy"));
    presidents.push(String::from("Hollande"));
    presidents.push(String::from("Macron"));

    println!("presidents are {:?}", presidents);

    let last = match presidents.pop() {
        Some(president) => president,
        None => "not found".to_string(),
    };

    println!("last is {}", last);

    let fourth = match presidents.get(3) {
        Some(president) => president,
        None => "not found",
    };

    println!("fourth president is {}", fourth);

    let countdown = vec![4, 3, 2, 1];

    println!("countdown is {:?}", countdown);
}
