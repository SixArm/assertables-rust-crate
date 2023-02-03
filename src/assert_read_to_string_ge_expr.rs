/// Assert a std::io::Read read_to_string() value is greater than or equal to an expression.
///
/// * If true, return Result `Ok(())`.
///
/// * Otherwise, return Result `Err` with a diagnostic message.
///
/// This macro provides the same statements as [`assert_`],
/// except this macro returns a Result, rather than doing a panic.
///
/// This macro is useful for runtime checks, such as checking parameters,
/// or santizing inputs, or handling different results in different ways.
///
/// # Related
///
/// * [`assert_read_to_string_ge_expr`]
/// * [`assert_read_to_string_ge_expr_as_result`]
/// * [`debug_assert_read_to_string_ge_expr`]
///
#[macro_export]
macro_rules! assert_read_to_string_ge_expr_as_result {
    ($a_reader:expr, $b_expr:expr $(,)?) => ({
        let mut a_string = String::new();
        let a_result = $a_reader.read_to_string(&mut a_string);
        if let Err(a_err) = a_result {
            Err(format!(
                concat!(
                    "assertion failed: `assert_read_to_string_ge_expr!(left_reader, right_expr)`\n",
                    " left_reader label: `{}`,\n",
                    " left_reader debug: `{:?}`,\n",
                    "  right_expr label: `{}`,\n",
                    "  right_expr debug: `{:?}`,\n",
                    "          left err: `{:?}`"
                ),
                stringify!($a_reader), $a_reader,
                stringify!($b_expr), $b_expr,
                a_err
            ))
        } else {
            let _a_size = a_result.unwrap();
            let b_string = String::from($b_expr);
            if a_string >= b_string {
                Ok(())
            } else {
                Err(format!(
                    concat!(
                        "assertion failed: `assert_read_to_string_ge_expr!(left_reader, right_expr)`\n",
                        " left_reader label: `{}`,\n",
                        " left_reader debug: `{:?}`,\n",
                        "  right_expr label: `{}`,\n",
                        "  right_expr debug: `{:?}`,\n",
                        "              left: `{:?}`,\n",
                        "             right: `{:?}`",
                    ),
                    stringify!($a_reader), $a_reader,
                    stringify!($b_expr), $b_expr,
                    a_string,
                    b_string
                ))
            }
        }
    });
}

#[cfg(test)]
mod test_x_result {
    #[allow(unused_imports)]
    use std::io::Read;

    #[test]
    fn test_assert_read_to_string_ge_expr_as_result_x_success() {
        let mut reader = "bravo".as_bytes();
        let value = String::from("alpha");
        let x = assert_read_to_string_ge_expr_as_result!(reader, &value);
        assert!(x.is_ok());
        assert_eq!(x, Ok(()));
    }

    #[test]
    fn test_assert_read_to_string_ge_expr_as_result_x_failure() {
        let mut reader = "alpha".as_bytes();
        let value = String::from("bravo");
        let x = assert_read_to_string_ge_expr_as_result!(reader, &value);
        assert!(x.is_err());
        assert_eq!(
            x.unwrap_err(),
            concat!(
                "assertion failed: `assert_read_to_string_ge_expr!(left_reader, right_expr)`\n",
                " left_reader label: `reader`,\n",
                " left_reader debug: `[]`,\n",
                "  right_expr label: `&value`,\n",
                "  right_expr debug: `\"bravo\"`,\n",
                "              left: `\"alpha\"`,\n",
                "             right: `\"bravo\"`"
            )
        );
    }
}

/// Assert a std::io::Read read_to_string() value is greater than or equal to an expression.
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
/// use std::io::Read;
///
/// # fn main() {
/// // Return Ok
/// let mut reader = "bravo".as_bytes();
/// let value = String::from("alpha");
/// assert_read_to_string_ge_expr!(reader, &value);
/// //-> ()
///
/// // Panic with error message
/// let result = panic::catch_unwind(|| {
/// let mut reader = "alpha".as_bytes();
/// let value = String::from("bravo");
/// assert_read_to_string_ge_expr!(reader, &value);
/// //-> panic!
/// });
/// assert!(result.is_err());
/// let actual = result.unwrap_err().downcast::<String>().unwrap().to_string();
/// let expect = concat!(
///     "assertion failed: `assert_read_to_string_ge_expr!(left_reader, right_expr)`\n",
///     " left_reader label: `reader`,\n",
///     " left_reader debug: `[]`,\n",
///     "  right_expr label: `&value`,\n",
///     "  right_expr debug: `\"bravo\"`,\n",
///     "              left: `\"alpha\"`,\n",
///     "             right: `\"bravo\"`"
/// );
/// assert_eq!(actual, expect);
/// # }
/// ```
///
/// # Related
///
/// * [`assert_read_to_string_ge_expr`]
/// * [`assert_read_to_string_ge_expr_as_result`]
/// * [`debug_assert_read_to_string_ge_expr`]
///
#[macro_export]
macro_rules! assert_read_to_string_ge_expr {
    ($a_reader:expr,  $b_expr:expr $(,)?) => ({
        match assert_read_to_string_ge_expr_as_result!($a_reader, $b_expr) {
            Ok(()) => (),
            Err(err) => panic!("{}", err),
        }
    });
    ($a_reader:expr, $b_expr:expr, $($message:tt)+) => ({
        match assert_read_to_string_ge_expr_as_result!($a_reader, $b_expr) {
            Ok(()) => (),
            Err(_err) => panic!("{}", $($message)+),
        }
    });
}

/// Assert zzz.
///
/// This macro provides the same statements as [`assert_read_to_string_ge_expr`],
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
/// * [`assert_read_to_string_ge_expr`]
/// * [`assert_read_to_string_ge_expr`]
/// * [`debug_assert_read_to_string_ge_expr`]
///
#[macro_export]
macro_rules! debug_assert_read_to_string_ge_expr {
    ($($arg:tt)*) => {
        if $crate::cfg!(debug_assertions) {
            $crate::assert_read_to_string_ge_expr!($($arg)*);
        }
    };
}
