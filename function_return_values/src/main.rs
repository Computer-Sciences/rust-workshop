fn main() {
    let result = square(3);
    println!("result is {}", result);
    let result_tuple = triple(33);
    let (input, result1) = result_tuple;
    println!("result_tuple is {:?}", result_tuple); // {:?} ==> debug formatting
    println!("input {} evaluates to {}", input, result1);
    let nothing: () = does_not_return();
    println!("nothing (union data type) is {:?}", nothing)
}

fn square(number: i32) -> i32 {
    println!("processing square({})", number);
    // expression returnng a value
    number * number
    // " return  number * number;" is also vald ayntax
}

// multiple returns with tuples
fn triple(number: i32) -> (i32, i32) {
    println!("tripling the number: {}", number);
    let input = number;
    let result = number * 3;
    (input, result)
}

// union data type
// used when no meaningful values returned by a fn
// represented by empty ()
// it is optional
fn does_not_return() -> () {
    println!("ain't returning nuthing!")
}
