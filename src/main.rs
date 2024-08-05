use std::fs;

fn extract_errors(text: &str) -> Vec<&str> {
    let split_text = text.split("\n");

    let mut results = vec![];

    for line in split_text {
        if line.starts_with("ERROR") {
            results.push(line);
        }
    }

    results
}

fn main() {
    match fs::read_to_string("logs.txt") {
        Ok(text) => {
            // This code will create &str
            // let error_logs = extract_errors(&text);
            // but this approach is more explicit
            let error_logs = extract_errors(text.as_str());
            println!("{:#?}", error_logs);
        }
        Err(why_this_failed) => { println!("Failed to read file: {}", why_this_failed) }
    }
}
