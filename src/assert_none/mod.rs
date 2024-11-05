//! Assert for None items.
//!
//! These macros help compare None items, such as `::std::Option::None` or similar.
//!
//! Assert expression is None:
//!
//! * [`assert_none!(a)`](macro@crate::assert_none)
//!   ≈ a is None
//!
//! # Example
//!
//! ```rust
//! use assertables::*;
//!
//! let a: Option<i8> = Option::None;
//! assert_none!(a);
//! ```

pub mod assert_none;
