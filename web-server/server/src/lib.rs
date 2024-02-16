use std::sync::mpsc;
use std::sync::{Arc, Mutex};
use std::thread;

// Public API for the ThreadPool

pub struct ThreadPool {
    workers: Vec<Worker>,
    // sender is used to send the jobs to the workers
    sender: mpsc::Sender<Message>,
}

// Worker struct will store the id and the thread
// This is useful to keep track of the threads in the pool instead of storing the threads directly
pub struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}

// Job type alias
// This is a type alias for a trait object that holds the type of closure that execute will execute
type Job = Box<dyn FnOnce() + Send + 'static>;

// This is the message that will be sent to the workers
// Either it will be a new job or a terminate message
enum Message {
    NewJob(Job),
    Terminate,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Message>>>) -> Worker {
        // We need to move the receiver to the thread to avoid the receiver being dropped
        // Loop will keep the thread alive and looking for jobs
        let thread = thread::spawn(move || loop {
            let message = receiver.lock().unwrap().recv().unwrap();

            // Match the message to see if it is a new job or a terminate message
            match message {
                Message::NewJob(job) => {
                    println!("Worker {} got a job; executing.", id);
                    job();
                }
                Message::Terminate => {
                    println!("Worker {} was told to terminate.", id);
                    break;
                }
            }
        });
        Worker {
            id,
            thread: Some(thread),
        }
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        println!("Sending terminate message to all workers.");
        // Looping through the workers
        for _ in &self.workers {
            self.sender.send(Message::Terminate).unwrap();
        }

        println!("Shutting down all workers.");
        for worker in &mut self.workers {
            println!("Terminating worker {}", worker.id);
            // If Option is Some, we want to take the value out of the Some variant
            if let Some(thread) = worker.thread.take() {
                // We want to wait for the thread to finish
                thread.join().unwrap();
            }
        }
    }
}

impl ThreadPool {
    /// Create a new ThreadPool
    ///
    /// The size is the number of threads in the pool
    ///
    /// # Panics
    ///
    /// The `new` function will panic if the size is 0 or less
    pub fn new(size: usize) -> ThreadPool {
        // Program will panic if the size is 0 or less
        assert!(size > 0);

        // Creating a new channel
        let (sender, receiver) = mpsc::channel();

        let mut workers = Vec::with_capacity(size as usize);

        let receiver = Arc::new(Mutex::new(receiver));

        for id in 0..size {
            // Creating the workers
            // We want the workers to share the receiver, so we need to use Arc smart pointer
            // Arc is a thread safe reference counting pointer

            // For thread mutability we can use Mutex smart pointer
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }

        ThreadPool { workers, sender }
    }

    // We want execute to work simmilar to the thread::spawn function
    pub fn execute<F>(&self, f: F)
    where
        // FnOnce means that the closure takes ownership of the variables
        // Send means that the closure can be sent to another thread
        // 'static means that the lifetime of the closure is the entire program
        F: FnOnce() + Send + 'static,
    {
        // We need to get a worker from the pool
        // and send the job (closure) to the worker
        let job = Box::new(f);
        // Send the job to the channel
        self.sender.send(Message::NewJob(job)).unwrap();
    }
}
