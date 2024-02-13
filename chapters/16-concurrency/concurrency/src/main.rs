use std::sync::mpsc;
use std::sync::{Arc, Mutex};
use std::{thread, time};

fn main() {
    thread_example();
    println!("------------------------------------");
    println!("------------------------------------");
    thread_example_join();
    println!("------------------------------------");
    println!("------------------------------------");
    move_closure();
    println!("------------------------------------");
    println!("------------------------------------");
    channels_example();
    println!("------------------------------------");
    println!("------------------------------------");
    shared_state();
}

fn thread_example() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(time::Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(time::Duration::from_millis(1));
    }

    handle.join().unwrap();
    // When the main thread ends, the spawned thread ends too.
    // The spawned thread will not finish its execution.
    // To fix this, we can use the join method to wait for the spawned thread to finish (next function)
}

fn thread_example_join() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(time::Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(time::Duration::from_millis(1));
    }

    handle.join().unwrap();
    // This will wait for the spawned thread to finish before the main thread ends.
}

fn move_closure() {
    let v = vec![1, 2, 3];

    let handle = thread::spawn(move || {
        println!("Here's a vector: {:?}", v);
    });

    handle.join().unwrap();
    // The move keyword is used to force the closure to take ownership of the values it uses in the environment.
    // This is necessary because the spawned thread may outlive the values it references.
}

fn channels_example() {
    // tx and rx are the sending and receiving ends of the channel, respectively.
    // mpsc stands for multiple producer, single consumer.
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let val = String::from("Hello World!");
        tx.send(val).unwrap();
    });

    // The recv method blocks the main thread’s execution and waits until a value is sent down the channel.
    let received = rx.recv().unwrap();
    println!("Got: {}", received);

    let (tx, rx) = mpsc::channel();

    // We can send multiple messages and receive them in a loop
    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(time::Duration::from_secs(1));
        }
    });

    // This loop will receive the messages sent by the spawned thread
    for received in rx {
        println!("Got: {}", received);
    }

    println!("----------------------------------");

    // We can also use multiple producer channels
    // We can clone the sending end of a channel to send messages from multiple threads.
    let (tx, rx) = mpsc::channel();
    let tx2 = mpsc::Sender::clone(&tx);

    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(time::Duration::from_secs(1));
        }
    });

    thread::spawn(move || {
        let vals = vec![
            String::from("hello"),
            String::from("this"),
            String::from("is"),
            String::from("another"),
            String::from("thread"),
        ];

        for val in vals {
            tx2.send(val).unwrap();
            thread::sleep(time::Duration::from_secs(1));
        }
    });

    for received in rx {
        println!("Got: {}", received);
    }
}

fn shared_state() {
    let m = Mutex::new(5);

    {
        // The lock method returns a smart pointer called MutexGuard, wrapped in a LockResult.
        // It will block the current thread until it is able to acquire the lock.
        let mut num = m.lock().unwrap();
        // We can use the dereference operator to modify the value inside the Mutex.
        // Releasing the lock is done automatically when the MutexGuard goes out of scope.
        *num = 6;
    }
    // Poisoned means that the thread holding the lock panicked.
    println!("m: {:?}", m);

    // Transfering state between threads
    // This will count the number of threads that have been created.
    // In order to have multiple owners of the counter, we need the Arc type.
    // Arc stands for atomic reference counter, basically a thread-safe Rc.
    let counter = Arc::new(Mutex::new(0));

    let mut handles = vec![];

    // This will create 10 threads that will increment the counter.
    for _ in 0..10 {
        // We need to clone the Rc type to be able to use it in multiple threads.
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
        });
        handles.push(handle);
    }

    // We need to wait for all the threads to finish before we can print the counter.
    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}
