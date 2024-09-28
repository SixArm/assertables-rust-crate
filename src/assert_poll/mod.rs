//! Assert macros for `Poll` {`Ready`, `Pending`}
//!
//! These macros help compare a `Poll` that is either `Ready`, `Pending`.
//!
//! The macros use these capabilities:
//!
//! * implements `.is_ready() -> boolean`
//!
//! * implements `.is_pending() -> boolean`
//!
//! # Macros
//!
//! * [`assert_poll_ready!(a)`](macro@crate::assert_poll_ready) ≈ a is Poll::Ready
//!
//! * [`assert_poll_ready_eq!(a, b)`](macro@crate::assert_poll_ready_eq) ≈ Poll::Ready(a) == Poll::Ready(b)
//!
//! * [`assert_poll_ready_ne!(a, b)`](macro@crate::assert_poll_ready_ne) ≈ Poll::Ready(a) == Poll::Ready(b)
//!
//! * [`assert_poll_pending!(a)`](macro@crate::assert_poll_pending) ≈ a is Poll::Pending
//!
//! # Example
//!
//! ```rust
//! use assertables::*;
//! use std::task::Poll;
//! use std::task::Poll::*;
//!
//!
//! # fn main() {
//! let a: Poll<i8> = Poll::Ready(1);
//! assert_poll_ready!(a);
//! # }
//! ```

pub mod assert_poll_ready;
pub mod assert_poll_ready_eq;
pub mod assert_poll_ready_ne;
pub mod assert_poll_pending;
