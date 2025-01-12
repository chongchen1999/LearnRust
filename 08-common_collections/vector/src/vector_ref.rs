pub fn vector_ref_demo() {
    let v = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let third: &i32 = &v[2];
    println!("The third element address is {:p}", third);

    match v.get(100) {
        Some(x) => println!("The third element is {}", x),
        None => println!("There is no third element"),
    }

    println!("The size of vector is: {}", std::mem::size_of::<Vec<f64>>());
    println!("The size of vector is: {}", std::mem::size_of_val(&v));
}
