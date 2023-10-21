use std::io::ErrorKind;
use std::fs::File;

// ErrorKind

fn main() {
    let path = "rand.txt";
    match File::open(path) {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create(path) {
                Ok(file) => file,
                Err(error) => panic!("Cannot create file: {:?}", error)
            },
            _other_error => panic!("Cannot open the file: {:?}", _other_error)
        }
    };
}
