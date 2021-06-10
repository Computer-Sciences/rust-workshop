fn main() {
    let rocket_fuel = String::from("MU-RF");
    // notice the borrow operator to create a reference
    // the function expects a reference, not a value
    let length = process_fuel(&rocket_fuel);
    println!("rocket_fuel is {}, lenght: {}", rocket_fuel, length);
}

// notice the borrow operator in parameter type
// we expect propellant to be a reference to a string,
// not a string value (/pointer to it...)
fn process_fuel(propellant: &String) -> usize {
    // propellant is a reference to the variable that points to the string data, to borrow the data
    // again, NO SHALLOW COPY here because propellant borrows the pointer, it does contain the pointer itself (let's say that it's a pointer to another pointer...)
    println!("Processing propellant {}", propellant);
    let length = propellant.len();
    length
}
