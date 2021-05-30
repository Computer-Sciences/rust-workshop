fn main() {
    // closures are anonymous functions that have access to variables in the enclosing scope

    // long form
    let double = |n1: u8| -> u8 { n1 * 2 };

    // short form
    let triple = |n1| n1 * 3;

    const DAYS_IN_YEAR: u16 = 365;
    // referencing variable from enclosing scope
    let quadruple_than_add_number_days_in_year = |n1: i32| n1 * 4 + DAYS_IN_YEAR as i32;

    const FACTOR: i32 = 22;
    let multiple_by_22 = |x| FACTOR * x;

    println!("{}", double(11));
    println!("{}", triple(99));
    println!("{}", quadruple_than_add_number_days_in_year(44));
    println!("{}", multiple_by_22(5));
}
