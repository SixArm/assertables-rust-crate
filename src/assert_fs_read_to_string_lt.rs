//! Assert a std::fs::read_to_string() value is less than another.
//!
//! * If true, return `()`.
//!
//! * Otherwise, call [`panic!`] with a message and the values of the
//!   expressions with their debug representations.
//!
//! # Examples
//!
//! ```rust
//! # #[macro_use] extern crate assertables;
//! # use std::panic;
//! use std::io::Read;
//!
//! # fn main() {
//! // Return Ok
//! let a ="alfa.txt";
//! let b ="bravo.txt";
//! assert_fs_read_to_string_lt!(&a, &b);
//! //-> ()
//!
//! // Panic with error message
//! let result = panic::catch_unwind(|| {
//! let mut a = "bravo.txt";
//! let mut b = "alfa.txt";
//! assert_fs_read_to_string_lt!(&a, &b);
//! //-> panic!
//! });
//! assert!(result.is_err());
//! let actual = result.unwrap_err().downcast::<String>().unwrap().to_string();
//! let expect = concat!(
//!     "assertion failed: `assert_fs_read_to_string_lt!(left_path, right_path)`\n",
//!     "  left_path label: `&a`,\n",
//!     "  left_path debug: `\"bravo.txt\"`,\n",
//!     " right_path label: `&b`,\n",
//!     " right_path debug: `\"alfa.txt\"`,\n",
//!     "             left: `\"bravo\\n\"`,\n",
//!     "            right: `\"alfa\\n\"`"
//! );
//! assert_eq!(actual, expect);
//! # }
//! ```
//!
//! # Module macros
//!
//! * [`assert_fs_read_to_string_lt`](macro.assert_fs_read_to_string_lt.html)
//! * [`assert_fs_read_to_string_lt_as_result`](macro.assert_fs_read_to_string_lt_as_result.html)
//! * [`debug_assert_fs_read_to_string_lt`](macro.debug_assert_fs_read_to_string_lt.html)

/// Assert a std::fs::read_to_string() value is less than another.
///
/// * If true, return Result `Ok(())`.
///
/// * Otherwise, return Result `Err` with a diagnostic message.
///
/// This macro provides the same statements as [`assert_fs_read_to_string_lt`](macro.assert_fs_read_to_string_lt.html),
/// except this macro returns a Result, rather than doing a panic.
///
/// This macro is useful for runtime checks, such as checking parameters,
/// or sanitizing inputs, or handling different results in different ways.
///
/// # Module macros
///
/// * [`assert_fs_read_to_string_lt`](macro.assert_fs_read_to_string_lt.html)
/// * [`assert_fs_read_to_string_lt_as_result`](macro.assert_fs_read_to_string_lt_as_result.html)
/// * [`debug_assert_fs_read_to_string_lt`](macro.debug_assert_fs_read_to_string_lt.html)
///
#[macro_export]
macro_rules! assert_fs_read_to_string_lt_as_result {
    ($a_path:expr, $b_path:expr $(,)?) => {{
        let a_result = ::std::fs::read_to_string($a_path);
        let b_result = ::std::fs::read_to_string($b_path);
        if a_result.is_err() || b_result.is_err() {
            Err(format!(
                concat!(
                    "assertion failed: `assert_fs_read_to_string_lt!(left_path, right_path)`\n",
                    "  left_path label: `{}`,\n",
                    "  left_path debug: `{:?}`,\n",
                    " right_path label: `{}`,\n",
                    " right_path debug: `{:?}`,\n",
                    "      left result: `{:?}`,\n",
                    "     right result: `{:?}`"
                ),
                stringify!($a_path),
                $a_path,
                stringify!($b_path),
                $b_path,
                a_result,
                b_result
            ))
        } else {
            let a_string = a_result.unwrap();
            let b_string = b_result.unwrap();
            if a_string < b_string {
                Ok(())
            } else {
                Err(format!(
                    concat!(
                        "assertion failed: `assert_fs_read_to_string_lt!(left_path, right_path)`\n",
                        "  left_path label: `{}`,\n",
                        "  left_path debug: `{:?}`,\n",
                        " right_path label: `{}`,\n",
                        " right_path debug: `{:?}`,\n",
                        "             left: `{:?}`,\n",
                        "            right: `{:?}`"
                    ),
                    stringify!($a_path),
                    $a_path,
                    stringify!($b_path),
                    $b_path,
                    a_string,
                    b_string
                ))
            }
        }
    }};
}

