// Mutex that protects the data vector, and then we spawn three threads
//that each acquire a lock on the mutex and modify an element of the vector.
// https://doc.rust-lang.org/std/sync/struct.Mutex.html

use std::sync::{Arc, Condvar, Mutex, RwLock};
use std::thread;

fn using_mutex() {
    println!("Mutex Example");
    let data = Arc::new(Mutex::new(vec![1, 2, 3]));

    let handles: Vec<_> = (0..3)
        .map(|i| {
            let data = Arc::clone(&data);
            thread::spawn(move || {
                let mut data = data.lock().unwrap();
                data[i] += 1;
                println!("{:?}", data);
            })
        })
        .collect();

    for handle in handles {
        handle.join().unwrap();
    }

    let data = data.lock().unwrap();
    println!("Final data: {:?}", *data);
}

fn using_rwlock() {
    println!("RwLock Example");
    // Create an RwLock protecting a vector
    let data = Arc::new(RwLock::new(vec![1, 2, 3]));

    // Spawn three threads that each acquire a write lock on the RwLock and modify an element of the vector
    let handles: Vec<_> = (0..3)
        .map(|i| {
            let data = Arc::clone(&data);
            thread::spawn(move || {
                let mut data = data.write().unwrap();
                data[i] += 1;
                println!("Thread {}: {:?}", i, *data);
            })
        })
        .collect();

    // Wait for all threads to finish
    for handle in handles {
        handle.join().unwrap();
    }

    // Acquire a read lock to print the final state of the vector
    let data = data.read().unwrap();

    println!("Final data: {:?}", *data);
}

fn using_condvar() {
    println!("Condvar Example");
    let data = Arc::new((Mutex::new(vec![1, 2, 3]), Condvar::new()));
    let mut handles = vec![];

    for i in 0..3 {
        let data = Arc::clone(&data);
        let handle = thread::spawn(move || {
            let (lock, cvar) = &*data;
            let mut data = lock.lock().unwrap();
            data[i] += 1;
            println!("Thread {}: {:?}", i, *data);
            cvar.notify_one();
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    let (lock, cvar) = &*data;
    let mut data = lock.lock().unwrap();
    while data.iter().any(|&x| x == 1) {
        data = cvar.wait(data).unwrap();
    }

    println!("Final data: {:?}", *data);
}

fn main() {
    using_mutex();
    using_rwlock();
    using_condvar();
}

/*

// This code will not compile because the borrow checker will prevent it.
use std::thread;

fn main() {
    let mut data = vec![1, 2, 3];

    for i in 0..3 {
        // Try to capture a mutable reference in multiple threads
        // This will fail to compile!
        thread::spawn(move || {
            data[i] += 1;
        });
    }

    // No data race can occur, this will not compile.
}
*/
