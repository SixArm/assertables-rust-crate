//! Assert for Pending items.
//!
//! These macros help compare Pending items, such as `::std::Poll::Pending` or similar.
//!
//! Assert expression is Pending:
//!
//! * [`assert_pending!(a)`](macro@crate::assert_pending)
//!   ≈ a is Pending
//!
//! # Example
//!
//! ```rust
//! use assertables::*;
//! use std::task::Poll;
//! use std::task::Poll::*;
//!
//! let a: Poll<i8> = Pending;
//! assert_pending!(a);
//! ```

pub mod assert_pending;
