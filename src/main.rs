use std::fs::File;
use std::io::{BufRead, BufReader};
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("I got {:?} arguments: {:?}.", args.len() - 1, &args[1..]);
    println!("My path is {}.", args[0]);
    let mut fn1 = String::from("non_existent_file.txt");
     if args.len()>1{
        fn1 = args[1].clone();
     }
    let file: Result<File, std::io::Error> = File::open(fn1);
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