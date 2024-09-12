//! Assert a set is not equal to another.
//!
//! # Example
//!
//! ```rust
//! # #[macro_use] extern crate assertables;
//! # fn main() {
//! let a = [1, 2];
//! let b = [3, 4];
//! assert_set_ne!(&a, &b);
//! # }
//! ```
//!
//! This implementation uses [`std::collections::BTreeSet`](https://doc.rust-lang.org/std/collections/struct.BTreeSet.html) to count items and sort them.
//!
//! # Module macros
//!
//! * [`assert_set_ne`](macro@crate::assert_set_ne)
//! * [`assert_set_ne_as_result`](macro@crate::assert_set_ne_as_result)
//! * [`debug_assert_set_ne`](macro@crate::debug_assert_set_ne)

/// Assert a set is not equal to another.
///
/// * If true, return Result `Ok(())`.
///
/// * Otherwise, return Result `Err` with a diagnostic message.
///
/// This macro provides the same statements as [`assert_`](macro.assert_.html),
/// except this macro returns a Result, rather than doing a panic.
///
/// This macro is useful for runtime checks, such as checking parameters,
/// or sanitizing inputs, or handling different results in different ways.
///
/// # Module macros
///
/// * [`assert_set_ne`](macro@crate::assert_set_ne)
/// * [`assert_set_ne_as_result`](macro@crate::assert_set_ne_as_result)
/// * [`debug_assert_set_ne`](macro@crate::debug_assert_set_ne)
///
#[macro_export]
macro_rules! assert_set_ne_as_result {
    ($a_collection:expr, $b_collection:expr $(,)?) => {{
        match (&$a_collection, &$b_collection) {
            (a_val, b_val) => {
                let a_set: ::std::collections::BTreeSet<_> = a_val.into_iter().collect();
                let b_set: ::std::collections::BTreeSet<_> = b_val.into_iter().collect();
                if a_set != b_set {
                    Ok(())
                } else {
                    Err(format!(
                        concat!(
                            "assertion failed: `assert_set_ne!(a_collection, b_collection)`\n",
                            " a label: `{}`,\n",
                            " a debug: `{:?}`,\n",
                            " b label: `{}`,\n",
                            " b debug: `{:?}`,\n",
                            "       a: `{:?}`,\n",
                            "       b: `{:?}`"
                        ),
                        stringify!($a_collection),
                        $a_collection,
                        stringify!($b_collection),
                        $b_collection,
                        &a_set,
                        &b_set
                    ))
                }
            }
        }
    }};
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_assert_set_ne_as_result_x_success() {
        let a = [1, 2];
        let b = [3, 4];
        let result = assert_set_ne_as_result!(&a, &b);
        assert_eq!(result, Ok(()));
    }

    #[test]
    fn test_assert_set_ne_as_result_x_failure() {
        let a = [1, 2];
        let b = [1, 2];
        let result = assert_set_ne_as_result!(&a, &b);
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            concat!(
                "assertion failed: `assert_set_ne!(a_collection, b_collection)`\n",
                " a label: `&a`,\n",
                " a debug: `[1, 2]`,\n",
                " b label: `&b`,\n",
                " b debug: `[1, 2]`,\n",
                "       a: `{1, 2}`,\n",
                "       b: `{1, 2}`"
            )
        );
    }
}

/// Assert a set is not equal to another.
///
/// * If true, return `()`.
///
/// * Otherwise, call [`panic!`] with a message and the values of the
///   expressions with their debug representations.
///
/// # Examples
///
/// ```rust
/// # #[macro_use] extern crate assertables;
/// # use std::panic;
/// # fn main() {
/// let a = [1, 2];
/// let b = [3, 4];
/// assert_set_ne!(&a, &b);
///
/// # let result = panic::catch_unwind(|| {
/// let a = [1, 2];
/// let b = [1, 2];
/// assert_set_ne!(&a, &b);
/// # });
/// // assertion failed: `assert_set_ne!(a_collection, b_collection)`
/// //  a label: `&a`,
/// //  a debug: `[1, 2]`,
/// //  b label: `&b`,
/// //  b debug: `[1, 2]`,
/// //        a: `{1, 2}`,
/// //        b: `{1, 2}`
/// # let actual = result.unwrap_err().downcast::<String>().unwrap().to_string();
/// # let expect = concat!(
/// #     "assertion failed: `assert_set_ne!(a_collection, b_collection)`\n",
/// #     " a label: `&a`,\n",
/// #     " a debug: `[1, 2]`,\n",
/// #     " b label: `&b`,\n",
/// #     " b debug: `[1, 2]`,\n",
/// #     "       a: `{1, 2}`,\n",
/// #     "       b: `{1, 2}`"
/// # );
/// # assert_eq!(actual, expect);
/// # }
/// ```
///
/// This implementation uses [`std::collections::BTreeSet`](https://doc.rust-lang.org/std/collections/struct.BTreeSet.html) to count items and sort them.
///
/// # Module macros
///
/// * [`assert_set_ne`](macro@crate::assert_set_ne)
/// * [`assert_set_ne_as_result`](macro@crate::assert_set_ne_as_result)
/// * [`debug_assert_set_ne`](macro@crate::debug_assert_set_ne)
///
#[macro_export]
macro_rules! assert_set_ne {
    ($a_collection:expr, $b_collection:expr $(,)?) => ({
        match assert_set_ne_as_result!($a_collection, $b_collection) {
            Ok(()) => (),
            Err(err) => panic!("{}", err),
        }
    });
    ($a_collection:expr, $b_collection:expr, $($message:tt)+) => ({
        match assert_set_ne_as_result!($a_collection, $b_collection) {
            Ok(()) => (),
            Err(_err) => panic!("{}", $($message)+),
        }
    });
}

/// Assert a set is not equal to another.
///
/// This macro provides the same statements as [`assert_set_ne`](macro.assert_set_ne.html),
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
/// * [`assert_set_ne`](macro@crate::assert_set_ne)
/// * [`assert_set_ne`](macro@crate::assert_set_ne)
/// * [`debug_assert_set_ne`](macro@crate::debug_assert_set_ne)
///
#[macro_export]
macro_rules! debug_assert_set_ne {
    ($($arg:tt)*) => {
        if $crate::cfg!(debug_assertions) {
            $crate::assert_set_ne!($($arg)*);
        }
    };
}
