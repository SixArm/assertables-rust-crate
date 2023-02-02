/// Assert a std::io::Read read_to_string() is not equal to another.
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
/// * [`assert_read_to_string_ne`]
/// * [`assert_read_to_string_ne_as_result`]
/// * [`debug_assert_read_to_string_ne`]
///
#[macro_export]
macro_rules! assert_read_to_string_ne_as_result {
    ($a_reader:expr, $b_reader:expr $(,)?) => ({
        let mut a_string = String::new();
        let mut b_string = String::new();
        let a_result = $a_reader.read_to_string(&mut a_string);
        let b_result = $b_reader.read_to_string(&mut b_string);
        if a_result.is_err() || b_result.is_err() {
            Err(format!(
                concat!(
                    "assertion failed: `assert_read_to_string_ne!(left_reader, right_reader)`\n",
                    "  left_reader label: `{}`,\n",
                    "  left_reader debug: `{:?}`,\n",
                    " right_reader label: `{}`,\n",
                    " right_reader debug: `{:?}`,\n",
                    "        left result: `{:?}`,\n",
                    "       right result: `{:?}`"
                ),
                stringify!($a_reader), $a_reader,
                stringify!($b_reader), $b_reader,
                a_result,
                b_result
            ))
        } else {
            let _a_size = a_result.unwrap();
            let _b_size = b_result.unwrap();
            if a_string != b_string {
                Ok(())
            } else {
                Err(format!(
                    concat!(
                        "assertion failed: `assert_read_to_string_ne!(left_reader, right_reader)`\n",
                        "  left_reader label: `{}`,\n",
                        "  left_reader debug: `{:?}`,\n",
                        " right_reader label: `{}`,\n",
                        " right_reader debug: `{:?}`,\n",
                        "               left: `{:?}`,\n",
                        "              right: `{:?}`"
                    ),
                    stringify!($a_reader), $a_reader,
                    stringify!($b_reader), $b_reader,
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
    fn test_assert_read_to_string_ne_as_result_x_success() {
        let mut a = "alpha".as_bytes();
        let mut b = "bravo".as_bytes();
        let x = assert_read_to_string_ne_as_result!(a, b);
        assert!(x.is_ok());
        assert_eq!(x, Ok(()));
    }

    #[test]
    fn test_assert_read_to_string_ne_as_result_x_failure() {
        let mut a = "alpha".as_bytes();
        let mut b = "alpha".as_bytes();
        let x = assert_read_to_string_ne_as_result!(a, b);
        assert!(x.is_err());
        assert_eq!(
            x.unwrap_err(),
            concat!(
                "assertion failed: `assert_read_to_string_ne!(left_reader, right_reader)`\n",
                "  left_reader label: `a`,\n",
                "  left_reader debug: `[]`,\n",
                " right_reader label: `b`,\n",
                " right_reader debug: `[]`,\n",
                "               left: `\"alpha\"`,\n",
                "              right: `\"alpha\"`"
            )
        );
    }
}

/// Assert a std::io::Read read_to_string() is not equal to another.
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
/// let mut a = "alpha".as_bytes();
/// let mut b = "bravo".as_bytes();
/// assert_read_to_string_ne!(a, b);
/// //-> ()
///
/// // Panic with error message
/// let result = panic::catch_unwind(|| {
/// let mut a = "alpha".as_bytes();
/// let mut b = "alpha".as_bytes();
/// assert_read_to_string_ne!(a, b);
/// //-> panic!
/// });
/// assert!(result.is_err());
/// let actual = result.unwrap_err().downcast::<String>().unwrap().to_string();
/// let expect = concat!(
///     "assertion failed: `assert_read_to_string_ne!(left_reader, right_reader)`\n",
///     "  left_reader label: `a`,\n",
///     "  left_reader debug: `[]`,\n",
///     " right_reader label: `b`,\n",
///     " right_reader debug: `[]`,\n",
///     "               left: `\"alpha\"`,\n",
///     "              right: `\"alpha\"`"
/// );
/// assert_eq!(actual, expect);
/// # }
/// ```
///
/// # Related
///
/// * [`assert_read_to_string_ne`]
/// * [`assert_read_to_string_ne_as_result`]
/// * [`debug_assert_read_to_string_ne`]
///
#[macro_export]
macro_rules! assert_read_to_string_ne {
    ($a_reader:expr, $b_reader:expr $(,)?) => ({
        match assert_read_to_string_ne_as_result!($a_reader, $b_reader) {
            Ok(()) => (),
            Err(err) => panic!("{}", err),
        }
    });
    ($a_reader:expr, $b_reader:expr, $($message:tt)+) => ({
        match assert_read_to_string_ne_as_result!($a_reader, $b_reader) {
            Ok(()) => (),
            Err(_err) => panic!("{}", $($message)+),
        }
    });
}

/// Assert a std::io::Read read_to_string() is not equal to another.
///
/// This macro provides the same statements as [`assert_read_to_string_ne`],
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
/// * [`assert_read_to_string_ne`]
/// * [`assert_read_to_string_ne`]
/// * [`debug_assert_read_to_string_ne`]
///
#[macro_export]
macro_rules! debug_assert_read_to_string_ne {
    ($($arg:tt)*) => {
        if $crate::cfg!(debug_assertions) {
            $crate::assert_read_to_string_ne!($($arg)*);
        }
    };
}
