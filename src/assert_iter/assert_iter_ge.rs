//! Assert an iter is greater than or equal to another.
//!
//! Pseudocode:<br>
//! (collection1 into iter) ≥ (collection2 into iter)
//!
//! # Example
//!
//! ```rust
//! use assertables::*;
//!
//! let a = [1, 2];
//! let b = [1, 2];
//! assert_iter_ge!(&a, &b);
//! ```
//!
//! This implementation uses [`::std::iter::Iterator`](https://doc.rust-lang.org/std/iter/trait.Iterator.html).
//!
//! # Module macros
//!
//! * [`assert_iter_ge`](macro@crate::assert_iter_ge)
//! * [`assert_iter_ge_as_result`](macro@crate::assert_iter_ge_as_result)
//! * [`debug_assert_iter_ge`](macro@crate::debug_assert_iter_ge)

/// Assert an iterable is greater than or equal to another.
///
/// Pseudocode:<br>
/// (collection1 into iter) ≥ (collection2 into iter)
///
/// * If true, return Result `Ok(())`.
///
/// * Otherwise, return Result `Err(message)`.
///
/// This macro is useful for runtime checks, such as checking parameters,
/// or sanitizing inputs, or handling different results in different ways.
///
/// This implementation uses [`::std::iter::Iterator`](https://doc.rust-lang.org/std/iter/trait.Iterator.html).
///
/// # Module macros
///
/// * [`assert_iter_ge`](macro@crate::assert_iter_ge)
/// * [`assert_iter_ge_as_result`](macro@crate::assert_iter_ge_as_result)
/// * [`debug_assert_iter_ge`](macro@crate::debug_assert_iter_ge)
///
#[macro_export]
macro_rules! assert_iter_ge_as_result {
    ($a_collection:expr, $b_collection:expr $(,)?) => {
        match (&$a_collection, &$b_collection) {
            (a_collection, b_collection) => {
                let a = a_collection.into_iter();
                let b = b_collection.into_iter();
                if a.ge(b) {
                    Ok(())
                } else {
                    Err(format!(
                        concat!(
                            "assertion failed: `assert_iter_ge!(a_collection, b_collection)`\n",
                            "https://docs.rs/assertables/9.8.1/assertables/macro.assert_iter_ge.html\n",
                            " a label: `{}`,\n",
                            " a debug: `{:?}`,\n",
                            " b label: `{}`,\n",
                            " b debug: `{:?}`"
                        ),
                        stringify!($a_collection),
                        a_collection,
                        stringify!($b_collection),
                        b_collection
                    ))
                }
            }
        }
    };
}

#[cfg(test)]
mod test_assert_iter_ge_as_result {
    // use std::sync::Once;

    #[test]
    fn gt() {
        let a = [3, 4];
        let b = [1, 2];
        for _ in 0..1 {
            let actual = assert_iter_ge_as_result!(&a, &b);
            assert_eq!(actual.unwrap(), ());
        }
    }

    #[test]
    fn eq() {
        let a = [1, 2];
        let b = [1, 2];
        for _ in 0..1 {
            let actual = assert_iter_ge_as_result!(&a, &b);
            assert_eq!(actual.unwrap(), ());
        }
    }

    #[test]
    fn lt() {
        let a = [1, 2];
        let b = [3, 4];
        let actual = assert_iter_ge_as_result!(&a, &b);
        let message = concat!(
            "assertion failed: `assert_iter_ge!(a_collection, b_collection)`\n",
            "https://docs.rs/assertables/9.8.1/assertables/macro.assert_iter_ge.html\n",
            " a label: `&a`,\n",
            " a debug: `[1, 2]`,\n",
            " b label: `&b`,\n",
            " b debug: `[3, 4]`"
        );
        assert_eq!(actual.unwrap_err(), message);
    }
}

