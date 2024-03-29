use std::fs::File;
use std::io::{self, BufRead};
use regex::Regex;

fn main() {
    // Regular expression: word break + letters "in" + word break
    let regex = Regex::new(r"\bin\b").expect("regex");

    // Open an existing file and prepare to read its lines
    let file = File::open("example.txt").expect("file");
    let lines = io::BufReader::new(file).lines();

    // For each line, try the regex; if it matches, then print the line.
    for line in lines {
        if let Ok(string) = line {
            if regex.is_match(&string) {
                println!("match: {}", string);
            }
        }
    }
}
