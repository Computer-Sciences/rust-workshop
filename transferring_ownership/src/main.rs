fn main() {
    let rocket_fuel = 1;
    process_fuel(rocket_fuel);
    println!("rocket_fuel is {}", rocket_fuel);
}

/*
    because propellant is i32 so leaves on the stack, the value is COPIED jn fn scope
*/
fn process_fuel(mut propellant: i32) {
    // the copy is modified
    propellant += 2;
    println!("Processing propellant {}", propellant);
}
