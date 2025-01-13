// Define a struct that holds a string slice (reference) with a lifetime 'a
struct ImportantExcerpt<'a> {
    part: &'a str,
}

// Implement methods for the struct
impl<'a> ImportantExcerpt<'a> {
    // Method 1: A simple method that returns an i32 (no references involved)
    fn level(&self) -> i32 {
        3
    }

    // Method 2: A method that takes a reference to self and another string slice,
    // and returns a reference to part of the struct's field
    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention please: {}", announcement);
        self.part
    }

    // Method 3: A method that takes two string slices and returns the longer one
    // This method does not depend on the struct's fields, so it uses independent lifetimes
    fn longest<'b>(&self, first: &'b str, second: &'b str) -> &'b str {
        if first.len() > second.len() {
            first
        } else {
            second
        }
    }
}

fn main() {
    // Create a string to be referenced by the struct
    let novel = String::from("Call me Ishmael. Some years ago...");

    // Create an instance of ImportantExcerpt, referencing part of the string
    let excerpt = ImportantExcerpt { part: &novel[..15] };

    // Use the level method (no lifetime annotations needed)
    println!("Level: {}", excerpt.level());

    // Use the announce_and_return_part method
    let announcement = "This is an important announcement!";
    let part = excerpt.announce_and_return_part(announcement);
    println!("Returned part: {}", part);

    // Use the longest method
    let first = "Hello";
    let second = "World!";
    let longest = excerpt.longest(first, second);
    println!("The longest string is: {}", longest);
}
