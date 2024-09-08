//! Assert macros for comparing set collections.
//!
//! These macros help with comparison of set parameters, such as two arrays or
//! two vectors. where the item order does not matter, and the item count does
//! not matter. These macros convert their inputs into HashSet iterators.
//!
//! For eq & ne:
//!
//! * [`assert_set_eq!(a, b)`](macro@crate::assert_set_eq) ≈ set a = set b
//!
//! * [`assert_set_ne!(a, b)`](macro@crate::assert_set_ne) ≈ set a ≠ set b
//!
//! For subset & superset:
//!
//! * [`assert_set_subset!(a, b)`](macro@crate::assert_set_subset) ≈ set a ⊆ set b
//!
//! * [`assert_set_superset!(a, b)`](macro@crate::assert_set_superset) ≈ set a ⊇ set b
//!
//! For joint & disjoint:
//!
//! * [`assert_set_joint!(a, b)`](macro@crate::assert_set_joint) ≈ set a ∩ set b ≠ ∅
//!
//! * [`assert_set_disjoint!(a, b)`](macro@crate::assert_set_disjoint) ≈ set a ∩ set b = ∅
//!
//!
//! # Example
//!
//! ```rust
//! # #[macro_use] extern crate assertables;
//! # fn main() {
//! let a = [1, 2];
//! let b = [2, 1];
//! assert_set_eq!(&a, &b);
//! # }
//! ```

// Comparisons
pub mod assert_set_eq;
pub mod assert_set_ne;

// Overlaps
pub mod assert_set_joint;
pub mod assert_set_disjoint;

// Containers
pub mod assert_set_subset;
pub mod assert_set_superset;