#[cfg(test)]
mod tests {
    use once_cell::sync::Lazy;
    #[allow(unused_imports)]
    use std::io::Read;
    use std::path::PathBuf;

    pub static DIR: Lazy<PathBuf> = Lazy::new(|| {
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("tests")
            .join("src")
            .join("std")
            .join("fs")
    });

    #[test]
    fn test_read_to_string_lt_as_result_x_success() {
        let a = DIR.join("alfa.txt");
        let b = DIR.join("bravo.txt");
        let x = assert_fs_read_to_string_lt_as_result!(&a, &b);
        assert_eq!(x, Ok(()));
    }

    #[test]
    fn test_read_to_string_lt_as_result_x_failure() {
        let a = DIR.join("bravo.txt");
        let b = DIR.join("alfa.txt");
        let x = assert_fs_read_to_string_lt_as_result!(&a, &b);
        assert!(x.is_err());
        assert_eq!(
            x.unwrap_err(),
            format!(
                "{}{}{}{}{}{}{}{}{}{}{}",
                "assertion failed: `assert_fs_read_to_string_lt!(left_path, right_path)`\n",
                "  left_path label: `&a`,\n",
                "  left_path debug: `\"",
                a.to_string_lossy(),
                "\"`,\n",
                " right_path label: `&b`,\n",
                " right_path debug: `\"",
                b.to_string_lossy(),
                "\"`,\n",
                "             left: `\"bravo\\n\"`,\n",
                "            right: `\"alfa\\n\"`"
            )
        );
    }
}

/// Assert a std::fs::read_to_string() value is less than another.
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
/// let a ="alfa.txt";
/// let b ="bravo.txt";
/// assert_fs_read_to_string_lt!(&a, &b);
/// //-> ()
///
/// // Panic with error message
/// let result = panic::catch_unwind(|| {
/// let mut a = "bravo.txt";
/// let mut b = "alfa.txt";
/// assert_fs_read_to_string_lt!(&a, &b);
/// //-> panic!
/// });
/// assert!(result.is_err());
/// let actual = result.unwrap_err().downcast::<String>().unwrap().to_string();
/// let expect = concat!(
///     "assertion failed: `assert_fs_read_to_string_lt!(left_path, right_path)`\n",
///     "  left_path label: `&a`,\n",
///     "  left_path debug: `\"bravo.txt\"`,\n",
///     " right_path label: `&b`,\n",
///     " right_path debug: `\"alfa.txt\"`,\n",
///     "             left: `\"bravo\\n\"`,\n",
///     "            right: `\"alfa\\n\"`"
/// );
/// assert_eq!(actual, expect);
/// # }
/// ```
///
/// # Module macros
///
/// * [`assert_fs_read_to_string_lt`](macro.assert_fs_read_to_string_lt.html)
/// * [`assert_fs_read_to_string_lt_as_result`](macro.assert_fs_read_to_string_lt_as_result.html)
/// * [`debug_assert_fs_read_to_string_lt`](macro.debug_assert_fs_read_to_string_lt.html)
///
#[macro_export]
macro_rules! assert_fs_read_to_string_lt {
    ($a_path:expr, $b_path:expr $(,)?) => ({
        match assert_fs_read_to_string_lt_as_result!($a_path, $b_path) {
            Ok(()) => (),
            Err(err) => panic!("{}", err),
        }
    });
    ($a_path:expr, $b_path:expr, $($message:tt)+) => ({
        match assert_fs_read_to_string_lt_as_result!($a_path, $b_path) {
            Ok(()) => (),
            Err(_err) => panic!("{}", $($message)+),
        }
    });
}

/// Assert a std::fs::read_to_string() value is less than another.
///
/// This macro provides the same statements as [`assert_fs_read_to_string_lt`](macro.assert_fs_read_to_string_lt.html),
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
/// * [`assert_fs_read_to_string_lt`](macro.assert_fs_read_to_string_lt.html)
/// * [`assert_fs_read_to_string_lt`](macro.assert_fs_read_to_string_lt.html)
/// * [`debug_assert_fs_read_to_string_lt`](macro.debug_assert_fs_read_to_string_lt.html)
///
#[macro_export]
macro_rules! debug_read_to_string_lt {
    ($($arg:tt)*) => {
        if $crate::cfg!(debug_assertions) {
            $crate::std::fs::read_to_string_lt!($($arg)*);
        }
    };
}
