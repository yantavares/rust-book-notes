# Chapter 4 - Ownership

## What is Ownership?

Ownership is Rust's most unique feature, and it enables Rust to make memory safety guarantees without needing a garbage collector. Therefore, it's important to understand how ownership works in Rust.

## --- Ownership Rules ---

1. Each value in Rust has a variable that's called its owner.
2. There can only be one owner at a time.
3. When the owner goes out of scope, the value will be dropped.

e.g.:

```rust
{
    let s = String::from("hello"); // s is valid from this point forward
    // do stuff with s
}   // this scope is now over, and s is no longer valid
```

**Also:**

- Slices do not have ownership.
- Primitive types have the `Copy` trait, so they are copied instead of moved.

## --- References Rules ---

1. At any given time, you can have either one mutable reference or any number of immutable references.
2. References must always be valid.
