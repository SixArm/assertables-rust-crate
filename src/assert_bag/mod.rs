//! Assert macros for comparing bag collections.
//!
//! These macros help with comparison of bag parameters, such as comparison of
//! two arrays or two vectors, where the item order does not matter, and the
//! item count does matter. These macros convert their inputs into HashMap
//! iterators.
//!
//! For eq & ne:
//!
//! * [`assert_bag_eq!(a, b)`](macro@crate::assert_bag_eq) ≈ bag a = bag b
//!
//! * [`assert_bag_ne!(a, b)`](macro@crate::assert_bag_ne) ≈ bag a ≠ bag b
//!
//! For subbag & superbag:
//!
//! * [`assert_bag_subbag(a, b)`](macro@crate::assert_bag_subbag) ≈ bag a ⊆ bag b
//!
//! * [`assert_bag_superbag!(a, b)`](macro@crate::assert_bag_superbag) ≈ bag a ⊇ bag b
//!
//!
//! # Example
//!
//! ```rust
//! # #[macro_use] extern crate assertables;
//! # fn main() {
//! let a = [1, 1];
//! let b = [1, 1];
//! assert_bag_eq!(&a, &b);
//! # }
//! ```

pub mod assert_bag_eq;
pub mod assert_bag_ne;
pub mod assert_bag_subbag;
pub mod assert_bag_superbag;
