//! Assert for comparing lengths.
//!
//! These macros help with collection lengths, such as for strings, arrays,
//! vectors, iterators, and anything that has a typical `.len()` method.
//!
//! Compare a length with another length:
//!
//! * [`assert_len_eq2!(a, b)`](macro@crate::assert_len_eq2) ≈ a.len() = b.len()
//!
//! * [`assert_len_ne2!(a, b)`](macro@crate::assert_len_ne2) ≈ a.len() ≠ b.len()
//!
//! * [`assert_len_lt2!(a, b)`](macro@crate::assert_len_lt2) ≈ a.len() < b.len()
//!
//! * [`assert_len_le2!(a, b)`](macro@crate::assert_len_le2) ≈ a.len() ≤ b.len()
//!
//! * [`assert_len_gt2!(a, b)`](macro@crate::assert_len_gt2) ≈ a.len() > b.len()
//!
//! * [`assert_len_ge2!(a, b)`](macro@crate::assert_len_ge2) ≈ a.len() ≥ b.len()
//!
//! Compare a length with an expression:
//!
//! * [`assert_len_eq!(a, expr)`](macro@crate::assert_len_eq) ≈ a.len() = expr
//!
//! * [`assert_len_ne!(a, expr)`](macro@crate::assert_len_ne) ≈ a.len() ≠ expr
//!
//! * [`assert_len_lt!(a, expr)`](macro@crate::assert_len_lt) ≈ a.len() < expr
//!
//! * [`assert_len_le!(a, expr)`](macro@crate::assert_len_le) ≈ a.len() ≤ expr
//!
//! * [`assert_len_gt!(a, expr)`](macro@crate::assert_len_gt) ≈ a.len() > expr
//!
//! * [`assert_len_ge!(a, expr)`](macro@crate::assert_len_ge) ≈ a.len() ≥ expr
//!
//! # Example
//!
//! ```rust
//! use assertables::*;
//!
//! # fn main() {
//! let a = "x";
//! let b = "x";
//! assert_len_eq2!(a, b);
//! # }
//! ```

// Compare another
pub mod assert_len_eq2;
pub mod assert_len_ge2;
pub mod assert_len_gt2;
pub mod assert_len_le2;
pub mod assert_len_lt2;
pub mod assert_len_ne2;

// Compare expression
pub mod assert_len_eq;
pub mod assert_len_ge;
pub mod assert_len_gt;
pub mod assert_len_le;
pub mod assert_len_lt;
pub mod assert_len_ne;
