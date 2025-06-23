//! Assert for comparing functions.
//!
//! These macros help compare functions that return anything.
//! The macros call the functions, then compare the return values.
//!
//! Compare a function with another function:
//!
//! * [`assert_fn_eq!(a_function, b_function)`](macro@crate::assert_fn_eq) ≈ a_function() = b_function()
//! * [`assert_fn_ne!(a_function, b_function)`](macro@crate::assert_fn_ne) ≈ a_function() ≠ b_function()
//! * [`assert_fn_ge!(a_function, b_function)`](macro@crate::assert_fn_ge) ≈ a_function() ≥ b_function()
//! * [`assert_fn_gt!(a_function, b_function)`](macro@crate::assert_fn_gt) ≈ a_function() > b_function()
//! * [`assert_fn_le!(a_function, b_function)`](macro@crate::assert_fn_le) ≈ a_function() ≤ b_function()
//! * [`assert_fn_lt!(a_function, b_function)`](macro@crate::assert_fn_lt) ≈ a_function() < b_function()
//!
//! Compare a function with an expression:
//!
//! * [`assert_fn_eq_x!(function, expr)`](macro@crate::assert_fn_eq_x) ≈ function() = expr
//! * [`assert_fn_ne_x!(function, expr)`](macro@crate::assert_fn_ne_x) ≈ function() ≠ expr
//! * [`assert_fn_ge_x!(function, expr)`](macro@crate::assert_fn_ge_x) ≈ function() ≥ expr
//! * [`assert_fn_gt_x!(function, expr)`](macro@crate::assert_fn_gt_x) ≈ function() > expr
//! * [`assert_fn_le_x!(function, expr)`](macro@crate::assert_fn_le_x) ≈ function() ≤ expr
//! * [`assert_fn_lt_x!(function, expr)`](macro@crate::assert_fn_lt_x) ≈ function() < expr
//!
//! # Example
//!
//! ```rust
//! use assertables::*;
//!
//! let a: i8 = -1;
//! let b: i8 = 1;
//! assert_fn_eq!(i8::abs, a, i8::abs, b);
//! ```

// Compare another
pub mod assert_fn_eq;
pub mod assert_fn_ge;
pub mod assert_fn_gt;
pub mod assert_fn_le;
pub mod assert_fn_lt;
pub mod assert_fn_ne;

// Compare expression
pub mod assert_fn_eq_x;
pub mod assert_fn_ge_x;
pub mod assert_fn_gt_x;
pub mod assert_fn_le_x;
pub mod assert_fn_lt_x;
pub mod assert_fn_ne_x;
