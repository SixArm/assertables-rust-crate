//! Assert every element of the iterator matches a predicate.
//!
//! Pseudocode:<br>
//! collection into iter ∀ predicate
//!
//! # Example
//!
//! ```rust
//! use assertables::*;
//!
//! let a = [1, 2, 3];
//! assert_any!(a.into_iter(), |x: i8| x > 0);
//! ```
//!
//! This implementation uses [`::std::iter::Iterator`](https://doc.rust-lang.org/std/iter/trait.Iterator.html).
//!
//! # Module macros
//!
//! * [`assert_any`](macro@crate::assert_any)
//! * [`assert_any_as_result`](macro@crate::assert_any_as_result)
//! * [`debug_assert_any`](macro@crate::debug_assert_any)

/// Assert every element of the iterator matches a predicate.
///
/// Pseudocode:<br>
/// collection into iter ∀ predicate
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
/// * [`assert_any`](macro@crate::assert_any)
/// * [`assert_any_as_result`](macro@crate::assert_any_as_result)
/// * [`debug_assert_any`](macro@crate::debug_assert_any)
///
#[macro_export]
macro_rules! assert_any_as_result {
    ($collection:expr, $predicate:expr $(,)?) => {
        match ($collection, $predicate) {
            (mut collection, predicate) => {
                if collection.any(predicate) {
                    Ok(())
                } else {
                    Err(format!(
                        concat!(
                            "assertion failed: `assert_any!(collection, predicate)`\n",
                            "https://docs.rs/assertables/9.8.1/assertables/macro.assert_any.html\n",
                            " collection label: `{}`,\n",
                            " collection debug: `{:?}`,\n",
                            "        predicate: `{}`",
                        ),
                        stringify!($collection),
                        $collection,
                        stringify!($predicate)
                    ))
                }
            }
        }
    };
}

#[cfg(test)]
mod test_assert_any_as_result {
    use std::sync::Once;

    #[test]
    fn success() {
        let a = [1, 2, 3];
        for _ in 0..1 {
            let actual = assert_any_as_result!(a.into_iter(), |x: i8| x > 0);
            assert_eq!(actual.unwrap(), ());
        }
    }

    #[test]
    fn success_once() {
        static A: Once = Once::new();
        fn a() -> [i8; 3] {
            if A.is_completed() {
                panic!("A.is_completed()")
            } else {
                A.call_once(|| {})
            }
            [1, 2, 3]
        }

        assert_eq!(A.is_completed(), false);
        let result = assert_any_as_result!(a().into_iter(), |x: i8| x > 0);
        assert!(result.is_ok());
        assert_eq!(A.is_completed(), true);
    }

    #[test]
    fn failure() {
        let a = [1, 2, 3];
        let actual = assert_any_as_result!(a.into_iter(), |x: i8| x > 3);
        let message = concat!(
            "assertion failed: `assert_any!(collection, predicate)`\n",
            "https://docs.rs/assertables/9.8.1/assertables/macro.assert_any.html\n",
            " collection label: `a.into_iter()`,\n",
            " collection debug: `IntoIter([1, 2, 3])`,\n",
            "        predicate: `|x: i8| x > 3`"
        );
        assert_eq!(actual.unwrap_err(), message);
    }
}

/// Assert every element of the iterator matches a predicate.
///
/// Pseudocode:<br>
/// collection into iter ∀ predicate
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
/// let a = [1, 2, 3];
/// assert_any!(a.into_iter(), |x: i8| x > 0);
///
/// # let result = panic::catch_unwind(|| {
/// // This will panic
/// let a = [1, 2, 3];
/// assert_any!(a.into_iter(), |x: i8| x > 3);
/// # });
/// // assertion failed: `assert_any!(collection, predicate)`
/// // https://docs.rs/assertables/…/assertables/macro.assert_any.html
/// //  collection label: `a.into_iter()`,
/// //  collection debug: `IntoIter([1, 2, 3])`,
/// //         predicate: `|x: i8| x > 3`
/// # let actual = result.unwrap_err().downcast::<String>().unwrap().to_string();
/// # let message = concat!(
/// #     "assertion failed: `assert_any!(collection, predicate)`\n",
/// #     "https://docs.rs/assertables/9.8.1/assertables/macro.assert_any.html\n",
/// #     " collection label: `a.into_iter()`,\n",
/// #     " collection debug: `IntoIter([1, 2, 3])`,\n",
/// #     "        predicate: `|x: i8| x > 3`",
/// # );
/// # assert_eq!(actual, message);
/// # }
/// ```
///
/// This implementation uses [`::std::iter::Iterator`](https://doc.rust-lang.org/std/iter/trait.Iterator.html).
///
/// # Module macros
///
/// * [`assert_any`](macro@crate::assert_any)
/// * [`assert_any_as_result`](macro@crate::assert_any_as_result)
/// * [`debug_assert_any`](macro@crate::debug_assert_any)
///
#[macro_export]
macro_rules! assert_any {
    ($collection:expr, $predicate:expr $(,)?) => {
        match $crate::assert_any_as_result!($collection, $predicate) {
            Ok(()) => (),
            Err(err) => panic!("{}", err),
        }
    };
    ($collection:expr, $predicate:expr, $($message:tt)+) => {
        match $crate::assert_any_as_result!($collection, $predicate) {
            Ok(()) => (),
            Err(err) => panic!("{}\n{}", format_args!($($message)+), err),
        }
    };
}

#[cfg(test)]
mod test_assert_any {
    use std::panic;

    #[test]
    fn success() {
        let a = [1, 2, 3];
        for _ in 0..1 {
            let actual = assert_any!(a.into_iter(), |x: i8| x > 0);
            assert_eq!(actual, ());
        }
    }

    #[test]
    fn failure() {
        let a = [1, 2, 3];
        let result = panic::catch_unwind(|| {
            let _actual = assert_any!(a.into_iter(), |x: i8| x > 3);
        });
        let message = concat!(
            "assertion failed: `assert_any!(collection, predicate)`\n",
            "https://docs.rs/assertables/9.8.1/assertables/macro.assert_any.html\n",
            " collection label: `a.into_iter()`,\n",
            " collection debug: `IntoIter([1, 2, 3])`,\n",
            "        predicate: `|x: i8| x > 3`"
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

/// Assert every element of the iterator matches a predicate.
///
/// Pseudocode:<br>
/// collection into iter ∀ predicate
///
/// This macro provides the same statements as [`assert_any`](macro.assert_any.html),
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
/// * [`assert_any`](macro@crate::assert_any)
/// * [`assert_any`](macro@crate::assert_any)
/// * [`debug_assert_any`](macro@crate::debug_assert_any)
///
#[macro_export]
macro_rules! debug_assert_any {
    ($($arg:tt)*) => {
        if $crate::cfg!(debug_assertions) {
            $crate::assert_any!($($arg)*);
        }
    };
}
