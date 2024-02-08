# Ownership in Rust

Ownership is a key concept in Rust, a system programming language known for its focus on safety and performance. Here's an overview of the ownership model in Rust:

## 1. Ownership Rules

- Each value in Rust has a **variable** that's called its _owner_.
- There can be **only one owner** at a time.
- When the owner goes out of scope, the value will be **dropped**.

## 2. Variable Scope

- A variable is valid from the point it's declared until the end of the current scope.
- Rust automatically calls the `drop` function and cleans up resources when a variable goes out of scope.

## 3. Moving

- Assigning a variable to another or passing it to a function moves the ownership.
- The original variable is no longer valid after the move.
- Applies to types that do **not** implement the `Copy` trait.

## 4. Copying

- Types with the `Copy` trait (like integers and characters) do not move but are copied.
- Both the original and new variable can be independently used.

## 5. Borrowing

- **Borrowing** refers to referencing a value without taking ownership.
- Immutable references are created with `&`.
- Mutable references are created with `&mut`, with the rule of only one mutable reference to a data piece in a scope.

## 6. Lifetimes

- Lifetimes describe the scope for which a reference is valid.
- They prevent references from outliving the data they refer to.
- Often, lifetimes are implicitly inferred by the Rust compiler.

## 7. Functions and Ownership

- Passing a variable to a function transfers ownership to the function parameters.
- Parameters are dropped when the function scope ends.
- To use a variable after passing it to a function, pass a reference.

The Rust ownership model prevents common bugs like null pointer dereferencing, double free, and memory leaks. It ensures memory safety without a garbage collector, which is essential for systems programming.

# Rust Ownership Examples

## Example 1: Basic Ownership

```rust
fn main() {
    let s1 = String::from("hello");
    let s2 = s1;

    // println!("{}, world!", s1); // This line would cause a compile-time error
    println!("{}, world!", s2); // This line works fine
}
```

### Explanation

- `s1` is the owner of the `String` value `"hello"`.
- On assigning `s1` to `s2`, the ownership is moved to `s2`. Hence, `s1` is no longer valid.
- Using `s1` after this point would result in a compile-time error, as Rust enforces single ownership of a value.

## Example 2: Ownership and Functions

```rust
fn main() {
    let s = String::from("hello");

    takes_ownership(s);

    // println!("{}", s); // This line would cause a compile-time error
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}
```

### Explanation

- Here, `s` owns the `String` value.
- Passing `s` to `takes_ownership` transfers the ownership to `some_string`.
- Post this function call, `s` becomes invalid as its ownership has been transferred.

## Example 3: Returning Ownership

```rust
fn main() {
    let s1 = gives_ownership(); // Ownership is moved to s1

    let s2 = String::from("hello");
    let s3 = takes_and_gives_back(s2); // s2 is moved into the function and back to s3

    // println!("{}", s2); // This line would cause a compile-time error
}

fn gives_ownership() -> String {
    let some_string = String::from("hello");
    some_string // Returned and ownership is moved out to the calling function
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string // Returned and ownership is moved out to the calling function
}
```

### Explanation

- In `gives_ownership`, the ownership of the return value is moved to `s1`.
- `s2`'s ownership is moved to `takes_and_gives_back`, and then to `s3`.
- `s2` becomes invalid after being passed to `takes_and_gives_back`, due to ownership transfer.
