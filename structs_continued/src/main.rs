// need to use debug trait to print the struct instance with debug
// more on traits later
#[derive(Debug, Clone)]
struct Spaceship {
    name: String,
    crew: u8,
    propellant: f64,
}
// methods are defined within a impl block
impl Spaceship {
    fn get_name(&self) -> &str {
        // transform string to string slice with borrow operator
        &self.name
    }
    fn add_fuel(&mut self, gallons: f64) {
        self.propellant += gallons;
    }
    // this is an associated function
    // associated to the struct data type
    // similar to method but no &self reference
    // used for functions related to the struct in general, not a specific instance (like static methods in Object orientation)
    // commonly used to construct instances of struct
    fn new(name: &str) -> Spaceship {
        Spaceship {
            name: String::from(name),
            crew: 11,        // default value for all instances
            propellant: 0.0, // default value for all instances
        }
    }
}
fn main() {
    let mut spaceship1 = Spaceship {
        name: String::from("spaceship1"),
        crew: 123,
        propellant: 1234567.654,
    };
    // accessing field value
    println!("spaceship1 has {} members.", spaceship1.crew);
    spaceship1.crew = 99;
    println!(
        "After attack, spaceship1 has now {} members.",
        spaceship1.crew
    );
    println!("spaceship1 is {:?}", spaceship1);
    // by default struct data is stored on the STACK
    // if struct contains HEAP-stored data (like a string),
    // the pointer is stored on the STACK and the data on the HEAP
    // create struct instance from fields of other instance
    //  (like spread operator in JS but two dots instead of three)
    // it is called struct update syntax
    // it allows to create new instances
    //  by copying the values of fields of an existing instance
    // (except for fields explicitly set in new instance)
    let spaceship2 = Spaceship {
        name: String::from("Galactos"),
        ..spaceship1
    };
    println!("spaceship2 is {:?} members.", spaceship2);
    // when modifying the copied struct instance
    // it will not affect new instances after the copy
    // (unlike JavaScript references that are kept,
    // when spreading objects with arrays...)
    spaceship1.name = String::from("Battlestar");
    spaceship1.crew = 78;
    assert_eq!(spaceship2.crew, 99);
    println!("spaceship1 is {:?}", spaceship1);
    println!("spaceship2 is {:?}", spaceship2);
    // to copy all fields, even the heap based ones,
    // you need to create a copy of the struct instance
    // otherwise when using struct update syntax it moves ownership of heap-based data
    // meaning that you could no longer use the associated fields from the copied instance
    let mut spaceship3 = Spaceship {
        // need to manually implement the clone trait
        // on the struct definition (see above)
        ..spaceship1.clone()
    };
    println!("spaceship3 is {:?}", spaceship3);
    let name = spaceship3.get_name();
    println!("name is {}", name);
    println!("spaceship3 propellant is {}", spaceship3.propellant);
    spaceship3.add_fuel(4567.113);
    println!(
        "After going to space refuel station, spaceship3 propellant is now {}",
        spaceship3.propellant
    );
    // to call an assocaited function, use the path operator (::)
    let spaceship4 = Spaceship::new("Serenity");
    assert_eq!(spaceship4.crew, 11);
}
