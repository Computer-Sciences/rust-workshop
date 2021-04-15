fn main() {
    let d2: [[i32; 3]; 3] = [[9, 8, 7], [6, 5, 4], [3, 2, 1]];
    let value = d2[1][0];
    println!("value is {}", value);

    // mutating a tuple
    let d3: [[[&str; 100]; 20]; 5];
    d3 = [[["ok"; 100]; 20]; 5];
    println!("value d3[3][11][35] is {}", d3[3][11][35])
}
