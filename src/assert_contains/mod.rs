//! Assert for a container and a containee.
//!
//! These macros help with comparison of a container (such as a string, array, range)
//! and a containee (such as a string substring, an array element, a range value).
//!
//! * [`assert_contains(container, containee)`](macro@crate::assert_contains) ≈ container.contains(containee)
//!
//! * [`assert_not_contains!(container, containee)`](macro@crate::assert_not_contains) ≈ !container.contains(containee)
//!
//!
//! # Example
//!
//! ```rust
//! use assertables::*;
//!
//! # fn main() {
//! // String contains substring
//! let a: &str = "alfa";
//! let b: &str = "lf";
//! assert_contains!(a, b);
//!
//! // Range contains value
//! let a = 1..3;
//! let b = 2;
//! assert_contains!(a, &b);
//! # }
//! ```

pub mod assert_contains;
pub mod assert_not_contains;
