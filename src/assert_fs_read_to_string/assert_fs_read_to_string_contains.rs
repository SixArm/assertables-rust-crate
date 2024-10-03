//! Assert a std::fs::read_to_string(path) contains a pattern.
//!
//! Pseudocode:<br>
//! std::fs::read_to_string(path) contains expr
//!
//! # Example
//!
//! ```rust
//! use assertables::*;
//! # fn main() {
//! let path = "alfa.txt";
//! let containee = "alfa";
//! assert_fs_read_to_string_contains!(&path, &containee);
//! # }
//! ```
//!
//! # Module macros
//!
//! * [`assert_fs_read_to_string_contains`](macro@crate::assert_fs_read_to_string_contains)
//! * [`assert_fs_read_to_string_contains_as_result`](macro@crate::assert_fs_read_to_string_contains_as_result)
//! * [`debug_assert_fs_read_to_string_contains`](macro@crate::debug_assert_fs_read_to_string_contains)

/// Assert a std::fs::read_to_string(path) contains a pattern.
///
/// Pseudocode:<br>
/// std::fs::read_to_string(path) contains expr
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
    ($path:expr, $containee:expr $(,)?) => {{
        match (&$path, &$containee) {
            (path, containee) => {
                let read_result = ::std::fs::read_to_string(path);
                if let Err(a_err) = read_result {
                    Err(format!(
                        concat!(
                            "assertion failed: `assert_fs_read_to_string_contains!(path, containee)`\n",
                            "https://docs.rs/assertables/8.13.0/assertables/macro.assert_fs_read_to_string_contains.html\n",
                            "      path label: `{}`,\n",
                            "      path debug: `{:?}`,\n",
                            " containee label: `{}`,\n",
                            " containee debug: `{:?}`,\n",
                            "        read err: `{:?}`"
                        ),
                        stringify!($path),
                        path,
                        stringify!($containee),
                        containee,
                        a_err
                    ))
                } else {
                    let read_string = read_result.unwrap();
                    if read_string.contains($containee) {
                        Ok(())
                    } else {
                        Err(format!(
                            concat!(
                                "assertion failed: `assert_fs_read_to_string_contains!(path, containee)`\n",
                                "https://docs.rs/assertables/8.13.0/assertables/macro.assert_fs_read_to_string_contains.html\n",
                                "      path label: `{}`,\n",
                                "      path debug: `{:?}`,\n",
                                " containee label: `{}`,\n",
                                " containee debug: `{:?}`,\n",
                                "     read string: `{:?}`",
                            ),
                            stringify!($path),
                            path,
                            stringify!($containee),
                            containee,
                            read_string
                        ))
                    }
                }
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
    fn test_read_to_string_contains_as_result_x_success() {
        let path = DIR.join("alfa.txt");
        let containee = "alfa";
        let result = assert_fs_read_to_string_contains_as_result!(&path, &containee);
        assert_eq!(result, Ok(()));
    }

    #[test]
    fn test_read_to_string_contains_as_result_x_failure() {
        let path = DIR.join("alfa.txt");
        let containee = "zzz";
        let result = assert_fs_read_to_string_contains_as_result!(&path, &containee);
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            format!(
                concat!(
                    "assertion failed: `assert_fs_read_to_string_contains!(path, containee)`\n",
                    "https://docs.rs/assertables/8.13.0/assertables/macro.assert_fs_read_to_string_contains.html\n",
                    "      path label: `&path`,\n",
                    "      path debug: `{:?}`,\n",
                    " containee label: `&containee`,\n",
                    " containee debug: `\"zzz\"`,\n",
                    "     read string: `\"alfa\\n\"`",
                ),
                path
            )
        );
    }
}

/// Assert a std::fs::read_to_string(path) contains a pattern.
///
/// Pseudocode:<br>
/// std::fs::read_to_string(path) contains expr
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
/// use std::io::Read;
///
/// # fn main() {
/// let path = "alfa.txt";
/// let containee = "alfa";
/// assert_fs_read_to_string_contains!(&path, &containee);
///
/// # let result = panic::catch_unwind(|| {
/// let path = "alfa.txt";
/// let containee = "zzz";
/// assert_fs_read_to_string_contains!(&path, &containee);
/// # });
/// // assertion failed: `assert_fs_read_to_string_contains!(path, containee)`
/// // https://docs.rs/assertables/8.13.0/assertables/macro.assert_fs_read_to_string_contains.html
/// //       path label: `&path`,
/// //       path debug: `\"alfa.txt\"`,
/// //  containee label: `&containee`,
/// //  containee debug: `\"zzz\"`,
/// //      read string: `\"alfa\\n\"`
/// # let actual = result.unwrap_err().downcast::<String>().unwrap().to_string();
/// # let expect = concat!(
/// #     "assertion failed: `assert_fs_read_to_string_contains!(path, containee)`\n",
/// #     "https://docs.rs/assertables/8.13.0/assertables/macro.assert_fs_read_to_string_contains.html\n",
/// #     "      path label: `&path`,\n",
/// #     "      path debug: `\"alfa.txt\"`,\n",
/// #     " containee label: `&containee`,\n",
/// #     " containee debug: `\"zzz\"`,\n",
/// #     "     read string: `\"alfa\\n\"`",
/// # );
/// # assert_eq!(actual, expect);
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
    ($path:expr, $containee:expr $(,)?) => {{
        match $crate::assert_fs_read_to_string_contains_as_result!($path, $containee) {
            Ok(()) => (),
            Err(err) => panic!("{}", err),
        }
    }};
    ($path:expr, $containee:expr, $($message:tt)+) => {{
        match $crate::assert_fs_read_to_string_contains_as_result!($path, $containee) {
            Ok(()) => (),
            Err(_err) => panic!("{}", $($message)+),
        }
    }};
}

/// Assert a std::fs::read_to_string(path) contains a pattern.
///
/// Pseudocode:<br>
/// std::fs::read_to_string(path) contains expr
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
