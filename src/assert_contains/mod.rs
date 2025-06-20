//! Assert for a container and a containee.
//!
//! These macros help with comparison of a container (such as a string, array, range)
//! and a containee (such as a string substring, an array element, a range value).
//!
//! * [`assert_contains(container, containee)`](macro@crate::assert_contains) ≈ container.contains(containee)
//!
//! * [`assert_not_contains!(container, containee)`](macro@crate::assert_not_contains) ≈ !container.contains(containee)
//!
//! These macros work with many kinds of Rust types, such as String, Vec, Range, HashSet.
//! The specifics depend on each type's implementation of a method `contains`, and some types
//! require the second argument to be borrowable, so be sure to check the Rust documentation.
//!
//! # Example
//!
//! ```rust
//! use assertables::*;
//!
//! // String contains substring
//! let a: &str = "alfa";
//! let b: &str = "lf";
//! assert_contains!(a, b);
//!
//! // Range contains value
//! let a = 1..3;
//! let b = 2;
//! assert_contains!(a, &b); // Borrow
//!
//! // Vector contains &element
//! let a = vec![1, 2, 3];
//! let b = 2;
//! assert_contains!(a, &b); // Borrow
//! ```

pub mod assert_contains;
pub mod assert_not_contains;
