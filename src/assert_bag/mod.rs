//! Assert for comparing bag collections.
//!
//! These macros help with comparison of bag parameters, such as comparison of
//! two arrays or two vectors, where the item order does not matter, and the
//! item count does matter. These macros convert their inputs into HashMap
//! iterators. See tutorial below.
//!
//! For eq & ne:
//!
//! * [`assert_bag_eq!(collection1, collection2)`](macro@crate::assert_bag_eq) ≈ bag a = bag b
//!
//! * [`assert_bag_ne!(collection1, collection2)`](macro@crate::assert_bag_ne) ≈ bag a ≠ bag b
//!
//! For subbag & superbag:
//!
//! * [`assert_bag_subbag(a, b)`](macro@crate::assert_bag_subbag) ≈ bag a ⊆ bag b
//!
//! * [`assert_bag_superbag!(collection1, collection2)`](macro@crate::assert_bag_superbag) ≈ bag a ⊇ bag b
//!
//!
//! # Example
//!
//! ```rust
//! use assertables::*;
//!
//! let a = [1, 1];
//! let b = [1, 1];
//! assert_bag_eq!(&a, &b);
//! ```
//!
//! ## Tutorial
//!
//! A **bag** means a collection of elements, without any ordering, and tracking duplicate elements.
//!
//! A bag is sometimes written by using mathematical notation, which looks like this:
//!
//! ```text
//! bag = {1, 1, 1, 2, 3}
//! ```
//!
//! Bags are equal when they contain all the same elements and corresponding counts in any order:
//!
//! ```text
//! {1, 1, 1, 2, 3} = {1, 1, 1, 2, 3} (same order)
//! {1, 1, 1, 2, 3} = {1, 3, 1, 2, 1} (different order)
//! ```
//!
//! Bags are not equal when they contain any different elements or any different corresponding counts:
//!
//! ```text
//! {1, 1, 1, 2, 3} ≠ {1, 1, 2, 3}
//! {1, 1, 1, 2, 3} ≠ {1, 1, 1, 2, 3, 4}
//! ```
//!
//! To create a bag using Rust, one way is to create an array or vector, then convert it into an iterator by using the method `into_iter`, then convert the elements into a map by using `std::collections::BTreeMap` and tracking each element's count:
//!
//! ```rust
//! # use ::std::collections::BTreeMap;
//! let array = [1, 1, 1, 2, 3];
//! let mut bag: BTreeMap<_, usize> = BTreeMap::new();
//! for x in array.into_iter() {
//!     let n = bag.entry(x).or_insert(0);
//!     *n += 1;
//! }
//! ```
//!
//! To compare two arrays as bags, one way is to convert each array to a bag, then use `assert_eq!` to compare the bags:
//!
//! ```rust
//! # use ::std::collections::BTreeMap;
//! let array1 = [1, 1, 1, 2, 3];
//! let array2 = [1, 3, 1, 2, 1];
//! let mut bag1: BTreeMap<_, usize> = BTreeMap::new();
//! for x in array1.into_iter() {
//!     let n = bag1.entry(x).or_insert(0);
//!     *n += 1;
//! }
//! let mut bag2: BTreeMap<_, usize> = BTreeMap::new();
//! for x in array2.into_iter() {
//!     let n = bag2.entry(x).or_insert(0);
//!     *n += 1;
//! }
//! assert_eq!(bag1, bag2);
//! ```
//!
//! The `assertables` crate provides macros that do the conversion for you:
//!
//! ```rust
//! # use ::std::collections::BTreeMap;
//! # use assertables::*;
//! let array1 = [1, 2, 3];
//! let array2 = [3, 2, 1];
//! assert_bag_eq!(array1, array2);
//! ```

/// Assert bag implementation preparation.
#[macro_export]
macro_rules! assert_bag_impl_prep {
    ($impl_into_iter:expr $(,)?) => {
        match ($impl_into_iter) {
            impl_into_iter => {
                let mut bag: std::collections::BTreeMap<_, usize> =
                    std::collections::BTreeMap::new();
                for x in impl_into_iter.into_iter() {
                    let n = bag.entry(x).or_insert(0);
                    *n += 1;
                }
                bag
            }
        }
    };
}

pub mod assert_bag_eq;
pub mod assert_bag_ne;
pub mod assert_bag_subbag;
pub mod assert_bag_superbag;
