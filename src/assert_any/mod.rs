//! Assert for comparing any element of an iterable.
//!
//! * [`assert_any(iter, predicate)`](macro@crate::assert_any) ≈ iter a any element predicate is true
//!
//! These macros help with comparison of iter parameters, such as a iter of struct
//! and a single struct.
//!
//! * [`assert_any_eq_x!(iter, item)`](macro@crate::assert_any_eq_x) ≈ iter a any element = b
//! * [`assert_any_ne_x!(iter, item)`](macro@crate::assert_any_ne_x) ≈ iter a any element ≠ b
//! * [`assert_any_lt_x!(iter, item)`](macro@crate::assert_any_gt_x) ≈ iter a any element < b
//! * [`assert_any_le_x!(iter, item)`](macro@crate::assert_any_gt_x) ≈ iter a any element ≤ b
//! * [`assert_any_gt_x!(iter, item)`](macro@crate::assert_any_gt_x) ≈ iter a any element > b
//! * [`assert_any_ge_x!(iter, item)`](macro@crate::assert_any_ge_x) ≈ iter a any element ≥ b
//!
//! # Example
//!
//! ```rust
//! use assertables::*;
//!
//! let a = [1, 2];
//! let b = 0;
//! assert_any_gt_x!(a.iter(), b);
//! ```

// Comparisons
pub mod assert_any;
pub mod assert_any_eq_x;
pub mod assert_any_ge_x;
pub mod assert_any_gt_x;
pub mod assert_any_le_x;
pub mod assert_any_lt_x;
pub mod assert_any_ne_x;
