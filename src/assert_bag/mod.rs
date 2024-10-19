//! Assert for comparing bag collections.
//!
//! These macros help with comparison of bag parameters, such as comparison of
//! two arrays or two vectors, where the item order does not matter, and the
//! item count does matter. These macros convert their inputs into HashMap
//! iterators.
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
//! # fn main() {
//! let a = [1, 1];
//! let b = [1, 1];
//! assert_bag_eq!(&a, &b);
//! # }
//! ```

/// Assert bag implementation preparation.
#[macro_export]
macro_rules! assert_bag_impl_prep {
    ($impl_into_iter:expr $(,)?) => {{
        match (&$impl_into_iter) {
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
    }};
}

/// Format assert failure error message.
#[macro_export]
macro_rules! assert_bag_impl_err {
    ($name:ident, $($arg:tt)*) => {
        format!(
            concat!(
                "assertion failed: `{}!(a_collection, b_collection)`\n",
                "https://docs.rs/assertables/8.18.0/assertables/macro.{}.html\n",
                " a label: `{}`,\n",
                " a debug: `{:?}`,\n",
                " b label: `{}`,\n",
                " b debug: `{:?}`,\n",
                "   a bag: `{:?}`,\n",
                "   b bag: `{:?}`"
            ),
            stringify!($name),
            stringify!($name),
            $($arg)*
        )
    }
}

pub mod assert_bag_eq;
pub mod assert_bag_ne;
pub mod assert_bag_subbag;
pub mod assert_bag_superbag;
