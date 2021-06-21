// by default new structs do not implement any trait
// you as the programmer give it the traits it needs

// derivable traits provide default implementations for common traits

// gives access to basic functionalities
// instead of having to implement custom traits

// when deriving traits,
// the compiler generates default code for the required methods

// derivable traits are (subject to change as the language evolves):

// Eq - comparison
// PartialEq
// Ord
// PartialOrd
// Clone
// Copy
// Hash
// Default - create empty instance of a data type
// Debug - provide formatted debug string

// to derivve traits:  #[derive(trait_a, trait_b, ..)]

// default implementation of PartialEq requires all fields of both structs to be equal to return true
// (check documentation  std::cmp::PartialEq)

// default implementation of PartialOrd compares the values of the fields,
// parses the fields in order of definition and
// once found a field that can be compared,
// will return result of comparison
// and not look at other fields (check documentation  std::cmp::PartialOrd)

#[derive(PartialEq, PartialOrd, Debug)]
struct Car {
    model: String,
    brand: String,
    velocity: u16, // km/h
}

fn main() {
    let gle_class_coupe = Car {
        model: String::from("GLE-Class Coupe"),
        brand: String::from("Mercedes-Benz"),
        velocity: 280,
    };

    let gle_class = Car {
        model: String::from("GLE-Class"),
        brand: String::from("Mercedes-Benz"),
        velocity: 218,
    };

    // allowed because of derived PartialEq trait

    println!("Are the cars equal: {}", gle_class_coupe == gle_class);

    // allowed because of derived PartialOrd trait

    // here the name field is compared to assert which one is greater

    println!(
        "Is the coupe greater then the SUV: {}",
        gle_class_coupe > gle_class
    );

    // allowed because of derived Debug trait

    println!("{:?}", gle_class);
}
