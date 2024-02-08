# Chapter 3 - Programming Concepts in Rust

## 3.1 - Variables

- **Immutability by Default**: Variables in Rust are immutable unless explicitly made mutable.
- **Mutable Variables**: Use the `mut` keyword to make a variable mutable.
- **Constants**: Constants are always immutable and are declared with the `const` keyword.
- **Constant Type Annotation**: Constants must have their type annotated.
- **Runtime Constants**: Constants cannot be set to the result of a function call or any value that is computed at runtime.
- **Shadowing**: Reuse the same variable name for a new value through shadowing.
- **Shadowing vs. Mutation**: Shadowing is distinct from mutation (`mut`); shadowing requires `let` and provides compile-time safety against accidental reassignment.

## 3.2 - Data Types

- **Scalar Data Types**:
  - **Integers**: Signed (`i32`, `i64`, etc.) or unsigned (`u32`, `u64`, etc.), with a fixed size.
  - **Floating-Point Numbers**: `f32` or `f64`.
  - **Booleans**: `true` or `false`.
  - **Characters**: Single quotes, e.g., `'a'`.
- **Compound Data Types**:
  - **Tuples**: Fixed length, can contain multiple data types.
  - **Arrays**: Fixed length, a single data type.
- **Integer Overflow**: Causes a panic in Rust.
  - **Release Mode Behavior**: In release mode, integer overflow results in two's complement wrapping.

## 3.3 - Functions

- **Naming Convention**: Use snake case for function and variable names.
- **Function Placement**: Functions can be defined anywhere, including inside other functions.
- **Implicit Return**: The last expression in a function is implicitly returned.
- **Statements**: Perform actions but do not return values.

## 3.4 - Control Flow

- **`if` Expressions**: Can be used on the right side of a `let` statement.
  - Example: `let number = if condition { 5 } else { 6 };`

## 3.5 - Loops

- **`loop`**: An infinite loop.
- **`while`**: Conditionally infinite loops.
- **`for`**: Iterate over a collection. Example: `for element in array.iter() { ... }`
- **Loop Labeling**: Loops can be labeled for specific control flow, e.g., `outer: loop { ... break outer; }`
- **Loop Return Values**: Loops can return values, e.g., `let result = loop { ... break result; }`
- **Range in Loops**: Use ranges in `for` loops, e.g., `for number in (1..4).rev() { ... }`

## 3.6 - Comments

- **Normal Comments**: Marked with `//`.
- **Documentation Comments**: Marked with `///`, support Markdown syntax.
- **Block Comments**: Enclosed in `/* ... */`.
