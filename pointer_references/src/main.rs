fn main() {
    let mut arr_1: [u8; 2] = [33, 66];

    // ////////////////
    // fixed-length types (stored on the stack) are COPIED
    // ////////////////

    let arr_2 = arr_1;

    println!("arr_1 is {:?}", arr_1);

    arr_1 = [1, 2];

    println!("arr_1 is now {:?}", arr_1);
    println!("arr_2 is {:?}", arr_2);

    // ////////////////
    // mutable-length type values move the ownership to new variable
    // ////////////////

    let vec_1 = vec![3, 4];
    let vec_2 = vec_1;

    // can be no longer use the variable which ownership has been "moved"
    // println!("vec_1 is {:?}", vec_1); // => wll panic

    println!("vec_2 is {:?}", vec_2);

    // to borrow value owned by a variable without moving ownership,
    // use a reference to that value

    let vec_4 = vec![5, 6, 7];
    // borrowing value using a reference (&<NAME>)
    let vec_5 = &vec_4;
    println!("vec_4 is {:?}", vec_4);
    println!("vec_5 is {:?}", vec_5);
}
