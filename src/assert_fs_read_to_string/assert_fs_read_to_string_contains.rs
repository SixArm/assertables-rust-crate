//! Assert a std::fs::read_to_string() contains a pattern.
//!
//! # Example
//!
//! ```rust
//! # #[macro_use] extern crate assertables;
//! # fn main() {
//! let path = "alfa.txt";
//! let containee = "alfa";
//! assert_fs_read_to_string_contains!(&path, containee);
//! # }
//! ```
//!
//! # Module macros
//!
//! * [`assert_fs_read_to_string_contains`](macro@crate::assert_fs_read_to_string_contains)
//! * [`assert_fs_read_to_string_contains_as_result`](macro@crate::assert_fs_read_to_string_contains_as_result)
//! * [`debug_assert_fs_read_to_string_contains`](macro@crate::debug_assert_fs_read_to_string_contains)

/// Assert a std::fs::read_to_string() contains a pattern.
///
/// * If true, return Result `Ok(())`.
///
/// * Otherwise, return Result `Err` with a diagnostic message.
///
/// This macro provides the same statements as [`assert_fs_read_to_string_contains`](macro.assert_fs_read_to_string_contains_as_result.html),
/// except this macro returns a Result, rather than doing a panic.
///
/// This macro is useful for runtime checks, such as checking parameters,
/// or sanitizing inputs, or handling different results in different ways.
///
/// # Module macros
///
/// * [`assert_fs_read_to_string_contains`](macro@crate::assert_fs_read_to_string_contains)
/// * [`assert_fs_read_to_string_contains_as_result`](macro@crate::assert_fs_read_to_string_contains_as_result)
/// * [`debug_assert_fs_read_to_string_contains`](macro@crate::debug_assert_fs_read_to_string_contains)
///
#[macro_export]
macro_rules! assert_fs_read_to_string_contains_as_result {
    ($a_path:expr, $b_containee:expr $(,)?) => ({
        let a_result = ::std::fs::read_to_string($a_path);
        if let Err(a_err) = a_result {
            Err(format!(
                concat!(
                    "assertion failed: `assert_fs_read_to_string_contains!(left_path, right_containee)`\n",
                    "       left_path label: `{}`,\n",
                    "       left_path debug: `{:?}`,\n",
                    " right_containee label: `{}`,\n",
                    " right_containee debug: `{:?}`,\n",
                    "              left err: `{:?}`"
                ),
                stringify!($a_path), $a_path,
                stringify!($b_containee), $b_containee,
                a_err
            ))
        } else {
            let a_string = a_result.unwrap();
            if a_string.contains($b_containee) {
                Ok(())
            } else {
                Err(format!(
                    concat!(
                        "assertion failed: `assert_fs_read_to_string_contains!(left_path, right_containee)`\n",
                        "       left_path label: `{}`,\n",
                        "       left_path debug: `{:?}`,\n",
                        " right_containee label: `{}`,\n",
                        " right_containee debug: `{:?}`,\n",
                        "                  left: `{:?}`,\n",
                        "                 right: `{:?}`",
                    ),
                    stringify!($a_path), $a_path,
                    stringify!($b_containee), $b_containee,
                    a_string,
                    $b_containee
                ))
            }
        }
    });
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
    fn test_read_to_string_contains_as_result_x_success() {
        let path = DIR.join("alfa.txt");
        let containee = "alfa";
        let x = assert_fs_read_to_string_contains_as_result!(&path, containee);
        assert_eq!(x, Ok(()));
    }

    #[test]
    fn test_read_to_string_contains_as_result_x_failure() {
        let path = DIR.join("alfa.txt");
        let containee = "zzz";
        let x = assert_fs_read_to_string_contains_as_result!(&path, containee);
        assert!(x.is_err());
        assert_eq!(
            x.unwrap_err(),
            format!("{}{}{}{}{}{}{}{}{}",
                "assertion failed: `assert_fs_read_to_string_contains!(left_path, right_containee)`\n",
                "       left_path label: `&path`,\n",
                "       left_path debug: `\"", path.to_string_lossy(), "\"`,\n",
                " right_containee label: `containee`,\n",
                " right_containee debug: `\"zzz\"`,\n",
                "                  left: `\"alfa\\n\"`,\n",
                "                 right: `\"zzz\"`"
            )
        );
    }
}

/// Assert a std::fs::read_to_string() contains a pattern.
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
/// let path = "alfa.txt";
/// let containee = "alfa";
/// assert_fs_read_to_string_contains!(&path, containee);
/// //-> ()
///
/// // Panic with error message
/// let result = panic::catch_unwind(|| {
/// let path = "alfa.txt";
/// let containee = "zzz";
/// assert_fs_read_to_string_contains!(&path, containee);
/// //-> panic!
/// });
/// assert!(result.is_err());
/// let actual = result.unwrap_err().downcast::<String>().unwrap().to_string();
/// let expect = concat!(
///     "assertion failed: `assert_fs_read_to_string_contains!(left_path, right_containee)`\n",
///     "       left_path label: `&path`,\n",
///     "       left_path debug: `\"alfa.txt\"`,\n",
///     " right_containee label: `containee`,\n",
///     " right_containee debug: `\"zzz\"`,\n",
///     "                  left: `\"alfa\\n\"`,\n",
///     "                 right: `\"zzz\"`"
/// );
/// assert_eq!(actual, expect);
/// # }
/// ```
///
/// # Module macros
///
/// * [`assert_fs_read_to_string_contains`](macro@crate::assert_fs_read_to_string_contains)
/// * [`assert_fs_read_to_string_contains_as_result`](macro@crate::assert_fs_read_to_string_contains_as_result)
/// * [`debug_assert_fs_read_to_string_contains`](macro@crate::debug_assert_fs_read_to_string_contains)
///
#[macro_export]
macro_rules! assert_fs_read_to_string_contains {
    ($a_path:expr, $b:expr $(,)?) => ({
        match assert_fs_read_to_string_contains_as_result!($a_path, $b) {
            Ok(()) => (),
            Err(err) => panic!("{}", err),
        }
    });
    ($a_path:expr, $b:expr, $($message:tt)+) => ({
        match assert_fs_read_to_string_contains_as_result!($a_path, $b) {
            Ok(()) => (),
            Err(_err) => panic!("{}", $($message)+),
        }
    });
}

/// Assert a std::fs::read_to_string() contains a pattern.
///
/// This macro provides the same statements as [`assert_fs_read_to_string_contains`](macro.assert_fs_read_to_string_contains.html),
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
/// * [`assert_fs_read_to_string_contains`](macro@crate::assert_fs_read_to_string_contains)
/// * [`assert_fs_read_to_string_contains`](macro@crate::assert_fs_read_to_string_contains)
/// * [`debug_assert_fs_read_to_string_contains`](macro@crate::debug_assert_fs_read_to_string_contains)
///
#[macro_export]
macro_rules! debug_assert_fs_read_to_string_contains {
    ($($arg:tt)*) => {
        if $crate::cfg!(debug_assertions) {
            $crate::std::fs::read_to_string_contains!($($arg)*);
        }
    };
}
