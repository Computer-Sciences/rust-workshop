// stores data in key / value pairs

// use the keys to lookup corresponding values
// key -> value mapping is one way
// Rust uses a hash function to determine how to store data

// keys and values can be different data types
// all keys must have the same type
// all values must have the same type

// each key can only have one value associated with it at a time

// you need to import the type

// updating hashmap entries, approaches:

// #1: overwrite existing key/value pair
// #2: insert a new entry if a key does not exist
// #3: modify a value based on its existing value

use std::collections::HashMap;

fn main() {
    let mut missions = HashMap::new();

    missions.insert("Toto", 23); // HashMap<&str, i32>
    missions.insert("Toto", 33); // #1 update

    missions.insert("Titi", 45);
    missions.insert("Tata", 67);

    missions.entry("Tutu").or_insert(77); // #2 update

    // #3 update

    let tyty = missions.entry("Titi").or_insert(0);
    *tyty += 1;

    println!("missions is {:?}", missions.clone());

    let toto_missions = match missions.get("Toto") {
        Some(value) => value,
        None => &0,
    };

    println!("Toto_missions is {}", toto_missions);
}
