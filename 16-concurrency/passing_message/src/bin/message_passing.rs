use std::sync::mpsc;
use std::thread;

fn main() {
    // Create a channel
    let (tx, rx) = mpsc::channel();

    // Spawn a new thread and move the transmitter into it
    thread::spawn(move || {
        let val = String::from("hi");
        tx.send(val).unwrap(); // Send the message "hi"
    });

    // Main thread receives the message
    let received = rx.recv().unwrap();
    println!("Got: {}", received); // Print the received message
}
