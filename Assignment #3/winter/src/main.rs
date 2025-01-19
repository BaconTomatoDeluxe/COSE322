use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

fn main () {
    let lock_a = Arc::new(Mutex::new(0));
    let lock_b = Arc::new(Mutex::new(0));

    let lock_a_clone = lock_a.clone();
    let lock_b_clone = lock_b.clone();

    let thread_1 = thread::spawn(move || {
        let _a = lock_a.lock().unwrap();
        thread::sleep(Duration::from_secs(1));
        let _b = lock_b_clone.lock().unwrap();
    });

    let thread_2 = thread::spawn(move || {
        let b = lock_b.lock().unwrap();
        thread::sleep(Duration::from_secs(5));
        println!("Thread 2: Unlock b");
        drop(b);
        let _a = lock_a_clone.lock().unwrap();
    });

    thread_1.join().unwrap();
    thread_2.join().unwrap();

    println!("Done!");
}