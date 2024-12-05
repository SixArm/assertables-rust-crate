//! Assert for comparing delta differences.
//!
//! Compare a delta with an expression:
//!
//! * [`assert_diff_eq_x!(a, b, expr)`](macro@crate::assert_diff_eq_x) ≈ b - a = expr
//! * [`assert_diff_ne_x!(a, b, expr)`](macro@crate::assert_diff_ne_x) ≈ b - a ≠ expr
//! * [`assert_diff_lt_x!(a, b, expr)`](macro@crate::assert_diff_lt_x) ≈ b - a < expr
//! * [`assert_diff_le_x!(a, b, expr)`](macro@crate::assert_diff_le_x) ≈ b - a ≤ expr
//! * [`assert_diff_gt_x!(a, b, expr)`](macro@crate::assert_diff_gt_x) ≈ b - a > expr
//! * [`assert_diff_ge_x!(a, b, expr)`](macro@crate::assert_diff_ge_x) ≈ b - a ≥ expr
//!
//! # Example
//!
//! ```rust
//! use assertables::*;
//!
//! let a: i8 = 10;
//! let b: i8 = 12;
//! let x: i8 = 2;
//! assert_diff_eq_x!(a, b, x);
//! ```

pub mod assert_diff_eq_x;
pub mod assert_diff_ge_x;
pub mod assert_diff_gt_x;
pub mod assert_diff_le_x;
pub mod assert_diff_lt_x;
pub mod assert_diff_ne_x;
