//! Assert for comparing lengths.
//!
//! These macros help with collection lengths, such as for strings, arrays,
//! vectors, iterators, and anything that has a typical `.len()` method.
//!
//! Compare a length with another length:
//!
//! * [`assert_len_eq!(a, b)`](macro@crate::assert_len_eq) ≈ a.len() = b.len()
//! * [`assert_len_ne!(a, b)`](macro@crate::assert_len_ne) ≈ a.len() ≠ b.len()
//! * [`assert_len_lt!(a, b)`](macro@crate::assert_len_lt) ≈ a.len() < b.len()
//! * [`assert_len_le!(a, b)`](macro@crate::assert_len_le) ≈ a.len() ≤ b.len()
//! * [`assert_len_gt!(a, b)`](macro@crate::assert_len_gt) ≈ a.len() > b.len()
//! * [`assert_len_ge!(a, b)`](macro@crate::assert_len_ge) ≈ a.len() ≥ b.len()
//!
//! Compare a length with an expression:
//!
//! * [`assert_len_eq_x!(a, expr)`](macro@crate::assert_len_eq_x) ≈ a.len() = expr
//! * [`assert_len_ne_x!(a, expr)`](macro@crate::assert_len_ne_x) ≈ a.len() ≠ expr
//! * [`assert_len_lt_x!(a, expr)`](macro@crate::assert_len_lt_x) ≈ a.len() < expr
//! * [`assert_len_le_x!(a, expr)`](macro@crate::assert_len_le_x) ≈ a.len() ≤ expr
//! * [`assert_len_gt_x!(a, expr)`](macro@crate::assert_len_gt_x) ≈ a.len() > expr
//! * [`assert_len_ge_x!(a, expr)`](macro@crate::assert_len_ge_x) ≈ a.len() ≥ expr
//!
//! # Example
//!
//! ```rust
//! use assertables::*;
//!
//! let a = "x";
//! let b = "x";
//! assert_len_eq!(a, b);
//! ```

// Compare another
pub mod assert_len_eq;
pub mod assert_len_ge;
pub mod assert_len_gt;
pub mod assert_len_le;
pub mod assert_len_lt;
pub mod assert_len_ne;

// Compare expression
pub mod assert_len_eq_x;
pub mod assert_len_ge_x;
pub mod assert_len_gt_x;
pub mod assert_len_le_x;
pub mod assert_len_lt_x;
pub mod assert_len_ne_x;
