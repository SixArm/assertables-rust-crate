//! Assert for Err(_) items.
//!
//! These macros help compare Err(…) items, such as `std::Result::Err` or similar.
//!
//! Assert expression is Err(_):
//!
//! * [`assert_err!(a)`](macro@crate::assert_err)
//!   ≈ a is Err(_)
//!
//! Compare Err(…) to another Err(…):
//!
//! * [`assert_err_eq!(a, b)`](macro@crate::assert_err_eq)
//!   ≈ (a ⇒ Err(a̅) ⇒ a̅) = (b ⇒ Err(b̅) ⇒ b̅)
//!
//! * [`assert_err_ne!(a, b)`](macro@crate::assert_err_ne)
//!   ≈ (a ⇒ Err(a̅) ⇒ a̅) ≠ (b ⇒ Err(b̅) ⇒ b̅)
//!
//! Compare Err(…) to an expression:
//!
//! * [`assert_err_eq_expr!(a, expr)`](macro@crate::assert_err_eq_expr)
//!   ≈ (a ⇒ Err(a̅) ⇒ a̅) = b
//!
//! * [`assert_err_ne_expr!(a, b)`](macro@crate::assert_err_ne_expr)
//!   ≈ (a ⇒ Err(a̅) ⇒ a̅) ≠ b
//!
//! # Example
//!
//! ```rust
//! use assertables::*;
//!
//! # fn main() {
//! let a: Result<i8, i8> = Err(1);
//! assert_err!(a);
//! # }
//! ```

// Verify Err(_)
pub mod assert_err;

// Compare with another
pub mod assert_err_eq;
pub mod assert_err_ne;

// Compare with expression
pub mod assert_err_eq_expr;
pub mod assert_err_ne_expr;
