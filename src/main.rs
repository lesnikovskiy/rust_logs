use std::fs;

fn string_test(a: String, b: &String, c: &str) {}

fn main() {
    string_test(String::from("red"), &String::from("red"), "red");
    string_test("red".to_string(), &"red".to_string(), "red");

    // Creating slice string
    let color = String::from("red");
    let c = color.as_str();

    match fs::read_to_string("logs.txt") {
        Ok(text_that_was_read) => { println!("{}", text_that_was_read.len()) }
        Err(why_this_failed) => { println!("Failed to read file: {}", why_this_failed) }
    }
}
