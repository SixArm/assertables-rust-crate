//! Assert for comparing functions that return Result::Ok.
//!
//! These macros help compare functions that return results that are ok, such as
//! `::std::Result::Ok` or similar.
//!
//! The macros use these capabilities:
//!
//! * implements `.is_ok() -> bool`
//!
//! * implements `.unwrap_ok() -> comparable`
//!
//! Compare a function Ok() with another function Ok():
//!
//! * [`assert_fn_ok_eq!(a_function, b_function)`](macro@crate::assert_fn_ok_eq) ≈ a_function().unwrap_err() = b_function().unwrap_err()
//! * [`assert_fn_ok_ne!(a_function, b_function)`](macro@crate::assert_fn_ok_ne) ≈ a_function().unwrap_err() ≠ b_function().unwrap_err()
//! * [`assert_fn_ok_ge!(a_function, b_function)`](macro@crate::assert_fn_ok_ge) ≈ a_function().unwrap_err() ≥ b_function().unwrap_err()
//! * [`assert_fn_ok_gt!(a_function, b_function)`](macro@crate::assert_fn_ok_gt) ≈ a_function().unwrap_err() > b_function().unwrap_err()
//! * [`assert_fn_ok_le!(a_function, b_function)`](macro@crate::assert_fn_ok_le) ≈ a_function().unwrap_err() ≤ b_function().unwrap_err()
//! * [`assert_fn_ok_lt!(a_function, b_function)`](macro@crate::assert_fn_ok_lt) ≈ a_function().unwrap_err() < b_function().unwrap_err()
//!
//! Compare a function Ok() with an expression:
//!
//! * [`assert_fn_ok_eq_x!(function, expr)`](macro@crate::assert_fn_ok_eq_x) ≈ function().unwrap_err() = expr
//! * [`assert_fn_ok_ne_x!(function, expr)`](macro@crate::assert_fn_ok_ne_x) ≈ function().unwrap_err() ≠ expr
//! * [`assert_fn_ok_ge_x!(function, expr)`](macro@crate::assert_fn_ok_ge_x) ≈ function().unwrap_err() ≥ expr
//! * [`assert_fn_ok_gt_x!(function, expr)`](macro@crate::assert_fn_ok_gt_x) ≈ function().unwrap_err() > expr
//! * [`assert_fn_ok_le_x!(function, expr)`](macro@crate::assert_fn_ok_le_x) ≈ function().unwrap_err() ≤ expr
//! * [`assert_fn_ok_lt_x!(function, expr)`](macro@crate::assert_fn_ok_lt_x) ≈ function().unwrap_err() < expr
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

// Compare another
pub mod assert_fn_ok_eq;
pub mod assert_fn_ok_ge;
pub mod assert_fn_ok_gt;
pub mod assert_fn_ok_le;
pub mod assert_fn_ok_lt;
pub mod assert_fn_ok_ne;

// Compare expression
pub mod assert_fn_ok_eq_x;
pub mod assert_fn_ok_ge_x;
pub mod assert_fn_ok_gt_x;
pub mod assert_fn_ok_le_x;
pub mod assert_fn_ok_lt_x;
pub mod assert_fn_ok_ne_x;
