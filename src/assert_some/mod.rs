//! Assert macros for Some(_) items.
//!
//! These macros help compare Some(…) items, such as `std::Option::Some` or similar.
//!
//! Assert expression is Some(_):
//! 
//! * [`assert_some!(a)`](macro@crate::assert_some) 
//!   ≈ a is Some(_)
//! 
//! Compare Some(…) to another Some(…):
//! 
//! * [`assert_some_eq!(a, b)`](macro@crate::assert_some_eq) 
//!   ≈ (a ⇒ Some(a̅) ⇒ a̅) = (b ⇒ Some(b̅) ⇒ b̅)
//!
//! * [`assert_some_ne!(a, b)`](macro@crate::assert_some_ne) 
//!   ≈ (a ⇒ Some(a̅) ⇒ a̅) ≠ (b ⇒ Some(b̅) ⇒ b̅)
//!
//! Compare Some(…) to an expression:
//! 
//! * [`assert_some_eq_expr!(a, expr)`](macro@crate::assert_some_eq_expr) 
//!   ≈ (a ⇒ Some(a̅) ⇒ a̅) = b
//!
//! * [`assert_some_ne_expr!(a, b)`](macro@crate::assert_some_ne_expr) 
//!   ≈ (a ⇒ Some(a̅) ⇒ a̅) ≠ b

// Verify Some(_)
pub mod assert_some;

// Compare with another
pub mod assert_some_eq;
pub mod assert_some_ne;

// Compare with expression
pub mod assert_some_eq_expr;
pub mod assert_some_ne_expr;
