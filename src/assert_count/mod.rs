//! Assert for comparing counts.
//!
//! These macros help with collection counts, such as for strings, arrays, 
//! vectors, iterators, and anything that has a typical `.count()` method.
//!
//! Compare a count with another count:
//!
//! * [`assert_count_eq!(a, b)`](macro@crate::assert_count_eq) ≈ a.count() = b.count() 
//!
//! * [`assert_count_ne!(a, b)`](macro@crate::assert_count_ne) ≈ a.count() ≠ b.count() 
//!
//! * [`assert_count_lt!(a, b)`](macro@crate::assert_count_lt) ≈ a.count() < b.count() 
//!
//! * [`assert_count_le!(a, b)`](macro@crate::assert_count_le) ≈ a.count() ≤ b.count() 
//!
//! * [`assert_count_gt!(a, b)`](macro@crate::assert_count_gt) ≈ a.count() > b.count() 
//! 
//! * [`assert_count_ge!(a, b)`](macro@crate::assert_count_ge) ≈ a.count() ≥ b.count() 
//!
//! Compare a count with an expression:
//!
//! * [`assert_count_eq_expr!(a, expr)`](macro@crate::assert_count_eq_expr) ≈ a.count() = expr
//!
//! * [`assert_count_ne_expr!(a, expr)`](macro@crate::assert_count_ne_expr) ≈ a.count() ≠ expr
//!
//! * [`assert_count_lt_expr!(a, expr)`](macro@crate::assert_count_lt_expr) ≈ a.count() < expr
//!
//! * [`assert_count_le_expr!(a, expr)`](macro@crate::assert_count_le_expr) ≈ a.count() ≤ expr
//!
//! * [`assert_count_gt_expr!(a, expr)`](macro@crate::assert_count_gt_expr) ≈ a.count() > expr
//! 
//! * [`assert_count_ge_expr!(a, expr)`](macro@crate::assert_count_ge_expr) ≈ a.count() ≥ expr
//! 
//! # Example
//!
//! ```rust
//! use assertables::*;
//! 
//! # fn main() {
//! let a = "x".chars();
//! let b = "x".chars();
//! assert_count_eq!(a, b);
//! # }
//! ```

// Compare another
pub mod assert_count_eq;
pub mod assert_count_ne;
pub mod assert_count_lt;
pub mod assert_count_le;
pub mod assert_count_gt;
pub mod assert_count_ge;

// Compare expression
pub mod assert_count_eq_expr;
pub mod assert_count_ne_expr;
pub mod assert_count_lt_expr;
pub mod assert_count_le_expr;
pub mod assert_count_gt_expr;
pub mod assert_count_ge_expr;
