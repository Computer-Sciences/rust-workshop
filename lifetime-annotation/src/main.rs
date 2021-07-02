// defining a generic lifetime 'a

// explicitly defining a generic lifetime for parameter is called lifetime annotation
// must begin with apostrophe ('x)

// convention is to use a single lowercase letter ('a, 'b, 'c)
// but you are free to name it whatever ('a_lifetime, 'toto)

// pay attention to the lifetime annotation
// which follows the borrow operator followed by a space then the type

// in this example, by annotating the return type,
// we tell the compiler that the returned value has the same lifetime as the params
// in case there are different lifetimes,
// the compiler will take the smallest

// by annotating the lifetime of the parameters,
// we give the borrow checker the info to validate the returned reference

fn get_longest_name<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
    let result;
    let name1 = "Deep Space 9";
    let name2 = "Voyager";
    result = get_longest_name(name1, name2);
    println!("result is {}", result);
}
