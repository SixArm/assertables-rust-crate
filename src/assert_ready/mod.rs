//! Assert macros for Ready(_) items.
//!
//! These macros help compare Ready(…) items, such as `std::Ready::Ready` or similar.
//!
//! Assert expression is Ready(_):
//! 
//! * [`assert_ready!(a)`](macro@crate::assert_ready) 
//!   ≈ a is Ready(_)
//! 
//! Compare Ready(…) to another Ready(…):
//! 
//! * [`assert_ready_eq!(a, b)`](macro@crate::assert_ready_eq) 
//!   ≈ (a ⇒ Ready(a̅) ⇒ a̅) = (b ⇒ Ready(b̅) ⇒ b̅)
//!
//! * [`assert_ready_ne!(a, b)`](macro@crate::assert_ready_ne) 
//!   ≈ (a ⇒ Ready(a̅) ⇒ a̅) ≠ (b ⇒ Ready(b̅) ⇒ b̅)
//!
//! Compare Ready(…) to an expression:
//! 
//! * [`assert_ready_eq_expr!(a, expr)`](macro@crate::assert_ready_eq_expr) 
//!   ≈ (a ⇒ Ready(a̅) ⇒ a̅) = b
//!
//! * [`assert_ready_ne_expr!(a, b)`](macro@crate::assert_ready_ne_expr) 
//!   ≈ (a ⇒ Ready(a̅) ⇒ a̅) ≠ b

// Verify Ready(_)
pub mod assert_ready;

// Compare with another
pub mod assert_ready_eq;
pub mod assert_ready_ne;

// Compare with expression
pub mod assert_ready_eq_expr;
pub mod assert_ready_ne_expr;
