# Chapter 18 - Patterns and Matching in Rust

## Overview

In Rust, pattern matching is a powerful feature that allows you to handle different cases or scenarios efficiently. This chapter explores the concept of patterns and matching in Rust, covering various aspects such as exhaustiveness, placeholders, types of patterns, and where they can be used.

## Exhaustiveness in Matching

Matching in Rust is exhaustive, meaning that you must cover all possible cases in a match expression. This ensures that all potential scenarios are accounted for, leaving no room for undefined behavior or unexpected outcomes.

## Placeholder

The `_` placeholder in Rust can be used within patterns to match any value. It's particularly useful when you want to ignore specific values or cases.

## Types of Patterns

Rust distinguishes between two types of patterns:

1. Refutable Patterns: These patterns may not match for some possible values. They provide a way to handle cases where the match might fail.

2. Irrefutable Patterns: These patterns will match for any possible value. They are guaranteed to succeed and are commonly used in situations where matching is certain.

### Examples:

```rust
let x = Some(5);

// Refutable pattern
if let Some(5) = x {
    println!("It's 5");
}

// Irrefutable pattern (let statements are patterns in Rust)
let Some(y) = x;
```

## Usage of Irrefutable Patterns

Certain constructs in Rust only accept irrefutable patterns:

- Function parameters
- `let` statements
- `for` loops

This restriction ensures that these constructs operate reliably and predictably, without the risk of encountering unmatched patterns.
