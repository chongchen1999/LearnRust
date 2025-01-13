use std::fs::File;
use std::io::ErrorKind;

fn main() {
    let f = File::open("data/hello.txt");

    let f = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("data/hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            oe => panic!("Problem opening the file: {:?}", oe),
        },
    };

    println!("{:?}", f);
}
