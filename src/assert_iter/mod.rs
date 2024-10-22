//! Assert for comparing iter collections.
//!
//! These macros help with comparison of iter parameters, such as two arrays or
//! two vectors. These macros convert each input using the std::iter::Iterator trait.
//!
//! * [`assert_iter_eq2!(collection1, collection2)`](macro@crate::assert_iter_eq2) ≈ iter a = iter b
//!
//! * [`assert_iter_ne2!(collection1, collection2)`](macro@crate::assert_iter_ne2) ≈ iter a ≠ iter b
//!
//! * [`assert_iter_lt2!(collection1, collection2)`](macro@crate::assert_iter_gt2) ≈ iter a < iter b
//!
//! * [`assert_iter_le2!(collection1, collection2)`](macro@crate::assert_iter_gt2) ≈ iter a ≤ iter b
//!
//! * [`assert_iter_gt2!(collection1, collection2)`](macro@crate::assert_iter_gt2) ≈ iter a > iter b
//!
//! * [`assert_iter_ge2!(collection1, collection2)`](macro@crate::assert_iter_gt2) ≈ iter a ≥ iter b
//!
//! # Example
//!
//! ```rust
//! use assertables::*;
//!
//! # fn main() {
//! let a = [1, 2];
//! let b = [1, 2];
//! assert_iter_eq2!(&a, &b);
//! # }
//! ```

// Comparisons
pub mod assert_iter_eq2;
pub mod assert_iter_ge2;
pub mod assert_iter_gt2;
pub mod assert_iter_le2;
pub mod assert_iter_lt2;
pub mod assert_iter_ne2;
