//! Assert an item is in a container.
//!
//! Pseudocode:<br>
//! a is in container
//!
//! # Example
//!
//! ```rust
//! use assertables::*;
//!
//! let a = 1;
//! let b = 0..2;
//! assert_in!(a, b);
//! ```
//!
//! # Module macros
//!
//! * [`assert_in`](macro@crate::assert_in)
//! * [`assert_in_as_result`](macro@crate::assert_in_as_result)
//! * [`debug_assert_in`](macro@crate::debug_assert_in)

/// Assert an item is in a container.
///
/// Pseudocode:<br>
/// a is in container
///
/// * If true, return Result `Ok(())`.
///
/// * Otherwise, return Result `Err(message)`.
///
/// This macro is useful for runtime checks, such as checking parameters,
/// or sanitizing inputs, or handling different results in different ways.
///
/// # Module macros
///
/// * [`assert_in`](macro@crate::assert_in)
/// * [`assert_in_as_result`](macro@crate::assert_in_as_result)
/// * [`debug_assert_in`](macro@crate::debug_assert_in)
///
#[macro_export]
macro_rules! assert_in_as_result {
    ($a:expr, $container:expr $(,)?) => {{
        if $container.contains(&$a) {
            Ok(())
        } else {
            Err(format!(
                concat!(
                    "assertion failed: `assert_in!(a, container)`\n",
                    "https://docs.rs/assertables/9.5.1/assertables/macro.assert_in.html\n",
                    "         a label: `{}`,\n",
                    "         a debug: `{:?}`,\n",
                    " container label: `{}`,\n",
                    " container debug: `{:?}`",
                ),
                stringify!($a),
                $a,
                stringify!($container),
                $container,
            ))
        }
    }};
}

#[cfg(test)]
mod test_assert_in_as_result {

    #[test]
    fn success() {
        let a = 1;
        let b = 0..2;
        let actual = assert_in_as_result!(a, b);
        assert_eq!(actual.unwrap(), ());
    }

    #[test]
    fn failure() {
        let a = 1;
        let b = 2..4;
        let actual = assert_in_as_result!(a, b);
        let message = concat!(
            "assertion failed: `assert_in!(a, container)`\n",
            "https://docs.rs/assertables/9.5.1/assertables/macro.assert_in.html\n",
            "         a label: `a`,\n",
            "         a debug: `1`,\n",
            " container label: `b`,\n",
            " container debug: `2..4`"
        );
        assert_eq!(actual.unwrap_err(), message);
    }
}

/// Assert an item is in a container.
///
/// Pseudocode:<br>
/// a is in container
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
/// let a = 1;
/// let b = 0..2;
/// assert_in!(a, b);
///
/// # let result = panic::catch_unwind(|| {
/// // This will panic
/// let a = 1;
/// let b = 2..4;
/// assert_in!(a, b);
/// # });
/// // assertion failed: `assert_in!(a, container)`
/// // https://docs.rs/assertables/9.5.1/assertables/macro.assert_in.html
/// //  a label: `a`,
/// //  a debug: `1`,
/// //  container label: `b`,
/// //  container debug: `2..4`
/// # let actual = result.unwrap_err().downcast::<String>().unwrap().to_string();
/// # let message = concat!(
/// #     "assertion failed: `assert_in!(a, container)`\n",
/// #     "https://docs.rs/assertables/9.5.1/assertables/macro.assert_in.html\n",
/// #     "         a label: `a`,\n",
/// #     "         a debug: `1`,\n",
/// #     " container label: `b`,\n",
/// #     " container debug: `2..4`"
/// # );
/// # assert_eq!(actual, message);
/// # }
/// ```
///
/// # Module macros
///
/// * [`assert_in`](macro@crate::assert_in)
/// * [`assert_in_as_result`](macro@crate::assert_in_as_result)
/// * [`debug_assert_in`](macro@crate::debug_assert_in)
///
#[macro_export]
macro_rules! assert_in {
    ($a:expr, $container:expr $(,)?) => {{
        match $crate::assert_in_as_result!($a, $container) {
            Ok(()) => (),
            Err(err) => panic!("{}", err),
        }
    }};
    ($a:expr, $container:expr, $($message:tt)+) => {{
        match $crate::assert_in_as_result!($a, $container) {
            Ok(()) => (),
            Err(err) => panic!("{}\n{}", format_args!($($message)+), err),
        }
    }};
}

#[cfg(test)]
mod test_assert_in {
    use std::panic;

    #[test]
    fn success() {
        let a = 1;
        let b = 0..2;
        let actual = assert_in!(a, b);
        assert_eq!(actual, ());
    }

    #[test]
    fn failure() {
        let a = 1;
        let b = 2..4;
        let result = panic::catch_unwind(|| {
            let _actual = assert_in!(a, b);
        });
        let message = concat!(
            "assertion failed: `assert_in!(a, container)`\n",
            "https://docs.rs/assertables/9.5.1/assertables/macro.assert_in.html\n",
            "         a label: `a`,\n",
            "         a debug: `1`,\n",
            " container label: `b`,\n",
            " container debug: `2..4`"
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

/// Assert an item is in a container.
///
/// Pseudocode:<br>
/// a is in container
///
/// This macro provides the same statements as [`assert_in`](macro.assert_in.html),
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
/// * [`assert_in`](macro@crate::assert_in)
/// * [`assert_in`](macro@crate::assert_in)
/// * [`debug_assert_in`](macro@crate::debug_assert_in)
///
#[macro_export]
macro_rules! debug_assert_in {
    ($($arg:tt)*) => {
        if $crate::cfg!(debug_assertions) {
            $crate::assert_in!($($arg)*);
        }
    };
}
