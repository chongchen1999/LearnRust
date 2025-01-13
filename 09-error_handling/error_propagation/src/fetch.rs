use std::fs::File;
use std::io::{self, Read};

pub fn read_username_from_file() -> Result<String, io::Error> {
    let username_file_result = File::open("data/hello.txt");

    let mut username_file = match username_file_result {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut username = String::new();

    match username_file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(e) => Err(e),
    }
}

pub fn read_username_from_file_simple() -> Result<String, io::Error> {
    // If this fails, return the error
    let mut username_file = File::open("data/hello.txt")?;
    let mut username = String::new();

    // If this fails, return the error
    username_file.read_to_string(&mut username)?;
    Ok(username) // If everything succeeds, return the username
}