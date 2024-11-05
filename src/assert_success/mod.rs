//! Assert a success method is true.
//!
//! These macros help compare None items, such as `::std::Option::None` or similar.
//!
//! Assert expression is None:
//!
//! * [`assert_success!(a)`](macro@crate::assert_success)
//!   ≈ a.success() is true
//!
//! # Example
//!
//! ```rust
//! use assertables::*;
//!
//! #[derive(Debug)]
//! struct A;
//! impl A { fn success(&self) -> bool { true }}
//! let a = A{};
//! assert_success!(a);
//! ```

pub mod assert_success;
pub mod assert_success_false;
