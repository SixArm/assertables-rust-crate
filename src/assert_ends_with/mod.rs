//! Assert for a whole that may end with a part.
//!
//! These macros help with comparison of a whole (such as a string, array, range)
//! and a part (such as a string substring, an array element, a range value).
//!
//! * [`assert_ends_with(sequence, x)`](macro@crate::assert_ends_with) ≈ container.contains(containee)
//!
//! * [`assert_not_ends_with!(sequence, x)`](macro@crate::assert_not_ends_with) ≈ !container.contains(containee)
//!
//!
//! # Example
//!
//! ```rust
//! use assertables::*;
//!
//! # fn main() {
//! // String ends with substring?
//! let whole: &str = "alfa";
//! let part: &str = "fa";
//! assert_ends_with!(sequence, x);
//!
//! // Vector ends with element?
//! let whole = vec![1, 2, 3];
//! let part = [3];
//! assert_ends_with!(sequence, x);
//! # }
//! ```

pub mod assert_ends_with;
pub mod assert_not_ends_with;
