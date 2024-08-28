use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    // Collect command line arguments
    let args: Vec<String> = env::args().collect();
    // The first argument is the path that was used to call the program
    println!("Executable path is {}.", args[0]);
    let file_path= &args[1];

    // Open file
    let file = File::open(file_path);
    let file = match file {
        Ok(file) => file,
        Err(error) => {
            match error.kind() {
                std::io::ErrorKind::NotFound => {
                    panic!("File not found: {}", error)
                }
                _ => {
                    panic!("Error opening file: {}", error)
                }
            }
        }
    };

    // Read file
    let reader = BufReader::new(file);
    for line in reader.lines() {
        match line {
            Ok(line) => println!("{}", line),
            Err(error) => {
                panic!("Error reading line: {}", error)
            }
        }
    }
}
