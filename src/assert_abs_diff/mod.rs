//! Assert for comparing absolute differences.
//!
//! These macros help with collection lengths, such as for strings, arrays,
//! vectors, iterators, and anything that has a typical `.len()` method.
//!
//! Compare an absolute difference with an expression:
//!
//! * [`assert_abs_diff_eq_x!(a, b, x)`](macro@crate::assert_abs_diff_eq) ≈ | a - b | = x
//!
//! * [`assert_abs_diff_ne_x!(a, b, x)`](macro@crate::assert_abs_diff_ne) ≈ | a - b | ≠ x
//!
//! * [`assert_abs_diff_lt_x!(a, b, x)`](macro@crate::assert_abs_diff_lt) ≈ | a - b | < x
//!
//! * [`assert_abs_diff_le_x!(a, b, x)`](macro@crate::assert_abs_diff_le) ≈ | a - b | ≤ x
//!
//! * [`assert_abs_diff_gt_x!(a, b, x)`](macro@crate::assert_abs_diff_gt) ≈ | a - b | > x
//!
//! * [`assert_abs_diff_ge_x!(a, b, x)`](macro@crate::assert_abs_diff_ge) ≈ | a - b | ≥ x
//!
//! # Example
//!
//! ```rust
//! use assertables::*;
//!
//! let a = 10;
//! let b = 13;
//! let x = 3;
//! assert_abs_diff_eq_x!(a, b, x);
//! ```

pub mod assert_abs_diff_eq_x;
pub mod assert_abs_diff_ge_x;
pub mod assert_abs_diff_gt_x;
pub mod assert_abs_diff_le_x;
pub mod assert_abs_diff_lt_x;
pub mod assert_abs_diff_ne_x;

pub mod assert_abs_diff_eq; // Deprecated
pub mod assert_abs_diff_ge; // Deprecated
pub mod assert_abs_diff_gt; // Deprecated
pub mod assert_abs_diff_le; // Deprecated
pub mod assert_abs_diff_lt; // Deprecated
pub mod assert_abs_diff_ne; // Deprecated
