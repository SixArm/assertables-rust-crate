//! Assert for comparing set collections.
//!
//! These macros help with comparison of set parameters, such as two arrays or
//! two vectors. where the item order does not matter, and the item count does
//! not matter. These macros convert their inputs into HashSet iterators.
//!
//! For eq & ne:
//!
//! * [`assert_set_eq!(collection1, collection2)`](macro@crate::assert_set_eq) ≈ set a = set b
//!
//! * [`assert_set_ne!(collection1, collection2)`](macro@crate::assert_set_ne) ≈ set a ≠ set b
//!
//! For subset & superset:
//!
//! * [`assert_set_subset!(collection1, collection2)`](macro@crate::assert_set_subset) ≈ set a ⊆ set b
//!
//! * [`assert_set_superset!(collection1, collection2)`](macro@crate::assert_set_superset) ≈ set a ⊇ set b
//!
//! For joint & disjoint:
//!
//! * [`assert_set_joint!(collection1, collection2)`](macro@crate::assert_set_joint) ≈ set a ∩ set b ≠ ∅
//!
//! * [`assert_set_disjoint!(collection1, collection2)`](macro@crate::assert_set_disjoint) ≈ set a ∩ set b = ∅
//!
//!
//! # Example
//!
//! ```rust
//! use assertables::*;
//!
//! # fn main() {
//! let a = [1, 2];
//! let b = [2, 1];
//! assert_set_eq!(&a, &b);
//! # }
//! ```

/// Assert set implementation preparation.
#[macro_export]
macro_rules! assert_set_impl_prep {
    ($impl_into_iter:expr $(,)?) => {{
        match (&$impl_into_iter) {
            impl_into_iter => impl_into_iter.into_iter().collect()
        }
    }};
}

// Comparisons
pub mod assert_set_eq;
pub mod assert_set_ne;

// Overlaps
pub mod assert_set_joint;
pub mod assert_set_disjoint;

// Containers
pub mod assert_set_subset;
pub mod assert_set_superset;
