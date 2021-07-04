fn main() {
    let countdown = [5, 4, 3, 2, 1];

    let item = countdown.get(5);
    println!("item is {:?}", item);

    let item = countdown.get(0);
    println!("item is {:?}", item);

    let item = countdown.get(3);

    // using match expression to return value
    let item = match item {
        Some(value) => value + 1,
        None => 0,
    };
    println!("item is {:?}", item);

    let item = countdown.get(20);
    let item = match item {
        Some(value) => value + 1,
        None => 0,
    };
    println!("item is {:?}", item);
}
