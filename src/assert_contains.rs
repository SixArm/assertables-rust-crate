/// Assert an expression (such as a string) contains an expression (such as a substring).
///
/// * If true, return Result `Ok(())`.
///
/// * Otherwise, return Result `Err` with a diagnostic message.
///
/// This macro provides the same statements as [`assert_contains`],
/// except this macro returns a Result, rather than doing a panic.
///
/// This macro is useful for runtime checks, such as checking parameters,
/// or santizing inputs, or handling different results in different ways.
///
/// # Related
///
/// * [`assert_contains`]
/// * [`assert_contains_as_result`]
/// * [`debug_assert_contains`]
///
#[macro_export]
macro_rules! assert_contains_as_result {
    ($container:expr, $containee:expr $(,)?) => ({
        if $container.contains($containee) {
            Ok(())
        } else {
            Err(format!(
                concat!(
                    "assertion failed: `assert_contains!(container, containee)`\n",
                    " container label: `{}`,\n",
                    " container debug: `{:?}`,\n",
                    " containee label: `{}`,\n",
                    " containee debug: `{:?}`",
                ),
                stringify!($container), $container,
                stringify!($containee), $containee,
            ))
        }
    });
}

#[cfg(test)]
mod test_x_result {

    #[test]
    fn test_assert_contains_as_result_x_success() {
        let a = "foogoo";
        let b = "oo";
        let x = assert_contains_as_result!(a, b);
        assert_eq!(x.unwrap(), ());
    }

    #[test]
    fn test_assert_contains_as_result_x_failure() {
        let a = Regex::new(r"foogoo").unwrap();
        let b = "zz";
        let x = assert_contains_as_result!(a, b);
        let actual = x.unwrap_err();
        let expect = concat!(
            "assertion failed: `assert_contains!(container, containee)`\n",
            " container label: `a`,\n",
            " container debug: `\"foogoo\"`,\n",
            " containee label: `b`,\n",
            " containee debug: `\"zz\"`"
        );
        assert_eq!(actual, expect);
    }
}

/// Assert a container is a match for an expression.
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
/// // Return Ok
/// let a = "foogoo";
/// let b = "oo";
/// assert_contains!(a, b);
/// //-> ()
///
/// // Panic with error message
/// let result = panic::catch_unwind(|| {
/// let a = "foogoo";
/// let b = "zz";
/// assert_contains!(a, b);
/// //-> panic!
/// });
/// assert!(result.is_err());
/// let actual = result.unwrap_err().downcast::<String>().unwrap().to_string();
/// let expect = concat!(
///     "assertion failed: `assert_contains!(container, containee)`\n",
///     " container label: `a`,\n",
///     " container debug: `\"foogoo\"`,\n",
///     " containee label: `b`,\n",
///     " containee debug: `\"zz\"`"
/// );
/// assert_eq!(actual, expect);
/// # }
/// ```
///
/// # Related
///
/// * [`assert_contains`]
/// * [`assert_contains_as_result`]
/// * [`debug_assert_contains`]
///
#[macro_export]
macro_rules! assert_contains {
    ($container:expr, $containee:expr $(,)?) => ({
        match assert_contains_as_result!($container, $containee) {
            Ok(()) => (),
            Err(err) => panic!("{}", err),
        }
    });
    ($container:expr, $containee:expr, $($message:tt)+) => ({
        match assert_contains_as_result!($container, $containee) {
            Ok(()) => (),
            Err(_err) => panic!("{}", $($message)+),
        }
    });
}

/// Assert a container is a match for an expression.
///
/// This macro provides the same statements as [`assert_contains`],
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
/// This macro is intendend to work in a similar way to
/// [`std::debug_assert`](https://doc.rust-lang.org/std/macro.debug_assert.html).
///
/// # Related
///
/// * [`assert_contains`]
/// * [`assert_contains`]
/// * [`debug_assert_contains`]
///
#[macro_export]
macro_rules! debug_assert_contains {
    ($($arg:tt)*) => {
        if $crate::cfg!(debug_assertions) {
            $crate::assert_contains!($($arg)*);
        }
    };
}
