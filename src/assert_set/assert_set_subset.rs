//! Assert a set is a subset of another.
//!
//! Pseudocode:<br>
//! (a_collection ⇒ a_set) ⊂ (b_collection ⇒ b_set)
//!
//! # Example
//!
//! ```rust
//! use assertables::*;
//!
//! # fn main() {
//! let a = [1, 2];
//! let b = [1, 2, 3];
//! assert_set_subset!(&a, &b);
//! # }
//! ```
//!
//! This implementation uses [`::std::collections::BTreeSet`](https://doc.rust-lang.org/std/collections/struct.BTreeSet.html) to count items and sort them.
//!
//! # Module macros
//!
//! * [`assert_set_subset`](macro@crate::assert_set_subset)
//! * [`assert_set_subset_as_result`](macro@crate::assert_set_subset_as_result)
//! * [`debug_assert_set_subset`](macro@crate::debug_assert_set_subset)

/// Assert a set is a subset of another.
///
/// Pseudocode:<br>
/// (a_collection ⇒ a_set) ⊂ (b_collection ⇒ b_set)
///
/// * If true, return Result `Ok((a_set, b_set))`.
///
/// * Otherwise, return Result `Err(message)`.
///
/// This macro provides the same statements as [`assert_`](macro.assert_.html),
/// except this macro returns a Result, rather than doing a panic.
///
/// This macro is useful for runtime checks, such as checking parameters,
/// or sanitizing inputs, or handling different results in different ways.
///
/// # Module macros
///
/// * [`assert_set_subset`](macro@crate::assert_set_subset)
/// * [`assert_set_subset_as_result`](macro@crate::assert_set_subset_as_result)
/// * [`debug_assert_set_subset`](macro@crate::debug_assert_set_subset)
///
#[macro_export]
macro_rules! assert_set_subset_as_result {
    ($a_collection:expr, $b_collection:expr $(,)?) => {{
        match (&$a_collection, &$b_collection) {
            (a_collection, b_collection) => {
                let a: ::std::collections::BTreeSet<_> = assert_set_impl_prep!(a_collection);
                let b: ::std::collections::BTreeSet<_> = assert_set_impl_prep!(b_collection);
                if a.is_subset(&b) {
                    Ok((a, b))
                } else {
                    Err(
                        format!(
                            concat!(
                                "assertion failed: `assert_set_subset!(a_collection, b_collection)`\n",
                                "https://docs.rs/assertables/9.2.0/assertables/macro.assert_set_subset.html\n",
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
                            a,
                            b
                        )
                    )
                }
            }
        }
    }};
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeSet;

    #[test]
    fn test_assert_set_subset_as_result_x_success() {
        let a = [1, 2];
        let b = [1, 2, 3];
        let result = assert_set_subset_as_result!(&a, &b);
        assert_eq!(
            result.unwrap(),
            (BTreeSet::from([&1, &2]), BTreeSet::from([&1, &2, &3]))
        );
    }

    #[test]
    fn test_assert_set_subset_as_result_x_failure() {
        let a = [1, 2, 3];
        let b = [1, 2];
        let result = assert_set_subset_as_result!(&a, &b);
        assert_eq!(
            result.unwrap_err(),
            concat!(
                "assertion failed: `assert_set_subset!(a_collection, b_collection)`\n",
                "https://docs.rs/assertables/9.2.0/assertables/macro.assert_set_subset.html\n",
                " a label: `&a`,\n",
                " a debug: `[1, 2, 3]`,\n",
                " b label: `&b`,\n",
                " b debug: `[1, 2]`,\n",
                "       a: `{1, 2, 3}`,\n",
                "       b: `{1, 2}`"
            )
        );
    }
}

/// Assert a set is a subset of another.
///
/// Pseudocode:<br>
/// (a_collection ⇒ a_set) ⊂ (b_collection ⇒ b_set)
///
/// * If true, return `(a_set, b_set)`.
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
/// let a = [1, 2];
/// let b = [1, 2, 3];
/// assert_set_subset!(&a, &b);
///
/// # let result = panic::catch_unwind(|| {
/// // This will panic
/// let a = [1, 2, 3];
/// let b = [1, 2];
/// assert_set_subset!(&a, &b);
/// # });
/// // assertion failed: `assert_set_subset!(a_collection, b_collection)`
/// // https://docs.rs/assertables/9.2.0/assertables/macro.assert_set_subset.html
/// //  a label: `&a`,
/// //  a debug: `[1, 2, 3]`,
/// //  b label: `&b`,
/// //  b debug: `[1, 2]`,
/// //        a: `{1, 2, 3}`,
/// //        b: `{1, 2}`
/// # let actual = result.unwrap_err().downcast::<String>().unwrap().to_string();
/// # let expect = concat!(
/// #     "assertion failed: `assert_set_subset!(a_collection, b_collection)`\n",
/// #     "https://docs.rs/assertables/9.2.0/assertables/macro.assert_set_subset.html\n",
/// #     " a label: `&a`,\n",
/// #     " a debug: `[1, 2, 3]`,\n",
/// #     " b label: `&b`,\n",
/// #     " b debug: `[1, 2]`,\n",
/// #     "       a: `{1, 2, 3}`,\n",
/// #     "       b: `{1, 2}`"
/// # );
/// # assert_eq!(actual, expect);
/// # }
/// ```
///
/// This implementation uses [`::std::collections::BTreeSet`](https://doc.rust-lang.org/std/collections/struct.BTreeSet.html) to count items and sort them.
///
/// # Module macros
///
/// * [`assert_set_subset`](macro@crate::assert_set_subset)
/// * [`assert_set_subset_as_result`](macro@crate::assert_set_subset_as_result)
/// * [`debug_assert_set_subset`](macro@crate::debug_assert_set_subset)
///
#[macro_export]
macro_rules! assert_set_subset {
    ($a_collection:expr, $b_collection:expr $(,)?) => {{
        match $crate::assert_set_subset_as_result!($a_collection, $b_collection) {
            Ok(x) => x,
            Err(err) => panic!("{}", err),
        }
    }};
    ($a_collection:expr, $b_collection:expr, $($message:tt)+) => {{
        match $crate::assert_set_subset_as_result!($a_collection, $b_collection) {
            Ok(x) => x,
            Err(_err) => panic!("{}", $($message)+),
        }
    }};
}

/// Assert a set is a subset of another.
///
/// Pseudocode:<br>
/// (a_collection ⇒ a_set) ⊂ (b_collection ⇒ b_set)
///
/// This macro provides the same statements as [`assert_set_subset`](macro.assert_set_subset.html),
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
/// [`::std::debug_assert`](https://doc.rust-lang.org/std/macro.debug_assert.html).
///
/// # Module macros
///
/// * [`assert_set_subset`](macro@crate::assert_set_subset)
/// * [`assert_set_subset`](macro@crate::assert_set_subset)
/// * [`debug_assert_set_subset`](macro@crate::debug_assert_set_subset)
///
#[macro_export]
macro_rules! debug_assert_set_subset {
    ($($arg:tt)*) => {
        if $crate::cfg!(debug_assertions) {
            $crate::assert_set_subset!($($arg)*);
        }
    };
}
