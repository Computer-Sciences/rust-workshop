// lifetime elision rules

// set of rules for the compiler to analyze reference lifetimes
// describes situations that do not require explicit lifetime annotations

// if any ambiguity remains, explicit lifetime annotation will be required

// currently, 3 lifetime elision rules:

// #1
// each input parameter that is a reference is assigned its own lifetime

// fn get_first_word<'a>(s: &'a str) -> &str {

// lifetimes only apply for parameters that are references,
// non-reference params don't need them

// fn get_first_word<'a>(s: &'a str, t: i32) -> &str {

// fn get_first_word<'a, 'b>(s: &'a str, t: &'b str) -> &str {
// fn get_first_word<'a, 'b, 'c>(s: &'a str, t: &'b str, u: &'c str) -> &str {

// #2
// if there's only one input lifetime,
// assign it to all output lifetimes
// (the output lifetime must necessarily match one of the input lifetime)

// qualifies for elision thus lifetimes can be omitted
// fn get_first_word<'a>(s: &'a str) -> &'a str {
// ==>``fn get_first_word(s: &str) -> &str {

// #3
// if there's a &self or &mut self input parameter,
// its lifetime will be assigned to all output lifetimes

// fn do_stuff(&self, input: &str) -> &str {
// applying rule #1 ==> fn do_stuff<'a,'b>(&'a self, input: &'b str) -> &str {
// applying rule #3 ==> fn do_stuff<'a,'b>(&'a self, input: &'b str) -> &'a str {

// these elision rules are for the compiler
// but it's good to know when you can omit lifetimes

fn main() {
    let message = String::from("Hi there");
    let first_word = get_first_word(&message);
    println!("first_word is {}", first_word);
}

fn get_first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (index, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..index]; // found blank space
        }
    }

    &s // no blank space found, input is a single word
}
