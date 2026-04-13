//! Assert for comparing all elements of an iterable.
//!
//! * [`assert_all!(iter, predicate)`](macro@crate::assert_all) ≈ iter a all elements predicate is true
//!
//! These macros help with comparison of iter parameters, such as a iter of struct
//! and a single struct.
//!
//! * [`assert_all_eq_x!(iter, item)`](macro@crate::assert_all_eq_x) ≈ iter a all elements = b
//! * [`assert_all_ne_x!(iter, item)`](macro@crate::assert_all_ne_x) ≈ iter a all elements ≠ b
//! * [`assert_all_lt_x!(iter, item)`](macro@crate::assert_all_gt_x) ≈ iter a all elements < b
//! * [`assert_all_le_x!(iter, item)`](macro@crate::assert_all_gt_x) ≈ iter a all elements ≤ b
//! * [`assert_all_gt_x!(iter, item)`](macro@crate::assert_all_gt_x) ≈ iter a all elements > b
//! * [`assert_all_ge_x!(iter, item)`](macro@crate::assert_all_ge_x) ≈ iter a all elements ≥ b
//!
//! # Example
//!
//! ```rust
//! use assertables::*;
//!
//! let a = [1, 2];
//! let b = 0;
//! assert_all_gt_x!(a.iter(), b);
//! ```

// Comparisons
pub mod assert_all;
pub mod assert_all_eq_x;
pub mod assert_all_ge_x;
pub mod assert_all_gt_x;
pub mod assert_all_le_x;
pub mod assert_all_lt_x;
pub mod assert_all_ne_x;
