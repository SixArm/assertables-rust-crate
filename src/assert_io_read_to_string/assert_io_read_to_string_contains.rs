//! Assert a std::io::Read read_to_string() contains a pattern.
//!
//! # Example
//!
//! ```rust
//! # #[macro_use] extern crate assertables;
//! use std::io::Read;
//!
//! # fn main() {
//! let mut reader = "hello".as_bytes();
//! let containee = "ell";
//! assert_io_read_to_string_contains!(reader, &containee);
//! # }
//! ```
//!
//! # Module macros
//!
//! * [`assert_io_read_to_string_contains`](macro@crate::assert_io_read_to_string_contains)
//! * [`assert_io_read_to_string_contains_as_result`](macro@crate::assert_io_read_to_string_contains_as_result)
//! * [`debug_assert_io_read_to_string_contains`](macro@crate::debug_assert_io_read_to_string_contains)

/// Assert a std::io::Read read_to_string() contains a pattern.
///
/// * If true, return Result `Ok(())`.
///
/// * Otherwise, return Result `Err` with a diagnostic message.
///
/// This macro provides the same statements as [`assert_`](macro.assert_.html),
/// except this macro returns a Result, rather than doing a panic.
///
/// This macro is useful for runtime checks, such as checking parameters,
/// or sanitizing inputs, or handling different results in different ways.
///
/// # Module macros
///
/// * [`assert_io_read_to_string_contains`](macro@crate::assert_io_read_to_string_contains)
/// * [`assert_io_read_to_string_contains_as_result`](macro@crate::assert_io_read_to_string_contains_as_result)
/// * [`debug_assert_io_read_to_string_contains`](macro@crate::debug_assert_io_read_to_string_contains)
///
#[macro_export]
macro_rules! assert_io_read_to_string_contains_as_result {
    ($reader:expr, $containee:expr $(,)?) => ({
        let mut reader_string = String::new();
        let reader_result = $reader.read_to_string(&mut reader_string);
        if let Err(reader_err) = reader_result {
            Err(format!(
                concat!(
                    "assertion failed: `assert_io_read_to_string_contains!(reader, &containee)`\n",
                    "    reader label: `{}`,\n",
                    "    reader debug: `{:?}`,\n",
                    " containee label: `{}`,\n",
                    " containee debug: `{:?}`,\n",
                    "      reader err: `{:?}`"
                ),
                stringify!($reader),
                $reader,
                stringify!($containee),
                $containee,
                reader_err
            ))
        } else {
            if reader_string.contains($containee) {
                Ok(())
            } else {
                let _reader_size = reader_result.unwrap();
                Err(format!(
                    concat!(
                        "assertion failed: `assert_io_read_to_string_contains!(reader, &containee)`\n",
                        "    reader label: `{}`,\n",
                        "    reader debug: `{:?}`,\n",
                        " containee label: `{}`,\n",
                        " containee debug: `{:?}`,\n",
                        "  read to string: `{:?}`,\n",
                        " containee value: `{:?}`",
                    ),
                    stringify!($reader),
                    $reader,
                    stringify!($containee),
                    $containee,
                    reader_string,
                    $containee
                ))
            }
        }
    });
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use std::io::Read;

    #[test]
    fn test_assert_io_read_to_string_contains_as_result_x_success() {
        let mut reader = "alfa".as_bytes();
        let containee = "alfa";
        let result = assert_io_read_to_string_contains_as_result!(reader, &containee);
        assert_eq!(result, Ok(()));
    }

    #[test]
    fn test_assert_io_read_to_string_contains_as_result_x_failure() {
        let mut reader = "alfa".as_bytes();
        let containee = "zzz";
        let result = assert_io_read_to_string_contains_as_result!(reader, &containee);
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            concat!(
                "assertion failed: `assert_io_read_to_string_contains!(reader, &containee)`\n",
                "    reader label: `reader`,\n",
                "    reader debug: `[]`,\n",
                " containee label: `&containee`,\n",
                " containee debug: `\"zzz\"`,\n",
                "  read to string: `\"alfa\"`,\n",
                " containee value: `\"zzz\"`"
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
/// let mut reader = "hello".as_bytes();
/// let containee = "ell";
/// assert_io_read_to_string_contains!(reader, &containee);
///
/// # let result = panic::catch_unwind(|| {
/// let mut reader = "hello".as_bytes();
/// let containee = "zzz";
/// assert_io_read_to_string_contains!(reader, &containee);
/// # });
/// // assertion failed: `assert_io_read_to_string_contains!(reader, &containee)`
/// //     reader label: `&reader`,
/// //     reader debug: `[]`,
/// //  containee label: `&containee`,
/// //  containee debug: `\"zzz\"`,
/// //   read to string: `\"hello\"`,
/// //  containee value: `\"zzz\"`
/// # let actual = result.unwrap_err().downcast::<String>().unwrap().to_string();
/// # let expect = concat!(
/// #     "assertion failed: `assert_io_read_to_string_contains!(reader, &containee)`\n",
/// #     "    reader label: `reader`,\n",
/// #     "    reader debug: `[]`,\n",
/// #     " containee label: `&containee`,\n",
/// #     " containee debug: `\"zzz\"`,\n",
/// #     "  read to string: `\"hello\"`,\n",
/// #     " containee value: `\"zzz\"`"
/// # );
/// # assert_eq!(actual, expect);
/// # }
/// ```
///
/// # Module macros
///
/// * [`assert_io_read_to_string_contains`](macro@crate::assert_io_read_to_string_contains)
/// * [`assert_io_read_to_string_contains_as_result`](macro@crate::assert_io_read_to_string_contains_as_result)
/// * [`debug_assert_io_read_to_string_contains`](macro@crate::debug_assert_io_read_to_string_contains)
///
#[macro_export]
macro_rules! assert_io_read_to_string_contains {
    ($reader:expr, $containee:expr $(,)?) => ({
        match assert_io_read_to_string_contains_as_result!($reader, $containee) {
            Ok(()) => (),
            Err(err) => panic!("{}", err),
        }
    });
    ($a:expr, $b:expr, $($message:tt)+) => ({
        match assert_io_read_to_string_contains_as_result!($reader, $containee) {
            Ok(()) => (),
            Err(_err) => panic!("{}", $($message)+),
        }
    });
}

/// Assert a std::io::Read read_to_string() contains a pattern.
///
/// This macro provides the same statements as [`assert_io_read_to_string_contains`](macro.assert_io_read_to_string_contains.html),
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
/// [`std::debug_assert`](https://doc.rust-lang.org/std/macro.debug_assert.html).
///
/// # Module macros
///
/// * [`assert_io_read_to_string_contains`](macro@crate::assert_io_read_to_string_contains)
/// * [`assert_io_read_to_string_contains`](macro@crate::assert_io_read_to_string_contains)
/// * [`debug_assert_io_read_to_string_contains`](macro@crate::debug_assert_io_read_to_string_contains)
///
#[macro_export]
macro_rules! debug_assert_io_read_to_string_contains {
    ($($arg:tt)*) => {
        if $crate::cfg!(debug_assertions) {
            $crate::assert_io_read_to_string_contains!($($arg)*);
        }
    };
}
