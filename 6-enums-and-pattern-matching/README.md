# Chapter 6: Enums and Pattern Matching in Rust

## Enums in Rust

### Basic Concepts

- Enums in Rust are used when a type can be one of a few variants. This allows for clear enumeration of possible values.
- Enum variants in Rust typically use CamelCase naming convention, differing from the snake_case commonly used elsewhere in Rust.

  Example:

  ```rust
  enum IpAddrKind { V4, V6 }
  ```

### Namespacing and Data in Enums

- Enum variants are namespaced under the enum identifier, accessed using a double colon (`::`).
- Enums can hold data, allowing for more versatile structures.

  Example:

  ```rust
  enum IpAddr { V4(String), V6(String) }
  ```

### Methods on Enums

- Enums in Rust can have methods defined on them, adding to their flexibility and utility.

## The `Option` Enum

### Purpose

- The `Option` enum is a built-in feature in Rust used to express the possibility of absence of a value, acting as a safer alternative to null values found in many other languages.

### Definition

- The `Option` enum is generic and is defined as:

  ```rust
  enum Option<T> {
      Some(T),
      None,
  }
  ```

### Usage Examples

- `Option<T>` can be used to indicate the presence or absence of a value.

  Examples:

  ```rust
  let some_number = Some(5);
  let some_string = Some("a string");
  let absent_number: Option<i32> = None;
  ```

## Pattern Matching in Rust

### Exhaustiveness

- Pattern matching in Rust is exhaustive, meaning all possible cases must be accounted for in a match statement.

### Syntax and Usage

- The `_` pattern is used to match any case not explicitly handled.
- `if let` is a convenient syntax for matching a single pattern, providing a more concise alternative to `match` for simple cases.

  Examples:

  ```rust
  let some_u8_value = 0u8;
  match some_u8_value {
      1 => println!("one"),
      3 => println!("three"),
      5 => println!("five"),
      7 => println!("seven"),
      _ => (), // Do nothing
  }
  ```

  This is equivalent to:

  ```rust
  if let Some(3) = some_u8_value {
      println!("three");
  }
  ```
