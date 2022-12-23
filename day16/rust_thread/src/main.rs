fn main() {
    test_thread();
    test_channel();
    test_mutex();
}

use std::thread;
use std::time::Duration;
fn test_thread() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("子线程: hi {}", i);
            thread::sleep(Duration::from_millis(1));
        }
    });
    for i in 1..5 {
        println!("主线程: hi {}", i);
        thread::sleep(Duration::from_millis(1));
    }
    handle.join();
}

use std::sync::mpsc;
fn test_channel() {
    let (tx, rx) = mpsc::channel();
    let tx1 = mpsc::Sender::clone(&tx);
    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];
        for val in vals {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    thread::spawn(move || {
        let vals = vec![
            String::from("more"),
            String::from("messages"),
            String::from("for"),
            String::from("you"),
        ];
        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });
    //接收
    for received in rx {
        println!("Got: {}", received);
    }
}

use std::sync::{Arc, Mutex};
fn test_mutex() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];
    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
        });
        handles.push(handle);
    }
    for h in handles {
        h.join().unwrap();
    }
    println!("Result: {}", *counter.lock().unwrap());
}
