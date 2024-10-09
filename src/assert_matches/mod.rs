//! Assert matches for verifying an item matches a condition.
//!
//! Compare a length with another length:
//!
//! * [`assert_matches!(a, b)`](macro@crate::assert_matches) ≈ match(a) { b }
//!
//! * [`assert_not_matches!(a, b)`](macro@crate::assert_matches) ≈ match(a) { b }
//! 
//! # Example
//!
//! ```rust
//! use assertables::*;
//! 
//! # fn main() {
//! let a = 'a';
//! assert_matches!(a, 'a'..='z');
//! # }
//! ```

pub mod assert_matches;
pub mod assert_not_matches;