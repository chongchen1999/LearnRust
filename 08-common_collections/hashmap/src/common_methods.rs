use std::collections::HashMap;

pub fn create_demo() {
    let mut map = HashMap::new();
    map.insert(1, "a");
    map.insert(2, "b");
    map.insert(3, "c");
    for (key, value) in &map {
        println!("{}: {}", key, value);
    }
}

pub fn create_with_vector_demo() {
    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];

    let scores: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();

    println!("{:?}", scores);
}

pub fn hashmap_ownership() {
    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(&field_name, &field_value);
    // field_name and field_value are moved here

    println!("{}, {}", field_name, field_value);
}

pub fn hashmap_traversal_demo() {
    let mut scores = HashMap::new();
    let (k1, v1) = (String::from("Blue"), 10);
    let (k2, v2) = (String::from("Yellow"), 50);
    println!("address of (k1, v1) is: {:p}, {:p}", &k1, &v1);
    println!("address of (k2, v2) is: {:p}, {:p}", &k2, &v2);

    scores.insert(k1, v1);
    scores.insert(k2, v2);

    for (key, value) in &scores {
        println!("{}: {}", key, value);
        println!("address of (key, value) is: {:p}, {:p}", key, value);
    }
}