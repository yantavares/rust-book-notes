# Chapter 17 - Object-Oriented Features in Rust

Rust provides several object-oriented features, allowing developers to encapsulate data and behavior within their applications. Unlike traditional object-oriented languages that rely heavily on inheritance, Rust uses a combination of structs, enums, and traits to achieve polymorphism and encapsulation, promoting a more composition-over-inheritance approach.

## Structs and Enums as Objects

In Rust, `structs` and `enums` serve as the primary means to create custom data types that encapsulate data. These can be considered as Rust's version of objects.

- **Structs**: Allow you to create complex types that group together pieces of data.
- **Enums**: Enable you to define a type by enumerating its possible variants.

Example of a struct in Rust:

```rust
struct Point {
    x: i32,
    y: i32,
}
```

## Methods and Implementation Blocks

Rust allows defining methods on structs and enums using `impl` blocks. This is how behavior is added to Rust's objects.

Example of an `impl` block:

```rust
struct Rectangle {
    p1: Point,
    p2: Point,
}

impl Rectangle {
    fn area(&self) -> i32 {
        let length = self.p2.x - self.p1.x;
        let width = self.p2.y - self.p1.y;
        return length * width;
    }
}
```

## Traits for Common Behavior

Traits in Rust are used to define a set of methods that a type must implement, similar to interfaces in other languages. Traits are essential for achieving polymorphism in Rust.

- **Trait Objects**: To achieve object behavior, Rust uses trait objects. Trait objects allow for multiple types to be stored in the same structure at runtime, enabling dynamic polymorphism.
- **Dynamic Dispatch**: Trait objects use dynamic dispatch, where the method to be called is determined at runtime. This contrasts with static dispatch, where the method call is resolved at compile time.

## Composition Over Inheritance

Rust does not support inheritance in the traditional sense. Instead, Rust encourages using composition to assemble multiple objects into a new object, thereby achieving similar outcomes to inheritance.

Example of composition:

```rust
struct Rectangle {
    p1: Point,
    p2: Point,
}
```

## Storing Trait Objects

- **Heap Storage**: Use `Box<dyn Trait>` to store trait objects on the heap.
- **Stack Storage**: Use `&dyn Trait` for stack storage of trait objects.
- **Function Return Types**: The `impl Trait` syntax can be used to return objects that implement a specific trait.

## Object Safety Rules

To ensure that a trait can be used as a trait object, it must adhere to certain rules:
1. The return type cannot be `Self`.
2. There are no generic type parameters.

These rules ensure the compiler can determine the size of the object at compile time, which is crucial for maintaining Rust's safety and performance guarantees.
