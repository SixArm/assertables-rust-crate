//! Assert for comparing set collections.
//!
//! These macros help with comparison of set parameters, such as two arrays or
//! two vectors. where the item order does not matter, and the item count does
//! not matter. These macros convert their inputs into HashSet iterators.
//! See tutorial below.
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
//! let a = [1, 2];
//! let b = [2, 1];
//! assert_set_eq!(&a, &b);
//! ```
//!
//! ## Tutorial
//!
//! A **set** means a collection of elements, without any ordering, without duplicate elements.
//!
//! A set is sometimes written by using mathematical notation, which looks like this:
//!
//! ```text
//! set = {1, 2, 3}
//! ```
//!
//! The definition of a set includes never having duplicate elements:
//!
//! ```text
//! set = {1, 2, 3, 2} // error because the element 2 is a duplicate
//! ```
//!
//! Sets are equal when they contain all the same elements in any order:
//!
//! ```text
//! {1, 2, 3} = {1, 2, 3} (same order)
//! {1, 2, 3} = {3, 2, 1} (different order)
//! ```
//!
//! Sets are not equal when they contain any different elements:
//!
//! ```text
//! {1, 2, 3} ≠ {1, 2}
//! {1, 2, 3} ≠ {1, 2, 3, 4}
//! ```
//!
//! To create a set using Rust, one way is to create an array or vector, then convert it into an iterator by using the method `into_iter`, then convert the elements into a set by using `std::collections::BTreeSet`:
//!
//! ```rust
//! # use ::std::collections::BTreeSet;
//! let array = [1, 2, 3];
//! let set: BTreeSet<_> = array.into_iter().collect();
//! ```
//!
//! To compare two arrays as sets, one way is to convert each array to a set, then use `assert_eq!` to compare the sets:
//!
//! ```rust
//! # use ::std::collections::BTreeSet;
//! let array1 = [1, 2, 3];
//! let array2 = [3, 2, 1];
//! let set1: BTreeSet<_> = array1.into_iter().collect();
//! let set2: BTreeSet<_> = array2.into_iter().collect();
//! assert_eq!(set1, set2);
//! ```
//!
//! The `assertables` crate provides macros that do the conversion for you:
//!
//! ```rust
//! # use ::std::collections::BTreeSet;
//! # use assertables::*;
//! let array1 = [1, 2, 3];
//! let array2 = [3, 2, 1];
//! assert_set_eq!(array1, array2);
//! ```

/// Assert set implementation preparation.
#[macro_export]
macro_rules! assert_set_impl_prep {
    ($impl_into_iter:expr $(,)?) => {{
        match (&$impl_into_iter) {
            impl_into_iter => impl_into_iter.into_iter().collect(),
        }
    }};
}

// Comparisons
pub mod assert_set_eq;
pub mod assert_set_ne;

// Overlaps
pub mod assert_set_disjoint;
pub mod assert_set_joint;

// Containers
pub mod assert_set_subset;
pub mod assert_set_superset;
