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
//! * [`assert_ready_eq2!(a, b)`](macro@crate::assert_ready_eq2)
//!   ≈ (a ⇒ Ready(a1) ⇒ a1) = (b ⇒ Ready(b1) ⇒ b1)
//!
//! * [`assert_ready_ne2!(a, b)`](macro@crate::assert_ready_ne2)
//!   ≈ (a ⇒ Ready(a1) ⇒ a1) ≠ (b ⇒ Ready(b1) ⇒ b1)
//!
//! Compare Ready(…) to an expression:
//!
//! * [`assert_ready_eq!(a, expr)`](macro@crate::assert_ready_eq)
//!   ≈ (a ⇒ Ready(a1) ⇒ a1) = b
//!
//! * [`assert_ready_ne!(a, b)`](macro@crate::assert_ready_ne)
//!   ≈ (a ⇒ Ready(a1) ⇒ a1) ≠ b
//!
//! # Example
//!
//! ```rust
//! use assertables::*;
//! use std::task::Poll;
//! use std::task::Poll::*;
//!
//! # fn main() {
//! let a: Poll<i8> = Ready(1);
//! let b: Poll<i8> = Ready(1);
//! assert_ready_eq2!(a, b);
//! # }
//! ```

// Verify Ready(_)
pub mod assert_ready;

// Compare with another
pub mod assert_ready_eq2;
pub mod assert_ready_ne2;

// Compare with expression
pub mod assert_ready_eq;
pub mod assert_ready_ne;
