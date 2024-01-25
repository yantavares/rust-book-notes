# Chapter 6 - Enums and Pattern Matching

- For some reason, enums use camel case instead of snake case for their variants: `enum IpAddrKind { V4, V6 }`
- Enums are useful when you have a type that can be one of a few variants, and you want to be able to enumerate those variants
- Variants of an enum are namespaced under its identifier, and we use a double colon to separate the two
- Enums can receive data as well: `enum IpAddr { V4(String), V6(String) }`
- You can also define methods on enums (crazy)

- Option is a built-in enum that is used when a value could be something or nothing
- It is defined as follows:

```rust
enum Option<T> {
    Some(T),
    None,
}
```

- There are no null values in Rust, but you can use `Option<T>` to encode the fact that a value could be missing
- You can also use `Option<T>` in a match statement:

```rust
let some_number = Some(5);
let some_string = Some("a string");

let absent_number: Option<i32> = None;
```
