//! Assert for Ok(…) items.
//!
//! These macros help compare Ok(…) items, such as `::std::Result::Ok` or similar.
//!
//! Assert expression is Ok:
//!
//! * [`assert_ok!(a)`](macro@crate::assert_ok)
//!   ≈ a is Ok.
//!
//! Compare Ok(…) to another Ok(…):
//!
//! * [`assert_ok_eq!(a, b)`](macro@crate::assert_ok_eq) ≈ (a ⇒ Ok(a1) ⇒ a1) = (b ⇒ Ok(b1) ⇒ b1)
//! * [`assert_ok_ne!(a, b)`](macro@crate::assert_ok_ne) ≈ (a ⇒ Ok(a1) ⇒ a1) ≠ (b ⇒ Ok(b1) ⇒ b1)
//!
//! Compare Ok(…) to an expression:
//!
//! * [`assert_ok_eq_x!(a, expr)`](macro@crate::assert_ok_eq_x) ≈ (a ⇒ Ok(a1) ⇒ a1) = expr
//! * [`assert_ok_ne_x!(a, expr)`](macro@crate::assert_ok_ne_x) ≈ (a ⇒ Ok(a1) ⇒ a1) ≠ expr
//!
//! # Example
//!
//! ```rust
//! use assertables::*;
//!
//! # fn main() {
//! let a: Result<i8, i8> = Ok(1);
//! assert_ok!(a);
//! # }
//! ```

// Verify Ok(_)
pub mod assert_ok;

// Compare another
pub mod assert_ok_eq;
pub mod assert_ok_ne;

// Compare expression
pub mod assert_ok_eq_x;
pub mod assert_ok_ne_x;
