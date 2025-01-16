use std::fmt;

// Define the ConsList enum
enum ConsList<T> {
    Cons(T, Box<ConsList<T>>), // A pair of a value and a Box pointing to the rest of the list
    Nil,                       // End of the list
}

// Implement methods for ConsList
impl<T> ConsList<T> {
    // Create a new empty list
    fn new() -> Self {
        ConsList::Nil
    }

    // Prepend a value to the list
    fn prepend(self, value: T) -> Self {
        ConsList::Cons(value, Box::new(self))
    }

    // Get the length of the list
    fn len(&self) -> usize {
        match self {
            ConsList::Cons(_, tail) => 1 + tail.len(),
            ConsList::Nil => 0,
        }
    }

    // Check if the list is empty
    fn is_empty(&self) -> bool {
        matches!(self, ConsList::Nil)
    }
}

// Implement Display for pretty printing
impl<T: fmt::Display> fmt::Display for ConsList<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ConsList::Cons(head, tail) => write!(f, "{} -> {}", head, tail),
            ConsList::Nil => write!(f, "Nil"),
        }
    }
}

fn main() {
    // Create an empty list
    let mut list = ConsList::new();

    // Prepend some values
    list = list.prepend(3);
    list = list.prepend(2);
    list = list.prepend(1);

    // Print the list
    println!("List: {}", list); // Output: 1 -> 2 -> 3 -> Nil

    // Get the length of the list
    println!("Length: {}", list.len()); // Output: 3

    // Check if the list is empty
    println!("Is empty: {}", list.is_empty()); // Output: false
}
