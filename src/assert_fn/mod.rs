//! Assert for comparing functions.
//!
//! These macros help compare functions that return anything.
//! The macros call the functions, then compare the return values.
//!
//! Compare a function with another function:
//!
//! * [`assert_fn_eq2!(a_function, b_function)`](macro@crate::assert_fn_eq2) ≈ a_function() = b_function()
//!
//! * [`assert_fn_ne2!(a_function, b_function)`](macro@crate::assert_fn_ne2) ≈ a_function() ≠ b_function()
//!
//! * [`assert_fn_ge2!(a_function, b_function)`](macro@crate::assert_fn_ge2) ≈ a_function() ≥ b_function()
//!
//! * [`assert_fn_gt2!(a_function, b_function)`](macro@crate::assert_fn_gt2) ≈ a_function() > b_function()
//!
//! * [`assert_fn_le2!(a_function, b_function)`](macro@crate::assert_fn_le2) ≈ a_function() ≤ b_function()
//!
//! * [`assert_fn_lt2!(a_function, b_function)`](macro@crate::assert_fn_lt2) ≈ a_function() < b_function()
//!
//! Compare a function with an expression:
//!
//! * [`assert_fn_eq!(function, expr)`](macro@crate::assert_fn_eq) ≈ function() = expr
//!
//! * [`assert_fn_ne!(function, expr)`](macro@crate::assert_fn_ne) ≈ function() ≠ expr
//!
//! * [`assert_fn_ge!(function, expr)`](macro@crate::assert_fn_ge) ≈ function() ≥ expr
//!
//! * [`assert_fn_gt!(function, expr)`](macro@crate::assert_fn_gt) ≈ function() > expr
//!
//! * [`assert_fn_le!(function, expr)`](macro@crate::assert_fn_le) ≈ function() ≤ expr
//!
//! * [`assert_fn_lt!(function, expr)`](macro@crate::assert_fn_lt) ≈ function() < expr
//!
//!
//! # Example
//!
//! ```rust
//! use assertables::*;
//!
//! # fn main() {
//! let a: i8 = -1;
//! let b: i8 = 1;
//! assert_fn_eq2!(i8::abs, a, i8::abs, b);
//! # }
//! ```

// Comparisons
pub mod assert_fn_eq2;
pub mod assert_fn_ge2;
pub mod assert_fn_gt2;
pub mod assert_fn_le2;
pub mod assert_fn_lt2;
pub mod assert_fn_ne2;

// Compare with expression
pub mod assert_fn_eq;
pub mod assert_fn_ge;
pub mod assert_fn_gt;
pub mod assert_fn_le;
pub mod assert_fn_lt;
pub mod assert_fn_ne;
