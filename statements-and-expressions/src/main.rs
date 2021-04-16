fn main() {
    println!("Statement performs an action without returning a value");
    println!("statements end with a semicolon: a = 6;");
    println!("an expression evaluates to a resulting value");
    println!("expressions do NOT end with a semicolon: 3 + 4 which evaluates to 7");
    println!("adding a semicolon to an expressions transforms it into an statement");
    println!(
        "expressions are used as parts of statements: let total = r + c;\n\t{}\n\t{}",
        "where \"r + c\" is an expression", "and \"let total = r + c;\" is a statement"
    );
    println!("\nexpression 4 + 5 evaluates to: {}", 4 + 5);
}
