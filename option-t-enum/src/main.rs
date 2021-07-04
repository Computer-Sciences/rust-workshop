// in many languages,
//  errors occur when using a null value in a not-null context
// Rust does not have a traditional null value

// instead Rust uses a generic enum nammed Option
// which can be of two variants
// 1) Some: indicates that has a value
//  None : indicates that no value

// the Option enum is included in the prelude

fn main() {
    // instantiate an option enum
    let someone: Option<i32> = Some(1);
    println!("someone is {:?}", someone);

    let something: Option<&str> = Some("thing");
    println!("something is {:?}", something);

    let nothing: Option<i32> = None;
    println!("nothing is {:?}", nothing);

    let countdown = [5, 4, 3, 2, 1];

    // use get() on slices to get an option enum holding
    // a reference to the value at the specified index
    let item = countdown.get(5);
    println!("item is {:?}", item);

    let item = countdown.get(0);
    println!("item is {:?}", item);

    let item = countdown.get(2);
    let item = item.unwrap_or(&0) + 1;
    println!("item is {:?}", item);

    let item = countdown.get(20);

    // unwrap_or(): if variant is Some returns the stored data
    // if the variant is None, the passed argument is used instead

    let item = item.unwrap_or(&0) + 1;
    println!("item is {:?}", item);
}
