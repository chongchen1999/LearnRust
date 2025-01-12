pub fn vector_traversal_demo() {
    let v = vec![1, 2, 3, 4, 5];
    for i in 0..v.len() {
        println!("index {}, value {}, address {:p}", i, v[i], &v[i]);
    }
    for i in &v {
        println!("value {}, address {:p}", *i, i);
    }
}