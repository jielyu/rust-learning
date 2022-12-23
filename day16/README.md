# 无畏并发

## 使用线程同时运行代码

* 使用 `thread::spawn` 创建新线程

* 使用 `join` 等待子线程结束

```rust
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
```

* 启动子线程时的闭包如果需要捕获环境的数据，需要使用 `move` 方式，因为不确定借用是否会在使用之前失效

## 使用消息传递在县城建转移数据

* 使用通道传送数据时，所有权会转移
* `recv` 会等待接收发送线程中传送过来数据
* 通过克隆发送者可以创建多个生产者

```rust
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
```

## 共享状态的并发

* 使用 `Mutex<T>` 保证数据操作的互斥性
* 使用 `Arc<T>` 实现多线程中的多重所有权

```rust
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
```

Output:

```
Result: 10
```

* `Mutex<T>` 本身不可变，但却可以获得可变借用，与`RefCell<T>` 类似

## Sync和Send trait

* 只有实现了 `Send` trait 的类型才可以安全地在线程间转移所有权
* 只有实现了 `Sync` trait 的类型才可以安全地被多个线程引用
* 手动实现 `Send` 和 `Sync` trait是不安全的，当某个类型完全由实现了这两个trait的类型组成时，他就会自动实现这两个trait