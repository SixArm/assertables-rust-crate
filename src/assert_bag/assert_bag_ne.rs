//! Assert a bag is not equal to another.
//!
//! Pseudocode:<br>
//! (collection1 into bag) ≠ (collection2 into bag)
//!
//! # Example
//!
//! ```rust
//! use assertables::*;
//!
//! # fn main() {
//! let a = [1, 1];
//! let b = [1, 1, 1];
//! assert_bag_ne!(&a, &b);
//! # }
//! ```
//!
//! This implementation uses [`std::collections::BTreeMap`](https://doc.rust-lang.org/std/collections/struct.BTreeMap.html) to count items and sort them.
//!
//! # Module macros
//!
//! * [`assert_bag_ne`](macro@crate::assert_bag_ne)
//! * [`assert_bag_ne_as_result`](macro@crate::assert_bag_ne_as_result)
//! * [`debug_assert_bag_ne`](macro@crate::debug_assert_bag_ne)

/// Assert a bag is not equal to another.
///
/// Pseudocode:<br>
/// (collection1 into bag) ≠ (collection2 into bag)
///
/// * If true, return Result `Ok(())`.
///
/// * Otherwise, return Result `Err` with a diagnostic message.
///
/// This macro provides the same statements as [`assert_bag_ne`](macro.assert_bag_ne.html),
/// except this macro returns a Result, rather than doing a panic.
///
/// This macro is useful for runtime checks, such as checking parameters,
/// or sanitizing inputs, or handling different results in different ways.
///
/// # Module macros
///
/// * [`assert_bag_ne`](macro@crate::assert_bag_ne)
/// * [`assert_bag_ne_as_result`](macro@crate::assert_bag_ne_as_result)
/// * [`debug_assert_bag_ne`](macro@crate::debug_assert_bag_ne)
///
#[macro_export]
macro_rules! assert_bag_ne_as_result {
    ($a_collection:expr, $b_collection:expr $(,)?) => {{
        match (&$a_collection, &$b_collection) {
            (a_collection, b_collection) => {
                let a_bag = assert_bag_impl_prep!(a_collection);
                let b_bag = assert_bag_impl_prep!(b_collection);
                if a_bag != b_bag {
                    Ok(())
                } else {
                    Err(format!(
                        concat!(
                            "assertion failed: `assert_bag_ne!(a_collection, b_collection)`\n",
                            "https://docs.rs/assertables/", env!("CARGO_PKG_VERSION"), "/assertables/macro.assert_bag_ne.html\n",
                            " a label: `{}`,\n",
                            " a debug: `{:?}`,\n",
                            " b label: `{}`,\n",
                            " b debug: `{:?}`,\n",
                            "       a: `{:?}`,\n",
                            "       b: `{:?}`"
                        ),
                        stringify!($a_collection),
                        a_collection,
                        stringify!($b_collection),
                        b_collection,
                        a_bag,
                        b_bag
                    ))
                }
            }
        }
    }};
}

#[cfg(test)]
mod test_assert_x_result {

    #[test]
    fn test_assert_bag_ne_as_result_x_success() {
        let a = [1, 1];
        let b = [1, 1, 1];
        let result = assert_bag_ne_as_result!(&a, &b);
        assert_eq!(result, Ok(()));
    }

    #[test]
    fn test_assert_bag_ne_as_result_x_failure() {
        let a = [1, 1];
        let b = [1, 1];
        let result = assert_bag_ne_as_result!(&a, &b);
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            concat!(
                "assertion failed: `assert_bag_ne!(a_collection, b_collection)`\n",
                "https://docs.rs/assertables/", env!("CARGO_PKG_VERSION"), "/assertables/macro.assert_bag_ne.html\n",
                " a label: `&a`,\n",
                " a debug: `[1, 1]`,\n",
                " b label: `&b`,\n",
                " b debug: `[1, 1]`,\n",
                "       a: `{1: 2}`,\n",
                "       b: `{1: 2}`"
            )
        );
    }
}

