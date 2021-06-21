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

trait Serialization {
    // default trait implementation
    // to avoid having to implement it for each type that uses the trait

    fn describe(&self) -> String {
        String::from("some kind of vehicule.")
    }
}

impl Serialization for Car {
    // specific trait implementation for Car structure
    // overriding the default implementation

    fn describe(&self) -> String {
        format!(
            "{} from {} going at {} km/h.",
            self.model, self.brand, self.velocity
        )
    }
}

// will use the default trait implementation because no specific one
impl Serialization for Plane {}

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
