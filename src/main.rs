use std::fs;

fn main() {
    match fs::read_to_string("logs.txt") {
        Ok(text) => println!("{}", text),
        Err(error) => println!("Error occurred {}", error),
    }
}
