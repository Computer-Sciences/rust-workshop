// notice here that although the return type has the same lifetime as one param
// we must annotate the other param with another lifetime
// even if never returned
// to avoid any confusion

// remember that here thhe code is simple
// but the input and the returned value could coming from a compiled library
// that the compiler does not have access to the source code

fn get_longest_name<'a, 'b>(x: &'a str, y: &'b str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        x
    }
}

fn main() {
    let result;
    // START 'a
    let name1 = String::from("Deep Space 9");

    {
        // START 'b
        let name2 = String::from("Voyager");
        result = get_longest_name(&name1, &name2);
    } // END 'b

    println!("result is {}", result);
} // END 'a
