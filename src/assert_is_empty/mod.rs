//! Assert for method is_empty().
//!
//! These macros help with any item that implements self.is_empty().
//!
//! * [`assert_is_empty!(collection)`](macro@crate::assert_is_empty) ≈ collection.is_empty()
//!
//! * [`assert_not_empty!(collection)`](macro@crate::assert_not_empty) ≈ !collection.is_empty()
//!
//! # Example
//!
//! ```rust
//! use assertables::*;
//!
//! let a = "";
//! assert_is_empty!(a);
//! ```

pub mod assert_is_empty;
pub mod assert_not_empty;
