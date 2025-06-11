//! Assert for comparing counts.
//!
//! These macros help with collection counts, such as for strings, arrays,
//! vectors, iterators, and anything that has a typical `.count()` method.
//!
//! Compare a count with another count:
//!
//! * [`assert_count_eq!(a, b)`](macro@crate::assert_count_eq) ≈ a.count() = b.count()
//! * [`assert_count_ne!(a, b)`](macro@crate::assert_count_ne) ≈ a.count() ≠ b.count()
//! * [`assert_count_lt!(a, b)`](macro@crate::assert_count_lt) ≈ a.count() < b.count()
//! * [`assert_count_le!(a, b)`](macro@crate::assert_count_le) ≈ a.count() ≤ b.count()
//! * [`assert_count_gt!(a, b)`](macro@crate::assert_count_gt) ≈ a.count() > b.count()
//! * [`assert_count_ge!(a, b)`](macro@crate::assert_count_ge) ≈ a.count() ≥ b.count()
//!
//! Compare a count with an expression:
//!
//! * [`assert_count_eq_x!(a, expr)`](macro@crate::assert_count_eq_x) ≈ a.count() = expr
//! * [`assert_count_ne_x!(a, expr)`](macro@crate::assert_count_ne_x) ≈ a.count() ≠ expr
//! * [`assert_count_lt_x!(a, expr)`](macro@crate::assert_count_lt_x) ≈ a.count() < expr
//! * [`assert_count_le_x!(a, expr)`](macro@crate::assert_count_le_x) ≈ a.count() ≤ expr
//! * [`assert_count_gt_x!(a, expr)`](macro@crate::assert_count_gt_x) ≈ a.count() > expr
//! * [`assert_count_ge_x!(a, expr)`](macro@crate::assert_count_ge_x) ≈ a.count() ≥ expr
//!
//! # Example
//!
//! ```rust
//! use assertables::*;
//!
//! let a = "x".chars();
//! let b = "x".chars();
//! assert_count_eq!(&a, &b);
//! ```

// Compare another
pub mod assert_count_eq;
pub mod assert_count_ge;
pub mod assert_count_gt;
pub mod assert_count_le;
pub mod assert_count_lt;
pub mod assert_count_ne;

// Compare expression
pub mod assert_count_eq_x;
pub mod assert_count_ge_x;
pub mod assert_count_gt_x;
pub mod assert_count_le_x;
pub mod assert_count_lt_x;
pub mod assert_count_ne_x;