/// Assert a bag is not equal to another.
///
/// Pseudocode:<br>
/// (collection1 into bag) ≠ (collection2 into bag)
///
/// * If true, return `()`.
///
/// * Otherwise, call [`panic!`] with a message and the values of the
///   expressions with their debug representations.
///
/// # Examples
///
/// ```rust
/// use assertables::*;
/// # use std::panic;
///
/// # fn main() {
/// let a = [1, 1];
/// let b = [1, 1, 1];
/// assert_bag_ne!(&a, &b);
///
/// # let result = panic::catch_unwind(|| {
/// // This will panic
/// let a = [1, 1];
/// let b = [1, 1];
/// assert_bag_ne!(&a, &b);
/// # });
/// // assertion failed: `assert_bag_ne!(a_collection, b_collection)`
/// // https://docs.rs/assertables/8.17.0/assertables/macro.assert_bag_ne.html
/// //  a label: `&a`,
/// //  a debug: `[1, 1]`,
/// //  b label: `&b`,
/// //  b debug: `[1, 1]`,
/// //        a: `{1: 2}`,
/// //        b: `{1: 2}`
/// # let actual = result.unwrap_err().downcast::<String>().unwrap().to_string();
/// # let expect = concat!(
/// #     "assertion failed: `assert_bag_ne!(a_collection, b_collection)`\n",
/// #     "https://docs.rs/assertables/", env!("CARGO_PKG_VERSION"), "/assertables/macro.assert_bag_ne.html\n",
/// #     " a label: `&a`,\n",
/// #     " a debug: `[1, 1]`,\n",
/// #     " b label: `&b`,\n",
/// #     " b debug: `[1, 1]`,\n",
/// #     "       a: `{1: 2}`,\n",
/// #     "       b: `{1: 2}`"
/// # );
/// # assert_eq!(actual, expect);
/// # }
/// ```
///
/// This implementation uses [`std::collections::BTreeMap`](https://doc.rust-lang.org/std/collections/struct.BTreeMap.html) to count items and sort them.
///
/// # Module macros
///
/// * [`assert_bag_ne`](macro@crate::assert_bag_ne)
/// * [`assert_bag_ne_as_result`](macro@crate::assert_bag_ne_as_result)
/// * [`debug_assert_bag_ne`](macro@crate::debug_assert_bag_ne)
///
#[macro_export]
macro_rules! assert_bag_ne {
    ($a_collection:expr, $b_collection:expr $(,)?) => {{
        match $crate::assert_bag_ne_as_result!($a_collection, $b_collection) {
            Ok(()) => (),
            Err(err) => panic!("{}", err),
        }
    }};
    ($a_collection:expr, $b_collection:expr, $($message:tt)+) => {{
        match $crate::assert_bag_ne_as_result!($a_collection, $b_collection) {
            Ok(()) => (),
            Err(_err) => panic!("{}", $($message)+),
        }
    }};
}

/// Assert a bag is not equal to another.
///
/// Pseudocode:<br>
/// (collection1 into bag) ≠ (collection2 into bag)
///
/// This macro provides the same statements as [`assert_bag_ne`](macro.assert_bag_ne.html),
/// except this macro's statements are only enabled in non-optimized
/// builds by default. An optimized build will not execute this macro's
/// statements unless `-C debug-assertions` is passed to the compiler.
///
/// This macro is useful for checks that are too expensive to be present
/// in a release build but may be helpful during development.
///
/// The result of expanding this macro is always type checked.
///
/// An unchecked assertion allows a program in an inconsistent state to
/// keep running, which might have unexpected consequences but does not
/// introduce unsafety as long as this only happens in safe code. The
/// performance cost of assertions, however, is not measurable in general.
/// Replacing `assert*!` with `debug_assert*!` is thus only encouraged
/// after thorough profiling, and more importantly, only in safe code!
///
/// This macro is intended to work in a similar way to
/// [`std::debug_assert`](https://doc.rust-lang.org/std/macro.debug_assert.html).
///
/// # Module macros
///
/// * [`assert_bag_ne`](macro@crate::assert_bag_ne)
/// * [`assert_bag_ne_as_result`](macro@crate::assert_bag_ne_as_result)
/// * [`debug_assert_bag_ne`](macro@crate::debug_assert_bag_ne)
///
#[macro_export]
macro_rules! debug_assert_bag_ne {
    ($($arg:tt)*) => {
        if $crate::cfg!(debug_assertions) {
            $crate::assert_bag_ne!($($arg)*);
        }
    };
}
