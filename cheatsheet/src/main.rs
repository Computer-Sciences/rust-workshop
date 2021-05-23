mod print;
mod types;
mod vars;

fn main() {
    print::run();
    println!("============================");
    vars::run();
    println!("============================");
    types::run();
    println!("============================");
}
