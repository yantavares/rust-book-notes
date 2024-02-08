# Chapter 12 - Writing a CLI Application in Rust

## Introduction

This chapter covers the essentials of writing a command-line interface (CLI) application in Rust, focusing on accessing and handling command-line arguments, reading from the filesystem, managing process exit statuses, and leveraging environment variables. We'll also touch on error handling using Rust's `Result` type and trait objects for errors.

## Handling Command-Line Arguments with `std::env`

- `std::env` is a module that allows access to the command-line arguments passed to a Rust program.
- Using `std::env::args().collect()`, we can obtain a `Vec<String>` containing the command-line arguments. If no arguments are passed, this method returns a vector with one element: the name of the program.
  - Example:
    ```bash
    $ cargo run needle haystack
    ```
    This will return `["target/debug/minigrep", "needle", "haystack"]`.

## Filesystem Access with `std::fs`

- The `std::fs` module provides access to the filesystem, including functions to read and write files.
- `fs::read_to_string()` is a function that reads the contents of a file into a `String`, returning a `Result` with the contents if successful.

## Process Management with `std::process`

- `std::process` allows spawning new processes, interacting with them, and managing their exit statuses. This module enables us to exit the program with a non-zero status code in case of errors without using `panic!`.

## Error Handling

- `unwrap_or_else()` is a method for handling the `Result` type. It returns the value inside `Ok` if the result is `Ok`, or it executes a closure passed to it if the result is `Err`.
- `dyn Error` is a trait object for returning errors. It allows returning any type that implements the `Error` trait without specifying the exact type.

## Environment Variables

- Environment variables can be set in the terminal using `export` (e.g., `export CASE_INSENSITIVE=1`) and unset with `unset` (e.g., `unset CASE_INSENSITIVE`).
- Environment variables are specific to the terminal session and are not global; they are unset when the session ends.

## Miscellaneous

- `eprintln!` is used for printing to the standard error stream, which is useful for separating error messages from normal output.
- The unit type `()` represents an empty value or no value. Functions that don't explicitly return a value return `()` by default.
