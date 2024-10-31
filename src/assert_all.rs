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
//! # fn main() {
//! let a = [1, 2, 3];
//! assert_all!(a.into_iter(), |x: i8| x > 0);
//! # }
//! ```
//!
//! This implementation uses [`::std::iter::Iterator`](https://doc.rust-lang.org/std/iter/trait.Iterator.html).
//!
//! # Module macros
//!
//! * [`assert_all`](macro@crate::assert_all)
//! * [`assert_all_as_result`](macro@crate::assert_all_as_result)
//! * [`debug_assert_all`](macro@crate::debug_assert_all)

/// Assert every element of the iterator matches a predicate.
///
/// Pseudocode:<br>
/// collection into iter ∀ predicate
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
/// * [`assert_all`](macro@crate::assert_all)
/// * [`assert_all_as_result`](macro@crate::assert_all_as_result)
/// * [`debug_assert_all`](macro@crate::debug_assert_all)
///
#[macro_export]
macro_rules! assert_all_as_result {
    ($collection:expr, $predicate:expr $(,)?) => {{
        match (&$collection, &$predicate) {
            (collection, _predicate) => {
                if $collection.all($predicate) {
                    Ok(())
                } else {
                    Err(format!(
                        concat!(
                            "assertion failed: `assert_all!(collection, predicate)`\n",
                            "https://docs.rs/assertables/9.2.0/assertables/macro.assert_all.html\n",
                            " collection label: `{}`,\n",
                            " collection debug: `{:?}`,\n",
                            "        predicate: `{}`"
                        ),
                        stringify!($collection),
                        collection,
                        stringify!($predicate)
                    ))
                }
            }
        }
    }};
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_assert_all_as_result_x_success() {
        let a = [1, 2, 3];
        let result = assert_all_as_result!(a.into_iter(), |x: i8| x > 0);
        assert_eq!(result, Ok(()));
    }

    #[test]
    fn test_assert_all_as_result_x_failure() {
        let a = [1, -2, 3];
        let result = assert_all_as_result!(a.into_iter(), |x: i8| x > 0);
        assert_eq!(
            result.unwrap_err(),
            concat!(
                "assertion failed: `assert_all!(collection, predicate)`\n",
                "https://docs.rs/assertables/9.2.0/assertables/macro.assert_all.html\n",
                " collection label: `a.into_iter()`,\n",
                " collection debug: `IntoIter([1, -2, 3])`,\n",
                "        predicate: `|x: i8| x > 0`"
            )
        );
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
/// assert_all!(a.into_iter(), |x: i8| x > 0);
///
/// # let result = panic::catch_unwind(|| {
/// // This will panic
/// let a = [1, -2, 3];
/// assert_all!(a.into_iter(), |x: i8| x > 0);
/// # });
/// // assertion failed: `assert_all!(collection, predicate)`
/// // https://docs.rs/assertables/9.2.0/assertables/macro.assert_all.html
/// //  collection label: `a.into_iter()`,
/// //  collection debug: `IntoIter([1, -2, 3])`,
/// //         predicate: `|x: i8| x > 0`
/// # let actual = result.unwrap_err().downcast::<String>().unwrap().to_string();
/// # let expect = concat!(
/// #     "assertion failed: `assert_all!(collection, predicate)`\n",
/// #     "https://docs.rs/assertables/9.2.0/assertables/macro.assert_all.html\n",
/// #     " collection label: `a.into_iter()`,\n",
/// #     " collection debug: `IntoIter([1, -2, 3])`,\n",
/// #     "        predicate: `|x: i8| x > 0`",
/// # );
/// # assert_eq!(actual, expect);
/// # }
/// ```
///
/// This implementation uses [`::std::iter::Iterator`](https://doc.rust-lang.org/std/iter/trait.Iterator.html).
///
/// # Module macros
///
/// * [`assert_all`](macro@crate::assert_all)
/// * [`assert_all_as_result`](macro@crate::assert_all_as_result)
/// * [`debug_assert_all`](macro@crate::debug_assert_all)
///
#[macro_export]
macro_rules! assert_all {
    ($collection:expr, $predicate:expr $(,)?) => {{
        match $crate::assert_all_as_result!($collection, $predicate) {
            Ok(()) => (),
            Err(err) => panic!("{}", err),
        }
    }};
    ($collection:expr, $predicate:expr, $($message:tt)+) => {{
        match $crate::assert_all_as_result!($collection, $predicate) {
            Ok(()) => (),
            Err(_err) => panic!("{}", $($message)+),
        }
    }};
}

/// Assert every element of the iterator matches a predicate.
///
/// Pseudocode:<br>
/// collection into iter ∀ predicate
///
/// This macro provides the same statements as [`assert_all`](macro.assert_all.html),
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
/// * [`assert_all`](macro@crate::assert_all)
/// * [`assert_all`](macro@crate::assert_all)
/// * [`debug_assert_all`](macro@crate::debug_assert_all)
///
#[macro_export]
macro_rules! debug_assert_all {
    ($($arg:tt)*) => {
        if $crate::cfg!(debug_assertions) {
            $crate::assert_all!($($arg)*);
        }
    };
}
