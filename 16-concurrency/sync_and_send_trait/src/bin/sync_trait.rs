use std::sync::{Arc, Mutex};
use std::thread;

struct ThreadSafe {
    data: Arc<Mutex<i32>>,
}

fn main() {
    let safe_instance = ThreadSafe {
        data: Arc::new(Mutex::new(42)),
    };

    let shared = safe_instance.data.clone();

    let threads: Vec<_> = (0..10)
        .map(|_| {
            let cloned = shared.clone();
            thread::spawn(move || {
                let mut lock = cloned.lock().unwrap();
                *lock += 1; // Safely modify the value
            })
        })
        .collect();

    for t in threads {
        t.join().unwrap();
    }

    println!("Final value: {}", *shared.lock().unwrap());
}
