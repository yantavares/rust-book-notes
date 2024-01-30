# Chapter 10.1 - Generic Types in Rust

## What are Generic Types in Rust?
- Generic types are a way to define functions or structs that can work with any type.
- Generics are defined using angle brackets `<>`, and the type is specified inside these brackets.

## Using Generic Types in Functions
- You can define generic functions in Rust to work with different types.
- Here's an example of a generic function that finds the largest element in a list of any type:
  ```rust
  fn largest<T>(list: &[T]) -> T {
      let mut largest = list[0];

      for &item in list.iter() {
          if item > largest {
              largest = item;
          }
      }

      largest
  }
  ```

- You can also have multiple generic types in a function like this:
  ```rust
  fn largest<T, U>(list1: &[T], list2: &[U]) -> T {
      let mut largest = list1[0];

      for &item in list1.iter() {
          if item > largest {
              largest = item;
          }
      }

      largest
  }
  ```

- Type restrictions can be applied to generic types using traits. For example, the `PartialOrd` and `Copy` traits can be used to ensure that the generic type can be compared and copied, respectively:
  ```rust
  fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
      let mut largest = list[0];

      for &item in list.iter() {
          if item > largest {
              largest = item;
          }
      }

      largest
  }
  ```

## Using Generic Types in Structs
- Generic types can also be used in structs. Here's an example of a Point struct with a generic type:
  ```rust
  struct Point<T> {
      x: T,
      y: T,
  }
  ```
