// tuple struct
// used to store a collection of mixed data without named fields
// used to be distinguished as a specific type
// (not just a regular tuple)

struct Signal(u8, bool, String);

// regular struct
// struct names are capitalized
// like classes in JavaScript and OOP generally

struct Car {
    // fields of the struct
    model: String,
    year: String,
    used: bool,
}

// method: functions/subroutines associated to a struct
// methods are defined within the context of a struct
// the first parameter of a method is the reference to a struct instance

impl Car {
    // construct car

    fn new(m: &str, y: &str) -> Car {
        Car {
            model: m.to_string(),
            year: y.to_string(),
            used: false,
        }
    }

    // self is equivalent to "this" is JavaScript

    fn serialize(&self) -> String {
        format!(
            "model: {} - year: {} - used: {}",
            self.model, self.year, self.used
        )
    }

    // mutate state

    fn marked_used(&mut self) {
        self.used = true;
    }
}

struct Position {
    latitude: f64,
    longitude: f64,
}

fn print_signal(s: &Signal) {
    // fields of a tuple struct are accessed like regular tuples values
    // using their index
    // remember tuple structs do not have named fields
    println!("s1 is {}, {}, {}", s.0, s.1, s.2);
}

fn main() {
    let mut pos_1 = Position {
        latitude: 27.299112,
        longitude: 95.387110,
    };

    println!("pos_1 is {:.3}, {:.3}", pos_1.latitude, pos_1.longitude);

    pos_1.latitude = 23.1111;

    println!("pos_1 is now {:.3}, {:.3}", pos_1.latitude, pos_1.longitude);

    let mut s1 = Signal(0, true, String::from("ok"));

    print_signal(&s1);

    s1.0 = 23;

    s1.1 = false;

    s1.2 = String::from("NETERR");

    println!("s1 is now {}, {}, {}", s1.0, s1.1, s1.2);

    let car_1 = Car::new("QBC", "2133");
    println!("car_1 is a {} of {}", car_1.model, car_1.year);

    let is_used = if car_1.used == true {
        "used"
    } else {
        "brand new"
    };

    println!("car_1 is {}", is_used);

    println!("car_1 is {}", car_1.serialize());

    let mut car_2 = Car::new("ZZ7", "2042");

    println!("car_2 is a {}", car_2.serialize());

    car_2.marked_used();

    println!("car_2 is now {}", car_2.serialize());
}
