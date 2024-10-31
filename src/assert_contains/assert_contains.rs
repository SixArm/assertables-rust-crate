//! Assert a container is a match for an expression.
//!
//! Pseudocode:<br>
//! a.contains(b)
//!
//! # Example
//!
//! ```rust
//! use assertables::*;
//!
//! # fn main() {
//! // String contains substring
//! let a: &str = "alfa";
//! let b: &str = "lf";
//! assert_contains!(a, b);
//!
//! // Range contains value
//! let a = 1..3;
//! let b = 2;
//! assert_contains!(a, &b);
//!
//! // Vector contains element
//! let a = vec![1, 2, 3];
//! let b = 2;
//! assert_contains!(a, &b);
//! # }
//! ```
//!
//! # Module macros
//!
//! * [`assert_contains`](macro@crate::assert_contains)
//! * [`assert_contains_as_result`](macro@crate::assert_contains_as_result)
//! * [`debug_assert_contains`](macro@crate::debug_assert_contains)

/// Assert an expression (such as a string) contains an expression (such as a substring).
///
/// Pseudocode:<br>
/// a.contains(b)
///
/// * If true, return Result `Ok(())`.
///
/// * Otherwise, return Result `Err(message)`.
///
/// This macro provides the same statements as [`assert_contains`](macro.assert_contains.html),
/// except this macro returns a Result, rather than doing a panic.
///
/// This macro is useful for runtime checks, such as checking parameters,
/// or sanitizing inputs, or handling different results in different ways.
///
/// # Module macros
///
/// * [`assert_contains`](macro@crate::assert_contains)
/// * [`assert_contains_as_result`](macro@crate::assert_contains_as_result)
/// * [`debug_assert_contains`](macro@crate::debug_assert_contains)
///
#[macro_export]
macro_rules! assert_contains_as_result {
    ($container:expr, $containee:expr $(,)?) => {{
        match (&$container, &$containee) {
            (container, containee) => {
                if container.contains($containee) {
                    Ok(())
                } else {
                    Err(
                        format!(
                            concat!(
                                "assertion failed: `assert_contains!(container, containee)`\n",
                                "https://docs.rs/assertables/9.2.0/assertables/macro.assert_contains.html\n",
                                " container label: `{}`,\n",
                                " container debug: `{:?}`,\n",
                                " containee label: `{}`,\n",
                                " containee debug: `{:?}`",
                            ),
                            stringify!($container),
                            container,
                            stringify!($containee),
                            containee,
                        )
                    )
                }
            }
        }
    }};
}

#[cfg(test)]
mod tests {

    mod str {

        #[test]
        fn success() {
            let a = "alfa";
            let b = "lf";
            let result = assert_contains_as_result!(a, b);
            assert_eq!(result.unwrap(), ());
        }

        #[test]
        fn failure() {
            let a: &str = "alfa";
            let b = "zz";
            let result = assert_contains_as_result!(a, b);
            let actual = result.unwrap_err();
            let expect = concat!(
                "assertion failed: `assert_contains!(container, containee)`\n",
                "https://docs.rs/assertables/9.2.0/assertables/macro.assert_contains.html\n",
                " container label: `a`,\n",
                " container debug: `\"alfa\"`,\n",
                " containee label: `b`,\n",
                " containee debug: `\"zz\"`"
            );
            assert_eq!(actual, expect);
        }
    }

    mod range {

        #[test]
        fn success() {
            let a = 1..3;
            let b = 2;
            let result = assert_contains_as_result!(a, &b);
            assert_eq!(result.unwrap(), ());
        }

        #[test]
        fn failure() {
            let a = 1..3;
            let b = 4;
            let result = assert_contains_as_result!(a, &b);
            let actual = result.unwrap_err();
            let expect = concat!(
                "assertion failed: `assert_contains!(container, containee)`\n",
                "https://docs.rs/assertables/9.2.0/assertables/macro.assert_contains.html\n",
                " container label: `a`,\n",
                " container debug: `1..3`,\n",
                " containee label: `&b`,\n",
                " containee debug: `4`"
            );
            assert_eq!(actual, expect);
        }
    }

    mod vec {

        #[test]
        fn success() {
            let a = 1..3;
            let b = 2;
            let result = assert_contains_as_result!(a, &b);
            assert_eq!(result.unwrap(), ());
        }

        #[test]
        fn failure() {
            let a = vec![1, 2, 3];
            let b = 4;
            let result = assert_contains_as_result!(a, &b);
            let actual = result.unwrap_err();
            let expect = concat!(
                "assertion failed: `assert_contains!(container, containee)`\n",
                "https://docs.rs/assertables/9.2.0/assertables/macro.assert_contains.html\n",
                " container label: `a`,\n",
                " container debug: `[1, 2, 3]`,\n",
                " containee label: `&b`,\n",
                " containee debug: `4`"
            );
            assert_eq!(actual, expect);
        }
    }
}

/// Assert a container is a match for an expression.
///
/// Pseudocode:<br>
/// a.contains(b)
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
/// // Return Ok when a string contains a substring
/// let a = "alfa";
/// let b = "lf";
/// assert_contains!(a, b);
///
/// // Return Ok when a range contains a value
/// let a = 1..3;
/// let b = 2;
/// assert_contains!(a, &b);
///
/// # let result = panic::catch_unwind(|| {
/// // This will panic
/// let a = "alfa";
/// let b = "zz";
/// assert_contains!(a, b);
/// # });
/// // assertion failed: `assert_contains!(container, containee)`
/// // https://docs.rs/assertables/9.2.0/assertables/macro.assert_contains.html
/// //  container label: `a`,
/// //  container debug: `\"alfa\"`,
/// //  containee label: `b`,
/// //  containee debug: `\"zz\"`
/// # let actual = result.unwrap_err().downcast::<String>().unwrap().to_string();
/// # let expect = concat!(
/// #     "assertion failed: `assert_contains!(container, containee)`\n",
/// #     "https://docs.rs/assertables/9.2.0/assertables/macro.assert_contains.html\n",
/// #     " container label: `a`,\n",
/// #     " container debug: `\"alfa\"`,\n",
/// #     " containee label: `b`,\n",
/// #     " containee debug: `\"zz\"`"
/// # );
/// # assert_eq!(actual, expect);
/// # }
/// ```
///
/// # Module macros
///
/// * [`assert_contains`](macro@crate::assert_contains)
/// * [`assert_contains_as_result`](macro@crate::assert_contains_as_result)
/// * [`debug_assert_contains`](macro@crate::debug_assert_contains)
///
#[macro_export]
macro_rules! assert_contains {
    ($container:expr, $containee:expr $(,)?) => {{
        match $crate::assert_contains_as_result!($container, $containee) {
            Ok(()) => (),
            Err(err) => panic!("{}", err),
        }
    }};
    ($container:expr, $containee:expr, $($message:tt)+) => {{
        match $crate::assert_contains_as_result!($container, $containee) {
            Ok(()) => (),
            Err(_err) => panic!("{}", $($message)+),
        }
    }};
}

/// Assert a container is a match for an expression.
///
/// Pseudocode:<br>
/// a.contains(b)
///
/// This macro provides the same statements as [`assert_contains`](macro.assert_contains.html),
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
/// * [`assert_contains`](macro@crate::assert_contains)
/// * [`assert_contains`](macro@crate::assert_contains)
/// * [`debug_assert_contains`](macro@crate::debug_assert_contains)
///
#[macro_export]
macro_rules! debug_assert_contains {
    ($($arg:tt)*) => {
        if $crate::cfg!(debug_assertions) {
            $crate::assert_contains!($($arg)*);
        }
    };
}
