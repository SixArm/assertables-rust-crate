//! Assert for comparing functions that return Result::Ok.
//!
//! These macros help compare functions that return results that are ok, such as
//! `::std::Result::Ok` or similar.
//!
//! The macros use these capabilities:
//!
//! * implements `.is_ok() -> boolean`
//!
//! * implements `.unwrap_ok() -> comparable`
//!
//! Compare a function Ok() with another function Ok():
//!
//! * [`assert_fn_ok_eq2!(a_function, b_function)`](macro@crate::assert_fn_ok_eq2) ≈ a_function().unwrap_err() = b_function().unwrap_err()
//!
//! * [`assert_fn_ok_ne2!(a_function, b_function)`](macro@crate::assert_fn_ok_ne2) ≈ a_function().unwrap_err() ≠ b_function().unwrap_err()
//!
//! * [`assert_fn_ok_ge2!(a_function, b_function)`](macro@crate::assert_fn_ok_ge2) ≈ a_function().unwrap_err() ≥ b_function().unwrap_err()
//!
//! * [`assert_fn_ok_gt2!(a_function, b_function)`](macro@crate::assert_fn_ok_gt2) ≈ a_function().unwrap_err() > b_function().unwrap_err()
//!
//! * [`assert_fn_ok_le2!(a_function, b_function)`](macro@crate::assert_fn_ok_le2) ≈ a_function().unwrap_err() ≤ b_function().unwrap_err()
//!
//! * [`assert_fn_ok_lt2!(a_function, b_function)`](macro@crate::assert_fn_ok_lt2) ≈ a_function().unwrap_err() < b_function().unwrap_err()
//!
//! Compare a function Ok() with an expression:
//!
//! * [`assert_fn_ok_eq2!(function, expr)`](macro@crate::assert_fn_ok_eq2) ≈ function().unwrap_err() = expr
//!
//! * [`assert_fn_ok_ne2!(function, expr)`](macro@crate::assert_fn_ok_ne2) ≈ function().unwrap_err() ≠ expr
//!
//! * [`assert_fn_ok_ge2!(function, expr)`](macro@crate::assert_fn_ok_ge2) ≈ function().unwrap_err() ≥ expr
//!
//! * [`assert_fn_ok_gt2!(function, expr)`](macro@crate::assert_fn_ok_gt2) ≈ function().unwrap_err() > expr
//!
//! * [`assert_fn_ok_le2!(function, expr)`](macro@crate::assert_fn_ok_le2) ≈ function().unwrap_err() ≤ expr
//!
//! * [`assert_fn_ok_lt2!(function, expr)`](macro@crate::assert_fn_ok_lt2) ≈ function().unwrap_err() < expr
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
//! assert_fn_ok_eq2!(f, a, f, b);
//! # }
//! ```

// Comparisons
pub mod assert_fn_ok_eq2;
pub mod assert_fn_ok_ge2;
pub mod assert_fn_ok_gt2;
pub mod assert_fn_ok_le2;
pub mod assert_fn_ok_lt2;
pub mod assert_fn_ok_ne2;

// Compare with expression
pub mod assert_fn_ok_eq;
pub mod assert_fn_ok_ge;
pub mod assert_fn_ok_gt;
pub mod assert_fn_ok_le;
pub mod assert_fn_ok_lt;
pub mod assert_fn_ok_ne;
