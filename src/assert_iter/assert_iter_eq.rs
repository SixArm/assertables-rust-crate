//! Assert an iter is equal to another.
//!
//! Pseudocode:<br>
//! (collection1 into iter) = (collection2 into iter)
//!
//! # Example
//!
//! ```rust
//! use assertables::*;
//!
//! # fn main() {
//! let a = [1, 2];
//! let b = [1, 2];
//! assert_iter_eq!(&a, &b);
//! # }
//! ```
//!
//! This implementation uses [`::std::iter::Iterator`](https://doc.rust-lang.org/std/iter/trait.Iterator.html).
//!
//! # Module macros
//!
//! * [`assert_iter_eq`](macro@crate::assert_iter_eq)
//! * [`assert_iter_eq_as_result`](macro@crate::assert_iter_eq_as_result)
//! * [`debug_assert_iter_eq`](macro@crate::debug_assert_iter_eq)

/// Assert an iterable is equal to another.
///
/// Pseudocode:<br>
/// (collection1 into iter) = (collection2 into iter)
///
/// * If true, return Result `Ok(())`.
///
/// * Otherwise, return Result `Err(message)`.
///
/// This macro provides the same statements as [`assert_`](macro.assert_.html),
/// except this macro returns a Result, rather than doing a panic.
///
/// This macro is useful for runtime checks, such as checking parameters,
/// or sanitizing inputs, or handling different results in different ways.
///
/// This implementation uses [`::std::iter::Iterator`](https://doc.rust-lang.org/std/iter/trait.Iterator.html).
///
/// # Module macros
///
/// * [`assert_iter_eq`](macro@crate::assert_iter_eq)
/// * [`assert_iter_eq_as_result`](macro@crate::assert_iter_eq_as_result)
/// * [`debug_assert_iter_eq`](macro@crate::debug_assert_iter_eq)
///
#[macro_export]
macro_rules! assert_iter_eq_as_result {
    ($a_collection:expr, $b_collection:expr $(,)?) => {{
        match (&$a_collection, &$b_collection) {
            (a_collection, b_collection) => {
                let a = a_collection.into_iter();
                let b = b_collection.into_iter();
                if a.eq(b) {
                    Ok(())
                } else {
                    Err(
                        format!(
                            concat!(
                                "assertion failed: `assert_iter_eq!(a_collection, b_collection)`\n",
                                "https://docs.rs/assertables/9.2.0/assertables/macro.assert_iter_eq.html\n",
                                " a label: `{}`,\n",
                                " a debug: `{:?}`,\n",
                                " b label: `{}`,\n",
                                " b debug: `{:?}`"
                            ),
                            stringify!($a_collection),
                            a_collection,
                            stringify!($b_collection),
                            b_collection
                        )
                    )
                }
            }
        }
    }};
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_assert_iter_eq_as_result_success() {
        let a = [1, 2];
        let b = [1, 2];
        let result = assert_iter_eq_as_result!(&a, &b);
        assert_eq!(result, Ok(()));
    }

    #[test]
    fn test_assert_iter_eq_as_result_failure() {
        let a = [1, 2];
        let b = [2, 1];
        let result = assert_iter_eq_as_result!(&a, &b);
        assert_eq!(
            result.unwrap_err(),
            concat!(
                "assertion failed: `assert_iter_eq!(a_collection, b_collection)`\n",
                "https://docs.rs/assertables/9.2.0/assertables/macro.assert_iter_eq.html\n",
                " a label: `&a`,\n",
                " a debug: `[1, 2]`,\n",
                " b label: `&b`,\n",
                " b debug: `[2, 1]`"
            )
        );
    }
}

/// Assert an iterable is equal to another.
///
/// Pseudocode:<br>
/// (collection1 into iter) = (collection2 into iter)
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
/// let a = [1, 2];
/// let b = [1, 2];
/// assert_iter_eq!(&a, &b);
///
/// # let result = panic::catch_unwind(|| {
/// // This will panic
/// let a = [1, 2];
/// let b = [2, 1];
/// assert_iter_eq!(&a, &b);
/// # });
/// // assertion failed: `assert_iter_eq!(a_collection, b_collection)`
/// // https://docs.rs/assertables/9.2.0/assertables/macro.assert_iter_eq.html
/// //  a label: `&a`,
/// //  a debug: `[1, 2]`,
/// //  b label: `&b`,
/// //  b debug: `[2, 1]`
/// # let actual = result.unwrap_err().downcast::<String>().unwrap().to_string();
/// # let expect = concat!(
/// #     "assertion failed: `assert_iter_eq!(a_collection, b_collection)`\n",
/// #     "https://docs.rs/assertables/9.2.0/assertables/macro.assert_iter_eq.html\n",
/// #     " a label: `&a`,\n",
/// #     " a debug: `[1, 2]`,\n",
/// #     " b label: `&b`,\n",
/// #     " b debug: `[2, 1]`",
/// # );
/// # assert_eq!(actual, expect);
/// # }
/// ```
///
/// This implementation uses [`::std::iter::Iterator`](https://doc.rust-lang.org/std/iter/trait.Iterator.html).
///
/// # Module macros
///
/// * [`assert_iter_eq`](macro@crate::assert_iter_eq)
/// * [`assert_iter_eq_as_result`](macro@crate::assert_iter_eq_as_result)
/// * [`debug_assert_iter_eq`](macro@crate::debug_assert_iter_eq)
///
#[macro_export]
macro_rules! assert_iter_eq {
    ($a_collection:expr, $b_collection:expr $(,)?) => {{
        match $crate::assert_iter_eq_as_result!($a_collection, $b_collection) {
            Ok(()) => (),
            Err(err) => panic!("{}", err),
        }
    }};
    ($a_collection:expr, $b_collection:expr, $($message:tt)+) => {{
        match $crate::assert_iter_eq_as_result!($a_collection, $b_collection) {
            Ok(()) => (),
            Err(_err) => panic!("{}", $($message)+),
        }
    }};
}

/// Assert an iterable is equal to another.
///
/// Pseudocode:<br>
/// (collection1 into iter) = (collection2 into iter)
///
/// This macro provides the same statements as [`assert_iter_eq`](macro.assert_iter_eq.html),
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
/// * [`assert_iter_eq`](macro@crate::assert_iter_eq)
/// * [`assert_iter_eq`](macro@crate::assert_iter_eq)
/// * [`debug_assert_iter_eq`](macro@crate::debug_assert_iter_eq)
///
#[macro_export]
macro_rules! debug_assert_iter_eq {
    ($($arg:tt)*) => {
        if $crate::cfg!(debug_assertions) {
            $crate::assert_iter_eq!($($arg)*);
        }
    };
}
