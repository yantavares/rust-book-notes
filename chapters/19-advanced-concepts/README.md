# Chapter 19 - Advanced Features in Rust

This chapter dives into some of the more advanced features of Rust, providing insights and examples for understanding and using these features effectively.

## Unsafe Rust

With unsafe Rust, we can perform operations that the compiler can't guarantee to be safe. It's important to note that using unsafe Rust doesn't turn off the borrow checker or disable Rust's safety checks. Instead, it allows certain operations that are not allowed in safe Rust due to the potential risks they carry. To use unsafe code, we utilize the `unsafe` keyword and wrap the code in an `unsafe` block.

### Capabilities of Unsafe Rust

- **Dereference a raw pointer**: Raw pointers can be both mutable and immutable and don't guarantee pointing to valid memory.
- **Call an unsafe function or method**: Some functions/methods can only be called within an unsafe block or if they themselves are marked as unsafe.
- **Access or modify a mutable static variable**: Mutable static variables have a fixed address in memory and can be accessed across multiple threads, requiring unsafe code to modify.
- **Implement an unsafe trait**: If a trait is marked as unsafe, implementing it requires unsafe code.
- **Access fields of unions**: Unions allow for multiple types to be stored in the same memory location, accessing them requires unsafe code.

### Example of Using Unsafe Code

```rust
unsafe {
    // unsafe code here
}
```

## Raw Pointers

Raw pointers, unlike references and smart pointers, have fewer safety guarantees.

- Can ignore the borrowing rules (e.g., have mutable and immutable pointers to the same location).
- Aren't guaranteed to point to valid memory.
- Are allowed to be null.
- Don't implement automatic cleanup.

### Creating Raw Pointers

```rust
let mut num = 5;

let r1 = &num as *const i32;
let r2 = &mut num as *mut i32;
```

Dereferencing raw pointers requires an unsafe block.

## Calling Unsafe Functions

To call an unsafe function, prepend the call with the `unsafe` keyword.

### Example:

```rust
unsafe fn dangerous() {}

unsafe {
    dangerous();
}
```

## Using `extern` Functions to Call External Code

The `extern` keyword allows calling external code from other languages, an operation considered unsafe.

### Example:

```rust
extern "C" {
    fn abs(input: i32) -> i32;
}
```

## Advanced Traits

### Associated Types

Associated types provide a way to define a placeholder type within a trait, allowing the trait to use some types without specifying what those types are until implementation.

#### Example:

```rust
pub trait Iterator {
    type Item;
    fn next(&mut self) -> Option<Self::Item>;
}
```

Implementing a trait requires specifying the concrete type for the associated type.

### Generic Types in Traits

If necessary, we can change the associated type by using generic types.

#### Example:

```rust
pub trait Iterator<T> {
    fn next(&mut self) -> Option<T>;
}
```

### Default Types for Associated Types

Default types can be useful, especially when overriding operators.

#### Example:

```rust
impl Add for Point {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

trait Add<RHS=Self> {
    type Output;
    fn add(self, rhs: RHS) -> Self::Output;
}
```

## Macros

Macros in Rust allow for metaprogramming, or writing code that writes other code, which can be particularly useful for reducing repetitive code.

### Declarative Macros

These are the most common type of macros, used for generating code based on pattern matching.

#### Example:

```rust
macro_rules! vec {
    ( $( $x:expr ),* ) => {
        {
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push($x);
            )*
            temp_vec
        }
    };
}
```

### Procedural Macros

Procedural macros generate code using Rust code, allowing for custom derive, attribute-like macros, and function-like macros.

#### Custom Derive Example:

```rust
use proc_macro;

#[some_attribute]
pub fn some_name(input: TokenStream) -> TokenStream {
    // code here
}
```

#### Attribute-like Macros Example:

```rust
#[route(GET, "/")]
fn index() {}
```

#### Function-like Macros Example:

```rust
let sql = sql!(SELECT * FROM posts WHERE id=1);
```

Macros expand before the compilation process, allowing the compiler to see the expanded code as if it were written by the programmer, differing from functions that compile directly without prior expansion.

For detailed explanations and more examples, diving into Chapter 19 of the Rust documentation is highly recommended.
