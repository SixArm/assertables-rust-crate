//! Assert for a sequence that may end with a part.
//!
//! These macros help with comparison of a sequence (such as a string, array, range)
//! and a part (such as a string substring, an array element, a range value).
//!
//! * [`assert_ends_with(sequence, subsequence)`](macro@crate::assert_ends_with) ≈ container.contains(containee)
//!
//! * [`assert_not_ends_with!(sequence, subsequence)`](macro@crate::assert_not_ends_with) ≈ !container.contains(containee)
//!
//!
//! # Example
//!
//! ```rust
//! use assertables::*;
//!
//! # fn main() {
//! // String ends with substring?
//! let sequence: &str = "alfa";
//! let subsequence: &str = "fa";
//! assert_ends_with!(sequence, subsequence);
//!
//! // Vector ends with element?
//! let sequence = vec![1, 2, 3];
//! let subsequence = [3];
//! assert_ends_with!(sequence, subsequence);
//! # }
//! ```

pub mod assert_ends_with;
pub mod assert_not_ends_with;
