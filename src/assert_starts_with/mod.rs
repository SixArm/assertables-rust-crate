//! Assert for a sequence that may start with a part.
//!
//! These macros help with comparison of a sequence (such as a string, vector, range)
//! and a part (such as a string substring, an array element, a range value).
//!
//! * [`assert_starts_with(sequence, subsequence)`](macro@crate::assert_starts_with) ≈ container.contains(containee)
//!
//! * [`assert_not_starts_with!(sequence, subsequence)`](macro@crate::assert_not_starts_with) ≈ !container.contains(containee)
//!
//!
//! # Example
//!
//! ```rust
//! use assertables::*;
//!
//! # fn main() {
//! // String starts with substring?
//! let sequence: &str = "alfa";
//! let subsequence: &str = "al";
//! assert_starts_with!(sequence, subsequence);
//!
//! // Vector starts with element?
//! let sequence = vec![1, 2, 3];
//! let subsequence = [1];
//! assert_starts_with!(sequence, subsequence);
//! # }
//! ```

pub mod assert_not_starts_with;
pub mod assert_starts_with;
