# Chapter 11 - Testing in Rust

Testing is a crucial part of software development, ensuring code behaves as expected. Rust provides a comprehensive testing framework that supports various types of tests, including unit tests, integration tests, and documentation tests. This chapter outlines how to write and organize tests in Rust.

## Basics of Testing in Rust

- **`#[test]` Attribute**: Marks a function as a test case.
- **Running Tests**: Use `cargo test` to run all tests in a project.
- **Success Criteria**: A test passes if it doesn't panic and returns `Ok`.

## Assertions

- **`assert!` Macro**: Verifies its argument is `true`.
- **`assert_eq!` and `assert_ne!` Macros**: Check for equality and inequality, respectively.
- **Custom Error Messages**: Assertions can include custom messages, e.g., `assert!(1 == 2, "1 is not equal to 2");`.

## Handling Test Failures

- **`should_panic` Attribute**: Tests that a function panics under certain conditions.
- **`Result` Type in Tests**: Allows returning `Ok` or `Err` from a test, utilizing the `?` operator for early returns.

## Test Configuration

- **Parallel vs. Sequential**: Control parallelism with `-- --test-threads=n`. Use `1` for sequential execution.
- **Capturing Output**: Standard output is hidden by default but can be shown with `-- --show-output`.
- **Running Specific Tests**: Target specific tests by name or pattern with `cargo test <name>`.

## Ignoring Tests

- **`#[ignore]` Attribute**: Excludes tests from the default test suite.
- **Running Ignored Tests**: Use `-- --ignored` to include tests marked with `#[ignore]`.

## Test Organization

### Unit Tests

- **Location**: Typically reside in the same file as the code they test, under a `mod tests` guarded by `#[cfg(test)]`.
- **Access**: Can test both public and private functions.

### Integration Tests

- **Location**: Stored in the `tests` directory at the project root, outside the `src` directory.
- **Scope**: Test the interaction between multiple modules or crates.
- **Note**: Each file in the `tests` directory is treated as a separate crate.

### Documentation Tests

- **Purpose**: Test examples provided in the documentation.
- **Location**: Embedded in doc comments (`///` or `//!`).

## Special Considerations

- **Conditional Compilation**: The `#[cfg(test)]` attribute includes code only when compiling tests, useful for test utilities and setup.
- **Integration Tests in Binary Crates**: Directly running integration tests in binary crates is not supported; typically, a library crate is used for the bulk of the code, with the binary crate acting as a thin wrapper.
- **Subdirectories in `tests`**: To recognize tests in subdirectories, declare modules with `mod` in a parent file.
