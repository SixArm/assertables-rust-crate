//! Assert for Some(_) items.
//!
//! These macros help compare Some(…) items, such as `::std::Option::Some` or similar.
//!
//! Assert expression is Some:
//!
//! * [`assert_some!(a)`](macro@crate::assert_some)
//!   ≈ a is Some
//!
//! Compare Some(…) to another Some(…):
//!
//! * [`assert_some_eq!(a, b)`](macro@crate::assert_some_eq) ≈ (a ⇒ Some(a1) ⇒ a1) = (b ⇒ Some(b1) ⇒ b1)
//! * [`assert_some_ne!(a, b)`](macro@crate::assert_some_ne) ≈ (a ⇒ Some(a1) ⇒ a1) ≠ (b ⇒ Some(b1) ⇒ b1)
//!
//! Compare Some(…) to an expression:
//!
//! * [`assert_some_eq_x!(a, expr)`](macro@crate::assert_some_eq_x) ≈ (a ⇒ Some(a1) ⇒ a1) = expr
//! * [`assert_some_ne_x!(a, expr)`](macro@crate::assert_some_ne_x) ≈ (a ⇒ Some(a1) ⇒ a1) ≠ expr
//!
//! # Example
//!
//! ```rust
//! use assertables::*;
//!
//! let a: Option<i8> = Option::Some(1);
//! assert_some!(a);
//! ```

// Verify Some(_)
pub mod assert_some;

// Compare another
pub mod assert_some_eq;
pub mod assert_some_ne;

// Compare expression
pub mod assert_some_eq_x;
pub mod assert_some_ne_x;
