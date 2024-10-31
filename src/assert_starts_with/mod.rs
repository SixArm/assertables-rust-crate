//! Assert for a whole that may start with a part.
//!
//! These macros help with comparison of a whole (such as a string, vector, range)
//! and a part (such as a string substring, an array element, a range value).
//!
//! * [`assert_starts_with(sequence, x)`](macro@crate::assert_starts_with) ≈ container.contains(containee)
//!
//! * [`assert_not_starts_with!(sequence, x)`](macro@crate::assert_not_starts_with) ≈ !container.contains(containee)
//!
//!
//! # Example
//!
//! ```rust
//! use assertables::*;
//!
//! # fn main() {
//! // String starts with substring?
//! let whole: &str = "alfa";
//! let part: &str = "al";
//! assert_starts_with!(sequence, x);
//!
//! // Vector starts with element?
//! let whole = vec![1, 2, 3];
//! let part = [1];
//! assert_starts_with!(sequence, x);
//! # }
//! ```

pub mod assert_not_starts_with;
pub mod assert_starts_with;
