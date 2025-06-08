//! Assert matches for verifying an item matches a case.
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
//! let a = 'a';
//! assert_matches!(a, 'a'..='z');
//! ```

pub mod assert_matches;
pub mod assert_not_matches;
