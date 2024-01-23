# Chapter 2 - Guessing Game

## Notes

### Basics of Rust Language

- **`use`**: Used to bring a library into scope.
- **Immutability**: Variables are immutable by default.
- **`let`**: Used to create a variable.
- **`mut`**: Used to make a variable mutable.
- **`String::new()`**: Creates a new empty string.

### Input and Output

- **Reading Input**: `io::stdin().read_line(&mut guess)` is used to read a line from the standard input.
- **References**: `&` is used to indicate that the argument is a reference.
- **Handling Input Results**: `read_line` returns a Result type, which is an enum with variants Ok and Err.
- **Error Handling**: `expect` is used to handle the Err variant of the Result type.

### Working with Dependencies

- **Cargo.toml**: After adding new dependencies to Cargo.toml, run `cargo build` to download and compile them.

### Comparison and Enums

- **Comparisons**: `std::cmp` is used for comparing values.
- **Ordering Enum**: Ordering is an enum with variants Less, Greater, and Equal.

### String Manipulation and Parsing

- **Trimming Strings**: `str.trim()` is used to remove whitespace from a string.
- **Parsing Strings**: `str.parse()` is used to parse a string into some kind of number.

### Control Flow

- **`match` Statement**: Used to handle the Result type.
- **Variable Shadowing**: Used to convert a value from one type to another.
