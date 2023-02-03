/// Assert a std::io::Read read_to_string() contains a pattern.
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
/// * [`assert_read_to_string_contains`]
/// * [`assert_read_to_string_contains_as_result`]
/// * [`debug_assert_read_to_string_contains`]
///
#[macro_export]
macro_rules! assert_read_to_string_contains_as_result {
    ($a_reader:expr, $b_containee:expr $(,)?) => ({
        let mut a_string = String::new();
        let a_result = $a_reader.read_to_string(&mut a_string);
        if let Err(a_err) = a_result {
            Err(format!(
                concat!(
                    "assertion failed: `assert_read_to_string_contains!(left_reader, right_containee)`\n",
                    "     left_reader label: `{}`,\n",
                    "     left_reader debug: `{:?}`,\n",
                    " right_containee label: `{}`,\n",
                    " right_containee debug: `{:?}`,\n",
                    "              left err: `{:?}`"
                ),
                stringify!($a_reader), $a_reader,
                stringify!($b_containee), $b_containee,
                a_err
            ))
        } else {
            if a_string.contains($b_containee) {
                Ok(())
            } else {
                let _a_size = a_result.unwrap();
                Err(format!(
                    concat!(
                        "assertion failed: `assert_read_to_string_contains!(left_reader, right_containee)`\n",
                        "     left_reader label: `{}`,\n",
                        "     left_reader debug: `{:?}`,\n",
                        " right_containee label: `{}`,\n",
                        " right_containee debug: `{:?}`,\n",
                        "                  left: `{:?}`,\n",
                        "                 right: `{:?}`",
                    ),
                    stringify!($a_reader), $a_reader,
                    stringify!($b_containee), $b_containee,
                    a_string,
                    $b_containee
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
    fn test_assert_read_to_string_contains_as_result_x_success() {
        let mut reader = "alpha".as_bytes();
        let containee = "lph";
        let x = assert_read_to_string_contains_as_result!(reader, containee);
        assert!(x.is_ok());
        assert_eq!(x, Ok(()));
    }

    #[test]
    fn test_assert_read_to_string_contains_as_result_x_failure() {
        let mut reader = "alpha".as_bytes();
        let containee = "zzz";
        let x = assert_read_to_string_contains_as_result!(reader, containee);
        assert!(x.is_err());
        assert_eq!(
            x.unwrap_err(),
            concat!(
                "assertion failed: `assert_read_to_string_contains!(left_reader, right_containee)`\n",
                "     left_reader label: `reader`,\n",
                "     left_reader debug: `[]`,\n",
                " right_containee label: `containee`,\n",
                " right_containee debug: `\"zzz\"`,\n",
                "                  left: `\"alpha\"`,\n",
                "                 right: `\"zzz\"`"
            )
        );
    }
}

/// Assert a std::io::Read read_to_string() contains a pattern.
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
/// let mut reader = "hello".as_bytes();
/// let containee = "ell";
/// assert_read_to_string_contains!(reader, containee);
/// //-> ()
///
/// // Panic with error message
/// let result = panic::catch_unwind(|| {
/// let mut reader = "hello".as_bytes();
/// let containee = "zzz";
/// assert_read_to_string_contains!(reader, containee);
/// //-> panic!
/// });
/// assert!(result.is_err());
/// let actual = result.unwrap_err().downcast::<String>().unwrap().to_string();
/// let expect = concat!(
///     "assertion failed: `assert_read_to_string_contains!(left_reader, right_containee)`\n",
///     "     left_reader label: `reader`,\n",
///     "     left_reader debug: `[]`,\n",
///     " right_containee label: `containee`,\n",
///     " right_containee debug: `\"zzz\"`,\n",
///     "                  left: `\"hello\"`,\n",
///     "                 right: `\"zzz\"`"
/// );
/// assert_eq!(actual, expect);
/// # }
/// ```
///
/// # Related
///
/// * [`assert_read_to_string_contains`]
/// * [`assert_read_to_string_contains_as_result`]
/// * [`debug_assert_read_to_string_contains`]
///
#[macro_export]
macro_rules! assert_read_to_string_contains {
    ($a_reader:expr, $b:expr $(,)?) => ({
        match assert_read_to_string_contains_as_result!($a_reader, $b) {
            Ok(()) => (),
            Err(err) => panic!("{}", err),
        }
    });
    ($a_reader:expr, $b:expr, $($message:tt)+) => ({
        match assert_read_to_string_contains_as_result!($a_reader, $b) {
            Ok(()) => (),
            Err(_err) => panic!("{}", $($message)+),
        }
    });
}

/// Assert a std::io::Read read_to_string() contains a pattern.
///
/// This macro provides the same statements as [`assert_read_to_string_contains`],
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
/// * [`assert_read_to_string_contains`]
/// * [`assert_read_to_string_contains`]
/// * [`debug_assert_read_to_string_contains`]
///
#[macro_export]
macro_rules! debug_assert_read_to_string_contains {
    ($($arg:tt)*) => {
        if $crate::cfg!(debug_assertions) {
            $crate::assert_read_to_string_contains!($($arg)*);
        }
    };
}
