#[no_mangle]
pub extern "C" fn add_numbers(a: i32, b: i32) -> i32 {
    a + b
}

#[no_mangle]
pub extern "C" fn greet() -> *const u8 {
    "Hello from Rust\0".as_ptr()
}
