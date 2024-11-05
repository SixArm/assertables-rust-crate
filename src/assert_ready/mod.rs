//! Assert for Ready(_) items.
//!
//! These macros help compare Ready(…) items, such as `::std::Ready::Ready` or similar.
//!
//! Assert expression is Ready:
//!
//! * [`assert_ready!(a)`](macro@crate::assert_ready)
//!   ≈ a is Ready
//!
//! Compare Ready(…) to another Ready(…):
//!
//! * [`assert_ready_eq!(a, b)`](macro@crate::assert_ready_eq) ≈ (a ⇒ Ready(a1) ⇒ a1) = (b ⇒ Ready(b1) ⇒ b1)
//! * [`assert_ready_ne!(a, b)`](macro@crate::assert_ready_ne) ≈ (a ⇒ Ready(a1) ⇒ a1) ≠ (b ⇒ Ready(b1) ⇒ b1)
//!
//! Compare Ready(…) to an expression:
//!
//! * [`assert_ready_eq_x!(a, expr)`](macro@crate::assert_ready_eq_x) ≈ (a ⇒ Ready(a1) ⇒ a1) = expr
//! * [`assert_ready_ne_x!(a, expr)`](macro@crate::assert_ready_ne_x) ≈ (a ⇒ Ready(a1) ⇒ a1) ≠ expr
//!
//! # Example
//!
//! ```rust
//! use assertables::*;
//! use std::task::Poll;
//! use std::task::Poll::*;
//!
//! let a: Poll<i8> = Ready(1);
//! let b: Poll<i8> = Ready(1);
//! assert_ready_eq!(a, b);
//! ```

// Verify Ready(_)
pub mod assert_ready;

// Compare another
pub mod assert_ready_eq;
pub mod assert_ready_ne;

// Compare expression
pub mod assert_ready_eq_x;
pub mod assert_ready_ne_x;
