use std::env;
use std::fs::{self, File};
use std::io::{self, BufRead, BufReader, Write};
use std::path::Path;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = args.get(1).expect("Please provide a file path");

    match process_file(file_path) {
        Ok((lines, words, chars)) => {
            println!("File processed successfully:");
            println!("Lines: {}, Words: {}, Characters: {}", lines, words, chars);
        }
        Err(e) => eprintln!("Error: {}", e),
    }
}

fn process_file(file_path: &str) -> Result<(usize, usize, usize), io::Error> {
    if Path::new(file_path).exists() {
        let file = File::open(file_path)?;
        let reader = BufReader::new(file);

        let mut line_count = 0;
        let mut word_count = 0;
        let mut char_count = 0;

        for line in reader.lines() {
            let line = line?;
            line_count += 1;
            word_count += line.split_whitespace().count();
            char_count += line.chars().count();
        }

        Ok((line_count, word_count, char_count))
    } else {
        let mut file = File::create(file_path)?;
        file.write_all(b"Welcome to Rust! This is a new file.")?;
        println!("File created with default content.");

        // return (0, 0, 0) for lines, words, and chars for new file
        Ok((0, 0, 0))
    }
}
