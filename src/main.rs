use std::io::{Write, BufReader, BufRead};
use std::fs::File;

// File handling and Result

// Result has 2 variants Ok and Err
// enum Result<T, E> {
//    Ok(T),
//    Err(E),
// }
// Where T represents the datatype of the return value, and E the type of error

fn main() {
    let path = "lines.txt";
    let mut output = match File::create(path) {
        Ok(file) => file,
        Err(error) => {
            panic!("Problem creating file: {:?}",  error);
        }
    };

    write!(output, "Just Some\nRandom words").expect("Failed to write to file"); // If any error happens, then throw this message

    let input = File::open(path).unwrap();
    let buffered = BufReader::new(input);

    for line in buffered.lines() {
        println!("{}", line.unwrap());
    }
}
