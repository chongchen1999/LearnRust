use std::cell::RefCell;
use std::mem::size_of;
use std::rc::Rc;
fn main() {
    let b = Box::new(5);
    println!("b = {}", b);
    println!("sizeof Box<> is: {}", std::mem::size_of_val(&b));

    let a = Rc::new(10u128);
    let b = Rc::clone(&a); // Another reference to the same data.
    println!("Reference count: {}", Rc::strong_count(&a));
    println!("sizeof Rc<> is: {}", std::mem::size_of_val(&a));
    println!("address of a: {:p}", &a);
    println!("address of b: {:p}", &b);
    println!("address of a points to: {:p}", &*a);
    println!("address of b points to: {:p}", &*b);

    test_ref_cell();
}

fn test_ref_cell() {
    println!("Size of RefCell<u32>: {}", size_of::<RefCell<u32>>()); // Likely 8 (u32) + 8 (overhead) = 16 bytes
    println!("Size of RefCell<u64>: {}", size_of::<RefCell<u64>>()); // Likely 8 (u64) + 8 (overhead) = 16 bytes
    println!("Size of RefCell<u64>: {}", size_of::<RefCell<u128>>()); // Likely 16 (u128) + 8 (overhead) = 24 bytes
    println!(
        "Size of RefCell<[u8; 32]>: {}",
        size_of::<RefCell<[u8; 32]>>()
    ); // Likely 32 (array) + 8 (overhead) = 40 bytes

    let data = RefCell::new(10i32);
    *data.borrow_mut() += 1; // Mutating through an immutable reference.
    println!("data = {}", data.borrow());

    let mut x = data.borrow_mut();
    *x += 1;
    println!("x = {}", x);
}
