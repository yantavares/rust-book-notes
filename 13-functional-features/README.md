# Chapter 13 - Functional Features: Iterators and Closures in Rust

## Closures

Closures in Rust are anonymous functions that can capture their environment, allowing them to access variables from the scope in which they're defined. Here's a basic example:

```rust
let x = 4;
let equal_to_x = |z| z == x;
let y = 4;
assert!(equal_to_x(y)); // true
```

The closure `|z| z == x` captures the value of `x` from its environment. Closures can capture values in three ways:

- By taking ownership
- By borrowing mutably
- By borrowing immutably

Rust's type inference allows us not to specify types for closure parameters or environment, simplifying their usage. However, once the type of a parameter is inferred, it cannot be changed:

```rust
let example = |z| z == x;
let s = example(String::from("hello"));
let n = example(5); // This line will cause a compile-time error
```

### Fn Traits

Closures implement one or more of the `Fn`, `FnMut`, and `FnOnce` traits to capture values from the environment:

- `FnOnce` consumes variables it captures.
- `FnMut` captures variables by mutable reference.
- `Fn` captures variables by reference.

Functions can also implement these traits, allowing them to be passed as arguments to other functions or used in structs, although they cannot capture the environment. A closure can be forced to take ownership of captured variables using the `move` keyword, which is necessary when the closure is returned from a function or passed between threads.

## Iterators

Iterators are used to perform operations on a sequence of items lazily. The `Iterator` trait defines the core functionality of iterators, primarily through the `next` method, which returns an `Option` indicating the next item in the sequence or `None` if there are no more items.

Iterators are central to Rust's approach to handling collections and other sequences of data. For example:

```rust
let v1 = vec![1, 2, 3];
let v1_iter = v1.iter();
for val in v1_iter {
    println!("Got: {}", val);
}
```

Iterators can be used directly with loops or transformed using adapter methods like `map`:

```rust
let v1: Vec<i32> = vec![1, 2, 3];
let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();
assert_eq!(v2, vec![2, 3, 4]);
```

They provide a variety of functionality for processing sequences of data, including:

- Returning immutable references (`iter`), mutable references (`iter_mut`), or owned values (`into_iter`).
- Consuming the iterator to produce a single value, like summing numbers.
- Producing new iterators with modified sequences, such as applying a transformation to each element.

Creating custom iterators involves implementing the `Iterator` trait and its `next` method, enabling the use of all iterator methods like `map`, `filter`, `collect`, etc.
