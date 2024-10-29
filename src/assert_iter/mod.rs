//! Assert for comparing iter collections.
//!
//! These macros help with comparison of iter parameters, such as two arrays or
//! two vectors. These macros convert each input using the std::iter::Iterator trait.
//!
//! * [`assert_iter_eq!(collection1, collection2)`](macro@crate::assert_iter_eq) ≈ iter a = iter b
//! * [`assert_iter_ne!(collection1, collection2)`](macro@crate::assert_iter_ne) ≈ iter a ≠ iter b
//! * [`assert_iter_lt!(collection1, collection2)`](macro@crate::assert_iter_gt) ≈ iter a < iter b
//! * [`assert_iter_le!(collection1, collection2)`](macro@crate::assert_iter_gt) ≈ iter a ≤ iter b
//! * [`assert_iter_gt!(collection1, collection2)`](macro@crate::assert_iter_gt) ≈ iter a > iter b
//! * [`assert_iter_ge!(collection1, collection2)`](macro@crate::assert_iter_ge) ≈ iter a ≥ iter b
//!
//! # Example
//!
//! ```rust
//! use assertables::*;
//!
//! # fn main() {
//! let a = [1, 2];
//! let b = [1, 2];
//! assert_iter_eq!(&a, &b);
//! # }
//! ```

// Comparisons
pub mod assert_iter_eq;
pub mod assert_iter_ge;
pub mod assert_iter_gt;
pub mod assert_iter_le;
pub mod assert_iter_lt;
pub mod assert_iter_ne;
