// used to create custom data type
struct Position {
    longitude: f64,
    latitude: f64,
}

// tuple struct
struct Signal(u8, bool, String);

struct Car {
    model: String,
    year: String,
    used: bool,
}

// associate functions to struct
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

fn main() {
    let mut pos_1 = Position {
        latitude: 27.299112,
        longitude: 95.387110,
    };

    println!("pos_1 is {:.3}, {:.3}", pos_1.latitude, pos_1.longitude);

    pos_1.latitude = 23.1111;
    println!("pos_1 is now {:.3}, {:.3}", pos_1.latitude, pos_1.longitude);

    let mut s1 = Signal(0, true, String::from("ok"));
    println!("s1 is {}, {}, {}", s1.0, s1.1, s1.2);

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
