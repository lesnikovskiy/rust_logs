use std::fs;

fn extract_errors(log: &str) -> Vec<&str> {
    let lines = log.split("\n");
    let mut vec = vec![];

    for line in lines {
        if line.starts_with("ERROR") {
            vec.push(line);
        }
    }

    vec
}

fn main() {
    match fs::read_to_string("logs.txt") {
        Ok(text) => {
            let lines = extract_errors(&text);
            lines.iter().for_each(|line| println!("{}", line))
        }
        Err(why_this_failed) => { println!("Failed to read file: {}", why_this_failed) }
    }
}