/// Assert an iterable is greater than or equal to another.
///
/// Pseudocode:<br>
/// (collection1 into iter) ≥ (collection2 into iter)
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
/// let a = [3, 4];
/// let b = [1, 2];
/// assert_iter_ge!(&a, &b);
///
/// # let result = panic::catch_unwind(|| {
/// // This will panic
/// let a = [1, 2];
/// let b = [3, 4];
/// assert_iter_ge!(&a, &b);
/// # });
/// // assertion failed: `assert_iter_ge!(a_collection, b_collection)`
/// // https://docs.rs/assertables/…/assertables/macro.assert_iter_ge.html
/// //  a label: `&a`,
/// //  a debug: `[1, 2]`,
/// //  b label: `&b`,
/// //  b debug: `[3, 4]`
/// # let actual = result.unwrap_err().downcast::<String>().unwrap().to_string();
/// # let message = concat!(
/// #     "assertion failed: `assert_iter_ge!(a_collection, b_collection)`\n",
/// #     "https://docs.rs/assertables/9.8.1/assertables/macro.assert_iter_ge.html\n",
/// #     " a label: `&a`,\n",
/// #     " a debug: `[1, 2]`,\n",
/// #     " b label: `&b`,\n",
/// #     " b debug: `[3, 4]`",
/// # );
/// # assert_eq!(actual, message);
/// # }
/// ```
///
/// This implementation uses [`::std::iter::Iterator`](https://doc.rust-lang.org/std/iter/trait.Iterator.html).
///
/// # Module macros
///
/// * [`assert_iter_ge`](macro@crate::assert_iter_ge)
/// * [`assert_iter_ge_as_result`](macro@crate::assert_iter_ge_as_result)
/// * [`debug_assert_iter_ge`](macro@crate::debug_assert_iter_ge)
///
#[macro_export]
macro_rules! assert_iter_ge {
    ($a_collection:expr, $b_collection:expr $(,)?) => {
        match $crate::assert_iter_ge_as_result!($a_collection, $b_collection) {
            Ok(()) => (),
            Err(err) => panic!("{}", err),
        }
    };
    ($a_collection:expr, $b_collection:expr, $($message:tt)+) => {
        match $crate::assert_iter_ge_as_result!($a_collection, $b_collection) {
            Ok(()) => (),
            Err(err) => panic!("{}\n{}", format_args!($($message)+), err),
        }
    };
}

#[cfg(test)]
mod test_assert_iter_ge {
    use std::panic;

    #[test]
    fn gt() {
        let a = [3, 4];
        let b = [1, 2];
        for _ in 0..1 {
            let actual = assert_iter_ge!(&a, &b);
            assert_eq!(actual, ());
        }
    }

    #[test]
    fn eq() {
        let a = [1, 2];
        let b = [1, 2];
        for _ in 0..1 {
            let actual = assert_iter_ge!(&a, &b);
            assert_eq!(actual, ());
        }
    }

    #[test]
    fn lt() {
        let a = [1, 2];
        let b = [3, 4];
        let result = panic::catch_unwind(|| {
            let _actual = assert_iter_ge!(&a, &b);
        });
        let message = concat!(
            "assertion failed: `assert_iter_ge!(a_collection, b_collection)`\n",
            "https://docs.rs/assertables/9.8.1/assertables/macro.assert_iter_ge.html\n",
            " a label: `&a`,\n",
            " a debug: `[1, 2]`,\n",
            " b label: `&b`,\n",
            " b debug: `[3, 4]`"
        );
        assert_eq!(
            result
                .unwrap_err()
                .downcast::<String>()
                .unwrap()
                .to_string(),
            message
        );
    }
}

/// Assert an iterable is greater than or equal to another.
///
/// Pseudocode:<br>
/// (collection1 into iter) ≥ (collection2 into iter)
///
/// This macro provides the same statements as [`assert_iter_ge`](macro.assert_iter_ge.html),
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
/// * [`assert_iter_ge`](macro@crate::assert_iter_ge)
/// * [`assert_iter_ge`](macro@crate::assert_iter_ge)
/// * [`debug_assert_iter_ge`](macro@crate::debug_assert_iter_ge)
///
#[macro_export]
macro_rules! debug_assert_iter_ge {
    ($($arg:tt)*) => {
        if $crate::cfg!(debug_assertions) {
            $crate::assert_iter_ge!($($arg)*);
        }
    };
}
