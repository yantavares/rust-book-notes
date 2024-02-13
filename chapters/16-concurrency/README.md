# Chapter 16 - Fearless Concurrency in Rust

## Introduction to Concurrency in Rust
Rust's approach to concurrency ensures memory safety and race condition avoidance through its ownership and type systems. In Rust, concurrency can be achieved in two ways:

- **Parallelism**: Running multiple tasks simultaneously.
- **Concurrency**: Managing multiple tasks at the same time even if not running them simultaneously.

Rust's model is influenced by the actor model, emphasizing independent entities (actors) communicating through message passing.

### Thread Types in Rust
Rust provides support for OS-level threads, which are managed by the operating system:

- **OS Threads**: These are the threads provided and managed by the operating system. Rust's `thread` module allows creation and management of OS threads.
- **Green Threads**: User-level threads that are managed by the language runtime. Rust does not natively use green threads; instead, concurrency can be achieved using the `async`/`await` syntax or external crates.

### The `thread` Module
The standard library's `thread` module is key to Rust's concurrency model, offering several functions to work with threads:

- **Creating a Thread**: Use the `spawn` function to create a new thread, which takes a closure as an argument to specify the thread's execution logic.
  ```rust
  use std::thread;

  fn main() {
      let handle = thread::spawn(|| {
          for i in 1..10 {
              println!("hi number {} from the spawned thread", i);
          }
      });

      for i in 1..5 {
          println!("hi number {} from the main thread", i);
      }

      handle.join().unwrap();
  }
  ```
- **Joining a Thread**: The `join` method waits for a thread to finish its execution, ensuring synchronization.
- **Thread Control**: Functions like `sleep` to pause thread execution and `yield_now` to yield execution to another thread are available.

### Moving Data into Threads
To pass data into a thread, Rust uses the `move` keyword in closures to transfer ownership, ensuring safe access across thread boundaries.

### Channels for Inter-thread Communication
Rust implements channels through the `std::sync::mpsc` module (multiple producer, single consumer) for message passing between threads.

### Shared State Concurrency
Rust allows shared state concurrency using mechanisms like mutexes to protect data access across threads. The `Mutex` type provides mutual exclusion, with `Arc` enabling atomic reference counting for shared ownership across threads.

### Concurrency Primitives
- **Mutex**: Mutual exclusion lock for protecting shared data.
- **Arc**: Atomically Reference Counted, used to share ownership of mutable data across threads safely.

### Handling Synchronization and Errors
- Mutexes and channels come with methods that return `Result` types to handle potential errors, such as a thread panicking while holding a lock or communication issues in channels.

### Traits for Concurrency
- **Send**: Types that can be transferred safely between threads.
- **Sync**: Types that can be safely shared between threads.
