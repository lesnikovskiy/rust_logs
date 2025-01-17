use std::fs;
use std::io::Error;

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

fn main() -> Result<(), Error> {
    let text = fs::read_to_string("logs.txt")?;
    let error_logs = extract_errors(&text);
    fs::write("errors.txt", error_logs.join("\n"))?;

    // match fs::read_to_string("logs.txt") {
    //     Ok(text) => {
    //         let error_logs = extract_errors(text.as_str());

    //         match fs::write("errors.txt", error_logs.join("\n")) {
    //             Ok(..) => println!("Wrote errors.txt"),
    //             Err(reason_write_failed) =>
    //                 println!("Writing of errors.txt failed: {}", reason_write_failed),
    //         }
    //     }
    //     Err(why_this_failed) => {
    //         println!("Failed to read file: {}", why_this_failed);
    //     }
    // }

    Ok(())
}
