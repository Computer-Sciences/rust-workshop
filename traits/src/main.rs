// trait;
// abstract way to define capabilities for functionality of specific data types

// collection of methods
// representing a set of behaviors necessary to accomplish some task

// data types can implement a trait:
// the type implements those specific methods so they'll be available for use with it

// generics use traits to specify the capabilities of unknown data types
// the trait act as a bound for which concrete types will be accepted
// as long as these concrete types implement the set of methods

// similar to interfaces in TypeScript and other languages supporting classical OOP

// Rust comes with default common traits (see the std::prelude)

// you can define your custom traits

struct Car {
    model: String,
    brand: String,
    velocity: u16, // km/h
}

struct Plane {
    name: String,
    nick_name: String,
    velocity: u16, // km/h
    crew: u8,
    passengers: u16,
    ceiling: u32,
}

// custom trait

trait Serialization {
    // method signatures
    fn describe(&self) -> String;
}

impl Serialization for Car {
    fn describe(&self) -> String {
        format!(
            "{} from {} going at {} km/h.",
            self.model, self.brand, self.velocity
        )
    }
}

impl Serialization for Plane {
    fn describe(&self) -> String {
        format!(
            "{} ({}) going at {} km/h with a crew of {} and up to {} passengers at {} m above.",
            self.name, self.nick_name, self.velocity, self.crew, self.passengers, self.ceiling
        )
    }
}

fn main() {
    let gle_class_coupe = Car {
        model: String::from("GLE-Class Coupe"),
        brand: String::from("Mercedes-Benz"),
        velocity: 280,
    };

    let g6 = Plane {
        name: String::from("Gulfstream G650 "),
        nick_name: String::from("G6"),
        velocity: 956,
        crew: 4,
        passengers: 19,
        ceiling: 15545,
    };

    println!("{}", gle_class_coupe.describe());
    println!("{}", g6.describe());
}
