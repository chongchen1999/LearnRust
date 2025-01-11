fn main() {
    // 1. References (Safe Pointers)
    let x = 42;
    let ref_x = &x; // immutable reference
    println!("address of x: {}", ref_x);

    let mut y = 42;
    let mut_ref_y = &mut y; // mutable reference
    println!("address of y: {}", mut_ref_y);

    // 2. Raw Pointers (Unsafe Pointers)
    let z = 42;
    let raw_const_ptr = &z as *const i32; // constant raw pointer

    let mut w = 42;
    let raw_mut_ptr = &mut w as *mut i32; // mutable raw pointer

    // Using raw pointers requires unsafe block
    unsafe {
        println!("raw const ptr value: {}", *raw_const_ptr);
        *raw_mut_ptr = 43;
        println!("raw mut ptr value: {}", *raw_mut_ptr);
    }

    // 3. Box (Heap Allocation)
    let box_val = Box::new(42); // allocates on heap
    println!("box value: {}", *box_val);
}
