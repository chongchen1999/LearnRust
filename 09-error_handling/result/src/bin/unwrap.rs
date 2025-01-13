use std::fs::File;
use std::io::ErrorKind;

fn main() {
    let f = File::open("data/hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("data/hello.txt").unwrap_or_else(|e| {
                panic!("Problem creating the file: {:?}", e);
            })
        } else {
            panic!("Problem opening the file: {:?}", error);
        }
    });

    println!("{:?}", f);
}
