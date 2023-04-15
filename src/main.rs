use std::{env, fs};

mod tests;

fn main() {
    let args = env::args();
    if args.len() < 2 {
        println!("Usage: semify <filename>");
        return;
    }

    let filename = args.skip(1).next().unwrap();
    let contents = fs::read_to_string(filename).expect("Could not read file");

    for line in contents.lines() {
        println!("{}", add_semicolon(line));
    }
}

/// Adds a semicolon to the end of the line if it should.
pub fn add_semicolon(line: &str) -> String {
    if should_add_semicolon(line) {
        format!("{};", line)
    } else {
        line.to_string()
    }
}

/// Determines whether a semicolon should be added to the end of the line.
fn should_add_semicolon(line: &str) -> bool {
    !is_str_whitespace(line) && !line.ends_with(":") && !line.ends_with(";")
}

/// Determines whether a string is empty or contains only whitespace.
fn is_str_whitespace(s: &str) -> bool {
    s.chars().all(char::is_whitespace)
}
