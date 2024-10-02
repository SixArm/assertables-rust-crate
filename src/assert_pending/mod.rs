//! Assert macro for Pending items.
//!
//! These macros help compare Pending items, such as `std::Poll::Pending` or similar.
//!
//! Assert expression is Pending:
//!
//! * [`assert_pending!(a)`](macro@crate::assert_pending)
//!   ≈ a is Pending

pub mod assert_pending;
