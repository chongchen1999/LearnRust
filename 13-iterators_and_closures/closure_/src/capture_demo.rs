pub fn immut_ref() {
    let list = vec![1, 2, 3];
    println!("Before defining closure: {list:?}");

    let only_borrows = || println!("From closure: {list:?}");

    println!("Before calling closure: {list:?}");
    only_borrows();
    println!("After calling closure: {list:?}");
    println!("{}", "-".repeat(50));
}

pub fn mut_ref() {
    let mut list = vec![1, 2, 3];
    println!("Before defining closure: {list:?}");

    let borrows_mutably = |list: &mut Vec<i32>| {
        list.push(7);
        println!("From closure: {list:?}");
    };

    borrows_mutably(&mut list);
    println!("After calling closure: {list:?}");
    println!("{}", "-".repeat(50));
}

pub fn error_demo() {
    let mut list = vec![1, 2, 3];
    println!("Before defining closure: {list:?}");

    let mut borrows_mutably = || list.push(7);

    // Attempt to print the list while the mutable borrow is active
    // println!("Before calling closure: {list:?}"); // This will cause an error

    borrows_mutably();
    println!("After calling closure: {list:?}");
}
