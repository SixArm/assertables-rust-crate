//! Assert for comparing counts.
//!
//! These macros help with collection counts, such as for strings, arrays,
//! vectors, iterators, and anything that has a typical `.count()` method.
//!
//! Compare a count with another count:
//!
//! * [`assert_count_eq2!(a, b)`](macro@crate::assert_count_eq2) ≈ a.count() = b.count()
//!
//! * [`assert_count_ne2!(a, b)`](macro@crate::assert_count_ne2) ≈ a.count() ≠ b.count()
//!
//! * [`assert_count_lt2!(a, b)`](macro@crate::assert_count_lt2) ≈ a.count() < b.count()
//!
//! * [`assert_count_le2!(a, b)`](macro@crate::assert_count_le2) ≈ a.count() ≤ b.count()
//!
//! * [`assert_count_gt2!(a, b)`](macro@crate::assert_count_gt2) ≈ a.count() > b.count()
//!
//! * [`assert_count_ge2!(a, b)`](macro@crate::assert_count_ge2) ≈ a.count() ≥ b.count()
//!
//! Compare a count with an expression:
//!
//! * [`assert_count_eq!(a, expr)`](macro@crate::assert_count_eq) ≈ a.count() = expr
//!
//! * [`assert_count_ne!(a, expr)`](macro@crate::assert_count_ne) ≈ a.count() ≠ expr
//!
//! * [`assert_count_lt!(a, expr)`](macro@crate::assert_count_lt) ≈ a.count() < expr
//!
//! * [`assert_count_le!(a, expr)`](macro@crate::assert_count_le) ≈ a.count() ≤ expr
//!
//! * [`assert_count_gt!(a, expr)`](macro@crate::assert_count_gt) ≈ a.count() > expr
//!
//! * [`assert_count_ge!(a, expr)`](macro@crate::assert_count_ge) ≈ a.count() ≥ expr
//!
//! # Example
//!
//! ```rust
//! use assertables::*;
//!
//! # fn main() {
//! let a = "x".chars();
//! let b = "x".chars();
//! assert_count_eq2!(a, b);
//! # }
//! ```

// Compare another
pub mod assert_count_eq2;
pub mod assert_count_ge2;
pub mod assert_count_gt2;
pub mod assert_count_le2;
pub mod assert_count_lt2;
pub mod assert_count_ne2;

// Compare expression
pub mod assert_count_eq;
pub mod assert_count_ge;
pub mod assert_count_gt;
pub mod assert_count_le;
pub mod assert_count_lt;
pub mod assert_count_ne;
