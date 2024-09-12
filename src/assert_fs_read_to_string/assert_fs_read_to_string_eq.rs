//! Assert a std::fs::read_to_string() value is equal to another.
//!
//! # Example
//!
//! ```rust
//! # #[macro_use] extern crate assertables;
//! # fn main() {
//! let a ="alfa.txt";
//! let b = "alfa.txt";
//! assert_fs_read_to_string_eq!(&a, &b);
//! # }
//! ```
//!
//! # Module macros
//!
//! * [`assert_fs_read_to_string_eq`](macro@crate::assert_fs_read_to_string_eq)
//! * [`assert_fs_read_to_string_eq_as_result`](macro@crate::assert_fs_read_to_string_eq_as_result)
//! * [`debug_assert_fs_read_to_string_eq`](macro@crate::debug_assert_fs_read_to_string_eq)

/// Assert a std::fs::read_to_string() is equal to another.
///
/// * If true, return Result `Ok(())`.
///
/// * Otherwise, return Result `Err` with a diagnostic message.
///
/// This macro provides the same statements as [`assert_fs_read_to_string_eq`](macro.assert_fs_read_to_string_eq_as_result.html),
/// except this macro returns a Result, rather than doing a panic.
///
/// This macro is useful for runtime checks, such as checking parameters,
/// or sanitizing inputs, or handling different results in different ways.
///
/// # Module macros
///
/// * [`assert_fs_read_to_string_eq`](macro@crate::assert_fs_read_to_string_eq)
/// * [`assert_fs_read_to_string_eq_as_result`](macro@crate::assert_fs_read_to_string_eq_as_result)
/// * [`debug_assert_fs_read_to_string_eq`](macro@crate::debug_assert_fs_read_to_string_eq)
///
#[macro_export]
macro_rules! assert_fs_read_to_string_eq_as_result {
    ($a_path:expr, $b_path:expr $(,)?) => {{
        let a_result = ::std::fs::read_to_string($a_path);
        let b_result = ::std::fs::read_to_string($b_path);
        if a_result.is_err() || b_result.is_err() {
            Err(format!(
                concat!(
                    "assertion failed: `assert_fs_read_to_string_eq!(a_path, b_path)`\n",
                    " a_path label: `{}`,\n",
                    " a_path debug: `{:?}`,\n",
                    " b_path label: `{}`,\n",
                    " b_path debug: `{:?}`,\n",
                    "     a result: `{:?}`,\n",
                    "     b result: `{:?}`"
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
            if a_string == b_string {
                Ok(())
            } else {
                Err(format!(
                    concat!(
                        "assertion failed: `assert_fs_read_to_string_eq!(a_path, b_path)`\n",
                        " a_path label: `{}`,\n",
                        " a_path debug: `{:?}`,\n",
                        " b_path label: `{}`,\n",
                        " b_path debug: `{:?}`,\n",
                        "     a string: `{:?}`,\n",
                        "     b string: `{:?}`"
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
    fn test_read_to_string_eq_as_result_x_success() {
        let a = DIR.join("alfa.txt");
        let b = DIR.join("alfa.txt");
        let result = assert_fs_read_to_string_eq_as_result!(&a, &b);
        assert_eq!(result, Ok(()));
    }

    #[test]
    fn test_read_to_string_eq_as_result_x_failure() {
        let a = DIR.join("alfa.txt");
        let b = DIR.join("bravo.txt");
        let result = assert_fs_read_to_string_eq_as_result!(&a, &b);
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            format!(
                "{}{}{}{}{}{}{}{}{}{}{}",
                "assertion failed: `assert_fs_read_to_string_eq!(a_path, b_path)`\n",
                " a_path label: `&a`,\n",
                " a_path debug: `\"",
                a.to_string_lossy(),
                "\"`,\n",
                " b_path label: `&b`,\n",
                " b_path debug: `\"",
                b.to_string_lossy(),
                "\"`,\n",
                "     a string: `\"alfa\\n\"`,\n",
                "     b string: `\"bravo\\n\"`"
            )
        );
    }
}

/// Assert a std::fs::read_to_string() value is equal to another.
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
/// let a = "alfa.txt";
/// let b = "alfa.txt";
/// assert_fs_read_to_string_eq!(&a, &b);
///
/// # let result = panic::catch_unwind(|| {
/// let a = "alfa.txt";
/// let b = "bravo.txt";
/// assert_fs_read_to_string_eq!(&a, &b);
/// # });
/// // assertion failed: `assert_fs_read_to_string_eq!(a_path, b_path)`
/// //  a_path label: `&a`,
/// //  a_path debug: `\"alfa.txt\"`,
/// //  b_path label: `&b`,
/// //  b_path debug: `\"bravo.txt\"`,
/// //      a string: `\"alfa\\n\"`,
/// //      b string: `\"bravo\\n\"`
/// # let actual = result.unwrap_err().downcast::<String>().unwrap().to_string();
/// # let expect = concat!(
/// #     "assertion failed: `assert_fs_read_to_string_eq!(a_path, b_path)`\n",
/// #     " a_path label: `&a`,\n",
/// #     " a_path debug: `\"alfa.txt\"`,\n",
/// #     " b_path label: `&b`,\n",
/// #     " b_path debug: `\"bravo.txt\"`,\n",
/// #     "     a string: `\"alfa\\n\"`,\n",
/// #     "     b string: `\"bravo\\n\"`"
/// # );
/// # assert_eq!(actual, expect);
/// # }
/// ```
///
/// # Module macros
///
/// * [`assert_fs_read_to_string_eq`](macro@crate::assert_fs_read_to_string_eq)
/// * [`assert_fs_read_to_string_eq_as_result`](macro@crate::assert_fs_read_to_string_eq_as_result)
/// * [`debug_assert_fs_read_to_string_eq`](macro@crate::debug_assert_fs_read_to_string_eq)
///
#[macro_export]
macro_rules! assert_fs_read_to_string_eq {
    ($a_path:expr, $b_path:expr $(,)?) => ({
        match assert_fs_read_to_string_eq_as_result!($a_path, $b_path) {
            Ok(()) => (),
            Err(err) => panic!("{}", err),
        }
    });
    ($a_path:expr, $b_path:expr, $($message:tt)+) => ({
        match assert_fs_read_to_string_eq_as_result!($a_path, $b_path) {
            Ok(()) => (),
            Err(_err) => panic!("{}", $($message)+),
        }
    });
}

/// Assert a std::fs::read_to_string() value is equal to another.
///
/// This macro provides the same statements as [`assert_fs_read_to_string_eq`](macro.assert_fs_read_to_string_eq.html),
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
/// * [`assert_fs_read_to_string_eq`](macro@crate::assert_fs_read_to_string_eq)
/// * [`assert_fs_read_to_string_eq`](macro@crate::assert_fs_read_to_string_eq)
/// * [`debug_assert_fs_read_to_string_eq`](macro@crate::debug_assert_fs_read_to_string_eq)
///
#[macro_export]
macro_rules! debug_assert_fs_read_to_string_eq {
    ($($arg:tt)*) => {
        if $crate::cfg!(debug_assertions) {
            $crate::std::fs::read_to_string_eq!($($arg)*);
        }
    };
}
