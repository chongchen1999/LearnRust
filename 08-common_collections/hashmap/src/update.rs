use std::collections::HashMap;

pub fn hashmap_update_demo() {
    // Create a new HashMap
    let mut scores = HashMap::new();

    // Inserting key-value pairs
    scores.insert(String::from("Alice"), 10);
    scores.insert(String::from("Bob"), 20);
    println!("After insertion: {:?}", scores);

    // Updating an existing value
    scores.insert(String::from("Alice"), 15); // Overwrites the previous value
    println!("After updating Alice's score: {:?}", scores);

    // Inserting a key-value pair only if the key does not exist
    scores.entry(String::from("Charlie")).or_insert(25);
    println!("After inserting Charlie's score: {:?}", scores);

    // Updating a value based on the old value
    let alice_score = scores.entry(String::from("Alice")).or_insert(0);
    *alice_score += 5;
    println!("After incrementing Alice's score: {:?}", scores);

    // Removing a key-value pair
    scores.remove(&String::from("Bob"));
    println!("After removing Bob's score: {:?}", scores);

    // Iterating over the HashMap
    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }
}