//! Assert macros for comparing functions that return errors.
//!
//! These macros help compare functions that return results that are errors,
//! such as `std::Result::Err` or similar.
//!
//! The macros use these capabilities:
//!
//! * implements `.is_err() -> boolean`
//!
//! * implements `.unwrap_err() -> comparable`
//!
//! Compare a function Err() with another function Err():
//!
//! * [`assert_fn_err_eq!(function1, function2)`](macro@crate::assert_fn_err_eq) ≈ function1().unwrap_err() = function2().unwrap_err()
//!
//! * [`assert_fn_err_ne!(function1, function2)`](macro@crate::assert_fn_err_ne) ≈ function1().unwrap_err() ≠ function2().unwrap_err()
//!
//! * [`assert_fn_err_ge!(function1, function2)`](macro@crate::assert_fn_err_ge) ≈ function1().unwrap_err() ≥ function2().unwrap_err()
//!
//! * [`assert_fn_err_gt!(function1, function2)`](macro@crate::assert_fn_err_gt) ≈ function1().unwrap_err() > function2().unwrap_err()
//!
//! * [`assert_fn_err_le!(function1, function2)`](macro@crate::assert_fn_err_le) ≈ function1().unwrap_err() ≤ function2().unwrap_err()
//!
//! * [`assert_fn_err_lt!(function1, function2)`](macro@crate::assert_fn_err_lt) ≈ function1().unwrap_err() < function2().unwrap_err()
//!
//! Compare a function Err() with an expression:
//!
//! * [`assert_fn_err_eq!(function, expr)`](macro@crate::assert_fn_err_eq) ≈ function().unwrap_err() = expr
//!
//! * [`assert_fn_err_ne!(function, expr)`](macro@crate::assert_fn_err_ne) ≈ function().unwrap_err() ≠ expr
//!
//! * [`assert_fn_err_ge!(function, expr)`](macro@crate::assert_fn_err_ge) ≈ function().unwrap_err() ≥ expr
//!
//! * [`assert_fn_err_gt!(function, expr)`](macro@crate::assert_fn_err_gt) ≈ function().unwrap_err() > expr
//!
//! * [`assert_fn_err_le!(function, expr)`](macro@crate::assert_fn_err_le) ≈ function().unwrap_err() ≤ expr
//!
//! * [`assert_fn_err_lt!(function, expr)`](macro@crate::assert_fn_err_lt) ≈ function().unwrap_err() < expr
//!
//!
//! # Example
//!
//! ```rust
//! # #[macro_use] extern crate assertables;
//! fn f(i: i8) -> Result<String, String> {
//!     match i {
//!         0..=9 => Ok(format!("{}", i)),
//!         _ => Err(format!("{:?} is out of range", i)),
//!     }
//! }
//!
//! # fn main() {
//! let a: i8 = 10;
//! let b: i8 = 10;
//! assert_fn_err_eq!(f, a, f, b);
//! # }
//! ```

// Comparisons
pub mod assert_fn_err_eq;
pub mod assert_fn_err_ne;
pub mod assert_fn_err_lt;
pub mod assert_fn_err_le;
pub mod assert_fn_err_gt;
pub mod assert_fn_err_ge;

// Comparisons with expressions
pub mod assert_fn_err_eq_expr;
pub mod assert_fn_err_ne_expr;
pub mod assert_fn_err_lt_expr;
pub mod assert_fn_err_le_expr;
pub mod assert_fn_err_gt_expr;
pub mod assert_fn_err_ge_expr;
