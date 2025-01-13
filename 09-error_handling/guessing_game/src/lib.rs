// src/lib.rs
pub struct Guess {
    value: i32,
}

impl Guess {
    /// Creates a new `Guess` instance with validation.
    /// Panics if the value is not between 1 and 100.
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {value}.");
        }
        Guess { value }
    }

    /// Returns the value of the `Guess`.
    pub fn value(&self) -> i32 {
        self.value
    }
}
