// An unsafe trait that requires implementors to guarantee that the data is always valid.
unsafe trait TrustedData {
    // This method returns a raw pointer to some data.
    // The implementor must ensure that the data is always valid and safe to dereference.
    fn get_data(&self) -> *const i32;
}

// A struct that implements the `TrustedData` trait.
struct MyData {
    value: i32,
}

// Implementing the `unsafe trait` requires the `unsafe` keyword.
unsafe impl TrustedData for MyData {
    fn get_data(&self) -> *const i32 {
        &self.value as *const i32
    }
}

fn main() {
    let data = MyData { value: 42 };

    // Using the unsafe trait's method requires careful handling.
    unsafe {
        let ptr = data.get_data();
        println!("Data: {}", *ptr);
    }
}
