//! Assert for comparing functions that return errors.
//!
//! These macros help compare functions that return results that are errors,
//! such as `::std::Result::Err` or similar.
//!
//! The macros use these capabilities:
//!
//! * implements `.is_err() -> bool`
//!
//! * implements `.unwrap_err() -> comparable`
//!
//! Compare a function Err() with another function Err():
//!
//! * [`assert_fn_err_eq!(a_function, b_function)`](macro@crate::assert_fn_err_eq) ≈ a_function().unwrap_err() = b_function().unwrap_err()
//! * [`assert_fn_err_ne!(a_function, b_function)`](macro@crate::assert_fn_err_ne) ≈ a_function().unwrap_err() ≠ b_function().unwrap_err()
//! * [`assert_fn_err_ge!(a_function, b_function)`](macro@crate::assert_fn_err_ge) ≈ a_function().unwrap_err() ≥ b_function().unwrap_err()
//! * [`assert_fn_err_gt!(a_function, b_function)`](macro@crate::assert_fn_err_gt) ≈ a_function().unwrap_err() > b_function().unwrap_err()
//! * [`assert_fn_err_le!(a_function, b_function)`](macro@crate::assert_fn_err_le) ≈ a_function().unwrap_err() ≤ b_function().unwrap_err()
//! * [`assert_fn_err_lt!(a_function, b_function)`](macro@crate::assert_fn_err_lt) ≈ a_function().unwrap_err() < b_function().unwrap_err()
//!
//! Compare a function Err() with an expression:
//!
//! * [`assert_fn_err_eq_x!(function, expr)`](macro@crate::assert_fn_err_eq_x) ≈ function().unwrap_err() = expr
//! * [`assert_fn_err_ne_x!(function, expr)`](macro@crate::assert_fn_err_ne_x) ≈ function().unwrap_err() ≠ expr
//! * [`assert_fn_err_ge_x!(function, expr)`](macro@crate::assert_fn_err_ge_x) ≈ function().unwrap_err() ≥ expr
//! * [`assert_fn_err_gt_x!(function, expr)`](macro@crate::assert_fn_err_gt_x) ≈ function().unwrap_err() > expr
//! * [`assert_fn_err_le_x!(function, expr)`](macro@crate::assert_fn_err_le_x) ≈ function().unwrap_err() ≤ expr
//! * [`assert_fn_err_lt_x!(function, expr)`](macro@crate::assert_fn_err_lt_x) ≈ function().unwrap_err() < expr
//!
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
//! let a: i8 = 10;
//! let b: i8 = 10;
//! assert_fn_err_eq!(f, a, f, b);
//! ```

// Compare another
pub mod assert_fn_err_eq;
pub mod assert_fn_err_ge;
pub mod assert_fn_err_gt;
pub mod assert_fn_err_le;
pub mod assert_fn_err_lt;
pub mod assert_fn_err_ne;

// Compare expression
pub mod assert_fn_err_eq_x;
pub mod assert_fn_err_ge_x;
pub mod assert_fn_err_gt_x;
pub mod assert_fn_err_le_x;
pub mod assert_fn_err_lt_x;
pub mod assert_fn_err_ne_x;
