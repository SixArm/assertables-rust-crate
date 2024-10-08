//! Assert for method is_match(…).
//!
//! These macros help with any item that implements self.is_match(…).
//!
//! * [`assert_is_match!(matcher, matchee)`](macro@crate::assert_is_match) ≈ matcher.is_match(matchee)
//!
//! * [`assert_not_match!(matcher, matchee)`](macro@crate::assert_not_match) ≈ !matcher.is_match(matchee)
//!
//! # Example
//!
//! ```rust
//! use assertables::*;
//! use regex::Regex;
//!
//! # fn main() {
//! let a = Regex::new(r"lf").unwrap();
//! let b = "alfa";
//! assert_is_match!(a, b);
//! # }
//! ```

pub mod assert_is_match;
pub mod assert_not_match;
