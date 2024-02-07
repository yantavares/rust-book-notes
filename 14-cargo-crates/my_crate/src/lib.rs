//! # My Crate
//!
//! `my_crate` is a collection of utilities to make performing certain
//! calculations more convenient.

/// Adds one to the number given.
///
/// # Examples
///
/// ```
/// let arg = 5;
/// let answer = my_crate::add_one(arg);
///
/// assert_eq!(6, answer);
/// ```
pub fn add_one(x: i32) -> i32 {
    x + 1
}

// In order for other code to use IntPair and FloatPair without having to
// specify the module path, we can re-export them from the root of the crate.

pub use kinds::FloatPair;
pub use kinds::IntPair;

// Now we don't have to specify the module path when using IntPair and FloatPair
// in other code. For example, we can use them like this:
// use my_crate::IntPair;

pub mod kinds {
    /// Two-element tuple of i32
    pub type IntPair = (i32, i32);
    /// Two-element tuple of f32
    pub type FloatPair = (f32, f32);
}
