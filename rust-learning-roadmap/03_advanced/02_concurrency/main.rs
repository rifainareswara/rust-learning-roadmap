// Concurrency in Rust
// ====================
// Threads, message passing, and shared state

use std::sync::{mpsc, Arc, Mutex};
use std::thread;
use std::time::Duration;

fn main() {
    // ================================
    // BASIC THREADS
    // ================================
    
    println!("=== Basic Threads ===\n");
    
    // Spawn a new thread
    let handle = thread::spawn(|| {
        for i in 1..5 {
            println!("Thread: counting {}", i);
            thread::sleep(Duration::from_millis(100));
        }
    });
    
    for i in 1..3 {
        println!("Main: counting {}", i);
        thread::sleep(Duration::from_millis(100));
    }
    
    // Wait for thread to finish
    handle.join().unwrap();
    println!("Thread finished!");
    
    // ================================
    // MOVE CLOSURES
    // ================================
    
    println!("\n=== Move Closures ===\n");
    
    let v = vec![1, 2, 3];
    
    // Use `move` to take ownership of captured values
    let handle = thread::spawn(move || {
        println!("Vector from thread: {:?}", v);
    });
    
    handle.join().unwrap();
    // v is no longer valid here - it was moved
    
    // ================================
    // MESSAGE PASSING (Channels)
    // ================================
    
    println!("\n=== Message Passing (mpsc) ===\n");
    
    // mpsc = multiple producer, single consumer
    let (tx, rx) = mpsc::channel();
    
    thread::spawn(move || {
        let messages = vec![
            String::from("Hello"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];
        
        for msg in messages {
            tx.send(msg).unwrap();
            thread::sleep(Duration::from_millis(200));
        }
    });
    
    // Receive messages (blocking)
    for received in rx {
        println!("Received: {}", received);
    }
    
    // ================================
    // MULTIPLE PRODUCERS
    // ================================
    
    println!("\n=== Multiple Producers ===\n");
    
    let (tx, rx) = mpsc::channel();
    let tx2 = tx.clone();
    
    thread::spawn(move || {
        let messages = vec!["A1", "A2", "A3"];
        for msg in messages {
            tx.send(format!("Producer A: {}", msg)).unwrap();
            thread::sleep(Duration::from_millis(100));
        }
    });
    
    thread::spawn(move || {
        let messages = vec!["B1", "B2", "B3"];
        for msg in messages {
            tx2.send(format!("Producer B: {}", msg)).unwrap();
            thread::sleep(Duration::from_millis(100));
        }
    });
    
    for received in rx {
        println!("{}", received);
    }
    
    // ================================
    // SHARED STATE (Mutex)
    // ================================
    
    println!("\n=== Shared State (Mutex) ===\n");
    
    let m = Mutex::new(5);
    
    {
        let mut num = m.lock().unwrap();
        *num = 6;
    }  // Lock is released here
    
    println!("m = {:?}", m);
    
    // ================================
    // MUTEX WITH MULTIPLE THREADS (Arc)
    // ================================
    
    println!("\n=== Arc<Mutex<T>> ===\n");
    
    // Arc = Atomic Reference Counting (thread-safe Rc)
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];
    
    for i in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
            println!("Thread {} incremented counter", i);
        });
        handles.push(handle);
    }
    
    for handle in handles {
        handle.join().unwrap();
    }
    
    println!("Final counter: {}", *counter.lock().unwrap());
    
    // ================================
    // PARALLEL COMPUTATION
    // ================================
    
    println!("\n=== Parallel Computation ===\n");
    
    let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let numbers = Arc::new(numbers);
    let sum = Arc::new(Mutex::new(0));
    
    let mut handles = vec![];
    
    // Split work across 2 threads
    for i in 0..2 {
        let numbers = Arc::clone(&numbers);
        let sum = Arc::clone(&sum);
        
        let handle = thread::spawn(move || {
            let start = i * 5;
            let end = start + 5;
            let partial: i32 = numbers[start..end].iter().sum();
            
            let mut total = sum.lock().unwrap();
            *total += partial;
            println!("Thread {}: partial sum = {}", i, partial);
        });
        
        handles.push(handle);
    }
    
    for handle in handles {
        handle.join().unwrap();
    }
    
    println!("Total sum: {}", *sum.lock().unwrap());
}

/*
=== CONCURRENCY SUMMARY ===

1. Threads:
   - thread::spawn() creates a new thread
   - handle.join() waits for thread to complete
   - `move` closure takes ownership

2. Message Passing:
   - mpsc::channel() creates sender/receiver
   - tx.send() sends data
   - rx.recv() or for loop receives

3. Shared State:
   - Mutex<T> provides mutual exclusion
   - lock() returns MutexGuard (auto-unlocks on drop)
   - Arc<T> for thread-safe reference counting

4. Rules:
   - Cannot share Rc<T> between threads (use Arc<T>)
   - Cannot share RefCell<T> between threads (use Mutex<T>)
*/

// EXERCISES:
// 1. Buat 3 threads yang masing-masing print nama mereka
// 2. Gunakan channel untuk mengirim hasil kalkulasi dari worker thread
// 3. Buat concurrent counter yang di-increment oleh 5 threads
