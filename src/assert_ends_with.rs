/// Assert an expression (such as a string) ends with an expression (such as a substring).
///
/// * If true, return Result `Ok(())`.
///
/// * Otherwise, return Result `Err` with a diagnostic message.
///
/// This macro provides the same statements as [`assert_ends_with`],
/// except this macro returns a Result, rather than doing a panic.
///
/// This macro is useful for runtime checks, such as checking parameters,
/// or sanitizing inputs, or handling different results in different ways.
///
/// # Module macros
///
/// * [`assert_ends_with`](macro.assert_ends_with.html)
/// * [`assert_ends_with_as_result`](macro.assert_ends_with_as_result.html)
/// * [`debug_assert_ends_with`](macro.debug_assert_ends_with.html)
///
#[macro_export]
macro_rules! assert_ends_with_as_result {
    ($a:expr, $b:expr $(,)?) => {{
        if $a.ends_with($b) {
            Ok(())
        } else {
            Err(format!(
                concat!(
                    "assertion failed: `assert_ends_with!(a, b)`\n",
                    " a label: `{}`,\n",
                    " a debug: `{:?}`,\n",
                    " b label: `{}`,\n",
                    " b debug: `{:?}`",
                ),
                stringify!($a),
                $a,
                stringify!($b),
                $b,
            ))
        }
    }};
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_assert_ends_with_as_result_x_success() {
        let a = "foogoo";
        let b = "goo";
        let x = assert_ends_with_as_result!(a, b);
        assert_eq!(x.unwrap(), ());
    }

    #[test]
    fn test_assert_ends_with_as_result_x_failure() {
        let a = "foogoo";
        let b = "foo";
        let x = assert_ends_with_as_result!(a, b);
        let actual = x.unwrap_err();
        let expect = concat!(
            "assertion failed: `assert_ends_with!(a, b)`\n",
            " a label: `a`,\n",
            " a debug: `\"foogoo\"`,\n",
            " b label: `b`,\n",
            " b debug: `\"foo\"`"
        );
        assert_eq!(actual, expect);
    }
}

/// Assert an expression (such as a string) ends with an expression (such as a string).
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
/// let b = "goo";
/// assert_ends_with!(a, b);
/// //-> ()
///
/// // Panic with error message
/// let result = panic::catch_unwind(|| {
/// let a = "foogoo";
/// let b = "foo";
/// assert_ends_with!(a, b);
/// //-> panic!
/// });
/// assert!(result.is_err());
/// let actual = result.unwrap_err().downcast::<String>().unwrap().to_string();
/// let expect = concat!(
///     "assertion failed: `assert_ends_with!(a, b)`\n",
///     " a label: `a`,\n",
///     " a debug: `\"foogoo\"`,\n",
///     " b label: `b`,\n",
///     " b debug: `\"foo\"`"
/// );
/// assert_eq!(actual, expect);
/// # }
/// ```
///
/// # Module macros
///
/// * [`assert_ends_with`](macro.assert_ends_with.html)
/// * [`assert_ends_with_as_result`](macro.assert_ends_with_as_result.html)
/// * [`debug_assert_ends_with`](macro.debug_assert_ends_with.html)
///
#[macro_export]
macro_rules! assert_ends_with {
    ($a:expr, $b:expr $(,)?) => ({
        match assert_ends_with_as_result!($a, $b) {
            Ok(()) => (),
            Err(err) => panic!("{}", err),
        }
    });
    ($a:expr, $b:expr, $($message:tt)+) => ({
        match assert_ends_with_as_result!($a, $b) {
            Ok(()) => (),
            Err(_err) => panic!("{}", $($message)+),
        }
    });
}

/// Assert an expression (such as a string) ends with an expression (such as a string).
///
/// This macro provides the same statements as [`assert_ends_with`],
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
/// # Module macros
///
/// * [`assert_ends_with`](macro.assert_ends_with.html)
/// * [`assert_ends_with`](macro.assert_ends_with.html)
/// * [`debug_assert_ends_with`](macro.debug_assert_ends_with.html)
///
#[macro_export]
macro_rules! debug_assert_ends_with {
    ($($arg:tt)*) => {
        if $crate::cfg!(debug_assertions) {
            $crate::assert_ends_with!($($arg)*);
        }
    };
}
