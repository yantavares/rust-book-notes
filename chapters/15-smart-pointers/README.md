# Chapter 15 - Smart Pointers in Rust

- Box Smart Pointer is used to store data on the heap.
e.g.
```rust
let b = Box::new(5);
```
- With this code, we are storing the value 5 on the heap and b is a smart pointer to the heap location.
- The Box<T> type is a smart pointer because it implements the Deref trait, which allows Box<T> values to be treated like references.
- Useful when we have a type whose size can't be known at compile time and we want to use a value of that type in a context that requires an exact size.
- A smart pointer gets automatically dropped when it goes out of scope.
- The Drop trait is used to implement the drop method for a smart pointer.
- Box implements the Drop and Deref traits.
- The Deref trait allows us to override the * operator (dereference operator).
- Implicit deref coercion is a feature that allows Rust to automatically convert a reference to a type that implements Deref into a reference to a type that Deref returns.
e.g.
```rust
fn hello(name: &str) {
    println!("Hello, {}!", name);
}

fn main() {
    let m = Box::new(String::from("Rust"));
    hello(&m);
}
```
- The hello function takes a &str as an argument, but we can pass a Box<String> value to the hello function because of deref coercion.
- Rust performs deref coercion when it finds types and trait implementations in three cases:
    - From &T to &U when T: Deref<Target=U>
    - From &mut T to &mut U when T: DerefMut<Target=U>
    - From &mut T to &U when T: Deref<Target=U>

    - Reference counting is a way to keep track of the number of references to a value and to clean up that value when the number of references is zero.
    - The Rc<T> type is a reference counting smart pointer.
    - The Rc<T> type is used to allocate memory on the heap and to keep track of the number of references to that memory.
    - The Rc<T> type is used to share ownership of a value between multiple parts of a program.
    - Rc<T> is a reference-counted smart pointer
    - It keeps track of the number of references to a value and deallocates the value when the number of references is zero
    - Rc<T> is only for use in single-threaded scenarios
    - Rc<T> is not safe to use in multithreaded scenarios
    - Rc<T> is not atomic, so it cannot be used in a concurrent setting
    - If we want to use Rc<T> in a multithreaded scenario, we can use the std::sync::Arc type instead
    - Arc<T> is an atomic reference-counted smart pointer
    - Arc<T> is safe to use in concurrent scenarios
