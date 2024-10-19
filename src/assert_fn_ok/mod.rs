//! Assert for comparing functions that return Result::Ok.
//!
//! These macros help compare functions that return results that are ok, such as
//! `std::Result::Ok` or similar.
//!
//! The macros use these capabilities:
//!
//! * implements `.is_ok() -> boolean`
//!
//! * implements `.unwrap_ok() -> comparable`
//!
//! Compare a function Ok() with another function Ok():
//!
//! * [`assert_fn_ok_eq!(function1, function2)`](macro@crate::assert_fn_ok_eq) ≈ function1().unwrap_err() = function2().unwrap_err()
//!
//! * [`assert_fn_ok_ne!(function1, function2)`](macro@crate::assert_fn_ok_ne) ≈ function1().unwrap_err() ≠ function2().unwrap_err()
//!
//! * [`assert_fn_ok_ge!(function1, function2)`](macro@crate::assert_fn_ok_ge) ≈ function1().unwrap_err() ≥ function2().unwrap_err()
//!
//! * [`assert_fn_ok_gt!(function1, function2)`](macro@crate::assert_fn_ok_gt) ≈ function1().unwrap_err() > function2().unwrap_err()
//!
//! * [`assert_fn_ok_le!(function1, function2)`](macro@crate::assert_fn_ok_le) ≈ function1().unwrap_err() ≤ function2().unwrap_err()
//!
//! * [`assert_fn_ok_lt!(function1, function2)`](macro@crate::assert_fn_ok_lt) ≈ function1().unwrap_err() < function2().unwrap_err()
//!
//! Compare a function Ok() with an expression:
//!
//! * [`assert_fn_ok_eq!(function, expr)`](macro@crate::assert_fn_ok_eq) ≈ function().unwrap_err() = expr
//!
//! * [`assert_fn_ok_ne!(function, expr)`](macro@crate::assert_fn_ok_ne) ≈ function().unwrap_err() ≠ expr
//!
//! * [`assert_fn_ok_ge!(function, expr)`](macro@crate::assert_fn_ok_ge) ≈ function().unwrap_err() ≥ expr
//!
//! * [`assert_fn_ok_gt!(function, expr)`](macro@crate::assert_fn_ok_gt) ≈ function().unwrap_err() > expr
//!
//! * [`assert_fn_ok_le!(function, expr)`](macro@crate::assert_fn_ok_le) ≈ function().unwrap_err() ≤ expr
//!
//! * [`assert_fn_ok_lt!(function, expr)`](macro@crate::assert_fn_ok_lt) ≈ function().unwrap_err() < expr
//!
//! # Example
//!
//! ```rust
//! use assertables::*;
//! fn f(i: i8) -> Result<String, String> {
//!     match i {
//!         0..=9 => Ok(format!("{}", i)),
//!         _ => Err(format!("{:?} is out of range", i)),
//!     }
//! }
//!
//! # fn main() {
//! let a: i8 = 1;
//! let b: i8 = 1;
//! assert_fn_ok_eq!(f, a, f, b);
//! # }
//! ```

// Comparisons
pub mod assert_fn_ok_eq;
pub mod assert_fn_ok_ge;
pub mod assert_fn_ok_gt;
pub mod assert_fn_ok_le;
pub mod assert_fn_ok_lt;
pub mod assert_fn_ok_ne;

// Compare with expression
pub mod assert_fn_ok_eq_expr;
pub mod assert_fn_ok_ge_expr;
pub mod assert_fn_ok_gt_expr;
pub mod assert_fn_ok_le_expr;
pub mod assert_fn_ok_lt_expr;
pub mod assert_fn_ok_ne_expr;
