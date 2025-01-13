fn main() {
    let _e = error_propagation::fetch::read_username_from_file();
    let _e = error_propagation::fetch::read_username_from_file_simple();
    let e = error_propagation::chaining::read_username_from_file();
    match e {
        Ok(username) => println!("Username: {}", username),
        Err(e) => eprintln!("Error reading username from file: {}", e),
    }

    let o = error_propagation::option::last_char_of_first_line("it is me\really?");
    match o {
        Some(c) => println!("Last char: {}", c),
        None => eprintln!("Error reading last char"),
    }
}
