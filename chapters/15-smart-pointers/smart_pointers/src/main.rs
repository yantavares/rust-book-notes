use std::ops::Deref;

fn main() {
    box_smart_pointer();
    deref_trait();
    deref_coercion();
    drop_trait();
    manually_drop();
    reference_counted_smart_pointer();
}

fn box_smart_pointer() {
    let b = Box::new(5); // b is a smart pointer to a heap-allocated integer
    println!("b = {}", b);

    // b is a smart pointer, so we can dereference it to get the value it points to
    assert_eq!(5, *b);

    // b is a smart pointer, so it can be moved to a new variable
    let c = b;
    println!("c = {}", c);

    // Example usage

    /*
    // Since List is recursive, compiler cannot define its size
    // So, the code below breaks.

    enum List {
        // `Cons` refers to the Cons List, famous in Lisp
        Cons(i32, List), // Recursive type
        Nil,
    }

    */

    // To fix the above issue, we can use Box to create a smart pointer
    // Rust can now figure out the size of the List type because it knows the size of a Box
    // The error happened because there was no way to know how much space a List would need
    #[derive(Debug)]
    enum List {
        Cons(i32, Box<List>),
        Nil,
    }

    use List::{Cons, Nil};

    // Create a new list
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));

    // Print the list
    println!("list = {:?}", list);
}

// Implementing a smart pointer
struct MyBox<T>(T); // Tuple struct

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

// Implementing the Deref trait
impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.0
    }
}

fn deref_trait() {
    let x = 5;
    let y = &x;
    let z = Box::new(x);

    assert_eq!(5, x); // true
    assert_eq!(5, *y); // true
                       // The * operator is the dereference operator
                       // It allows us to get the value a reference is pointing to

    assert_eq!(5, *z); // true

    let x = 5;
    let y = MyBox::new(x);

    assert_eq!(5, x); // true
    assert_eq!(5, *y); // true
}

fn deref_coercion() {
    // Deref coercion is a convenience that Rust performs on arguments to functions and methods
    // It converts a reference to a type that implements Deref into a reference to a type that Deref can convert the original type into
    // Deref coercion happens automatically when we pass a reference

    // Example
    fn hello(name: &str) {
        println!("Hello, {}!", name);
    }

    let m = MyBox::new(String::from("Rust"));
    hello(&m); // Rust automatically converts &MyBox<String> to &String
               // &MyBox<String> (deref-> &String (deref-> &str

    // Without Deref coercion, we would have to write the code below
    hello(&(*m)[..]); // (*m) dereferences the MyBox<String> into a String
                      // Then & and [..] take a string slice of the String
}

struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}

fn drop_trait() {
    // The Drop trait provides a way to run some code when a value goes out of scope
    // The most common use of the Drop trait is to free the resources that the implementor instance owns
    // The Drop trait requires us to implement one method named drop that takes a mutable reference to self
    // Rust calls the drop method automatically when a value goes out of scope, and we cannot call it explicitly

    let _c = CustomSmartPointer {
        data: String::from("my stuff"),
    };
    let _d = CustomSmartPointer {
        data: String::from("other stuff"),
    };
    println!("CustomSmartPointers created.");
    // c and d are dropped here
}

fn manually_drop() {
    // To manually drop a value before it goes out of scope, we can use the std::mem::drop function
    // This is rarely needed, because Rust automatically calls the drop method at the appropriate time
    let _e = CustomSmartPointer {
        data: String::from("some data"),
    };
    println!("CustomSmartPointer created.");
    drop(_e);
    println!("CustomSmartPointer dropped before the end of main.");
}

fn reference_counted_smart_pointer() {
    // Rc<T> is a reference-counted smart pointer
    // It keeps track of the number of references to a value and deallocates the value when the number of references is zero
    // Rc<T> is only for use in single-threaded scenarios
    // Rc<T> is not safe to use in multithreaded scenarios
    // Rc<T> is not atomic, so it cannot be used in a concurrent setting
    // If we want to use Rc<T> in a multithreaded scenario, we can use the std::sync::Arc type instead
    // Arc<T> is an atomic reference-counted smart pointer
    // Arc<T> is safe to use in concurrent scenarios

    use std::rc::Rc;

    let a = Rc::new(5);
    let b = Rc::clone(&a);
    let c = Rc::clone(&a);

    println!("a = {}, b = {}, c = {}", a, b, c);

    // Lets suppose we want a Cons list that can have multiple owners
    /*

    b ---> [3, ] --\
                    a ---> [5, ] ---> [10, ] ---> Nil
    c ---> [4, ] --/

    */

    #[derive(Debug)]
    enum List {
        Cons(i32, Rc<List>),
        Nil,
    }

    use List::{Cons, Nil};

    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    let b = Cons(3, Rc::clone(&a)); // Clone only increases the reference count, not the data
    let c = Cons(4, Rc::clone(&a));

    println!("a = {:?}, b = {:?}, c = {:?}", a, b, c);

    // We can find the number of references to a value using the Rc::strong_count function
    println!("a strong = {}", Rc::strong_count(&a));

    // When we call Rc::clone, the reference count to the data within the Rc is increased
    // When the reference count drops to zero, the Rc will deallocate the data as well
}

// * RefCell and the interior mutability pattern

trait Messenger {
    fn send(&self, msg: &str);
}

#[allow(dead_code)]
struct LimitTracker<'a, T: Messenger> {
    messenger: &'a T,
    value: usize,
    max: usize,
}

#[allow(dead_code)]
impl<'a, T> LimitTracker<'a, T>
where
    T: Messenger,
{
    fn new(messenger: &T, max: usize) -> LimitTracker<T> {
        LimitTracker {
            messenger,
            value: 0,
            max,
        }
    }

    fn set_value(&mut self, value: usize) {
        self.value = value;

        let percentage_of_max = self.value as f64 / self.max as f64;

        if percentage_of_max >= 1.0 {
            self.messenger.send("Error: You are over your quota!");
        } else if percentage_of_max >= 0.9 {
            self.messenger
                .send("Urgent warning: You've used up over 90% of your quota!");
        } else if percentage_of_max >= 0.75 {
            self.messenger
                .send("Warning: You've used up over 75% of your quota!");
        }
    }
}

// We may need to test the LimitTracker struct
// We can use a mock object to test the LimitTracker struct
// A mock object is a fake object that we can use to test the behavior of a real object

// We can use the RefCell<T> type to mutate the value of a field inside an Rc<T> type

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    struct MockMessenger {
        // We use RefCell to mutate the value of sent_messages
        sent_messages: RefCell<Vec<String>>,
    }

    impl MockMessenger {
        fn new() -> MockMessenger {
            MockMessenger {
                // We use RefCell to mutate the value of sent_messages
                sent_messages: RefCell::new(vec![]),
            }
        }
    }

    impl Messenger for MockMessenger {
        fn send(&self, message: &str) {
            // We use borrow_mut to mutate the value of sent_messages
            // We need to use borrow_mut because send takes an immutable reference to self,
            // so we cannot mutate the value of sent_messages directly
            self.sent_messages.borrow_mut().push(String::from(message));
        }
    }

    #[test]
    fn it_sends_an_over_75_percent_warning_message() {
        let mock_messenger = MockMessenger::new();
        let mut limit_tracker = LimitTracker::new(&mock_messenger, 100);

        limit_tracker.set_value(80);

        assert_eq!(mock_messenger.sent_messages.borrow().len(), 1);
    }
}
