//! Assert a ::std::fs::read_to_string(path) is not equal to another.
//!
//! Pseudocode:<br>
//! std::fs::read_to_string(a_path) ≠ std::fs::read_to_string(b_path)
//!
//! # Example
//!
//! ```rust
//! use assertables::*;
//!
//! # fn main() {
//! let a ="alfa.txt";
//! let b ="bravo.txt";
//! assert_fs_read_to_string_ne!(&a, &b);
//! # }
//! ```
//!
//! # Module macros
//!
//! * [`assert_fs_read_to_string_ne`](macro@crate::assert_fs_read_to_string_ne)
//! * [`assert_fs_read_to_string_ne_as_result`](macro@crate::assert_fs_read_to_string_ne_as_result)
//! * [`debug_assert_fs_read_to_string_ne`](macro@crate::debug_assert_fs_read_to_string_ne)

/// Assert a ::std::fs::read_to_string(path) is not equal to another.
///
/// Pseudocode:<br>
/// std::fs::read_to_string(a_path) ≠ std::fs::read_to_string(b_path)
///
/// * If true, return Result `Ok((a_path_into_string, b_path_into_string))`.
///
/// * Otherwise, return Result `Err(message)`.
///
/// This macro provides the same statements as [`assert_fs_read_to_string_ne`](macro.assert_fs_read_to_string_ne.html),
/// except this macro returns a Result, rather than doing a panic.
///
/// This macro is useful for runtime checks, such as checking parameters,
/// or sanitizing inputs, or handling different results in different ways.
///
/// # Module macros
///
/// * [`assert_fs_read_to_string_ne`](macro@crate::assert_fs_read_to_string_ne)
/// * [`assert_fs_read_to_string_ne_as_result`](macro@crate::assert_fs_read_to_string_ne_as_result)
/// * [`debug_assert_fs_read_to_string_ne`](macro@crate::debug_assert_fs_read_to_string_ne)
///
#[macro_export]
macro_rules! assert_fs_read_to_string_ne_as_result {
    ($a_path:expr, $b_path:expr $(,)?) => {{
        match (&$a_path, &$b_path) {
            (a_path, b_path) => {
                match (std::fs::read_to_string(a_path), std::fs::read_to_string(b_path)) {
                    (Ok(a_string), Ok(b_string)) => {
                        if a_string != b_string {
                            Ok((a_string, b_string))
                        } else {
                            Err(
                                format!(
                                    concat!(
                                        "assertion failed: `assert_fs_read_to_string_ne!(a_path, b_path)`\n",
                                        "https://docs.rs/assertables/9.2.0/assertables/macro.assert_fs_read_to_string_ne.html\n",
                                        " a_path label: `{}`,\n",
                                        " a_path debug: `{:?}`,\n",
                                        " b_path label: `{}`,\n",
                                        " b_path debug: `{:?}`,\n",
                                        "     a string: `{:?}`,\n",
                                        "     b string: `{:?}`"
                                    ),
                                    stringify!($a_path),
                                    a_path,
                                    stringify!($b_path),
                                    b_path,
                                    a_string,
                                    b_string
                                )
                            )
                        }
                    },
                    (a_result, b_result) => {
                        Err(
                            format!(
                                concat!(
                                    "assertion failed: `assert_fs_read_to_string_ne!(a_path, b_path)`\n",
                                    "https://docs.rs/assertables/9.2.0/assertables/macro.assert_fs_read_to_string_ne.html\n",
                                    " a_path label: `{}`,\n",
                                    " a_path debug: `{:?}`,\n",
                                    " b_path label: `{}`,\n",
                                    " b_path debug: `{:?}`,\n",
                                    "     a result: `{:?}`,\n",
                                    "     b result: `{:?}`"
                                ),
                                stringify!($a_path),
                                a_path,
                                stringify!($b_path),
                                b_path,
                                a_result,
                                b_result
                            )
                        )
                    }
                }
            }
        }
    }};
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use std::io::Read;
    use std::path::PathBuf;
    use std::sync::LazyLock;

    pub static DIR: LazyLock<PathBuf> = LazyLock::new(|| {
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("tests")
            .join("src")
            .join("std")
            .join("fs")
    });

    #[test]
    fn lt() {
        let a = DIR.join("alfa.txt");
        let b = DIR.join("bravo.txt");
        let result = assert_fs_read_to_string_ne_as_result!(&a, &b);
        assert_eq!(
            result.unwrap(),
            (String::from("alfa\n"), String::from("bravo\n"))
        );
    }

    #[test]
    fn gt() {
        let a = DIR.join("bravo.txt");
        let b = DIR.join("alfa.txt");
        let result = assert_fs_read_to_string_ne_as_result!(&a, &b);
        assert_eq!(
            result.unwrap(),
            (String::from("bravo\n"), String::from("alfa\n"))
        );
    }

    #[test]
    fn eq() {
        let a = DIR.join("alfa.txt");
        let b = DIR.join("alfa.txt");
        let result = assert_fs_read_to_string_ne_as_result!(&a, &b);
        assert_eq!(
            result.unwrap_err(),
            format!(
                concat!(
                    "assertion failed: `assert_fs_read_to_string_ne!(a_path, b_path)`\n",
                    "https://docs.rs/assertables/9.2.0/assertables/macro.assert_fs_read_to_string_ne.html\n",
                    " a_path label: `&a`,\n",
                    " a_path debug: `{:?}`,\n",
                    " b_path label: `&b`,\n",
                    " b_path debug: `{:?}`,\n",
                    "     a string: `\"alfa\\n\"`,\n",
                    "     b string: `\"alfa\\n\"`"
                ),
                a,
                b
            )
        );
    }
}

/// Assert a ::std::fs::read_to_string(path) is not equal to another.
///
/// Pseudocode:<br>
/// std::fs::read_to_string(a_path) ≠ std::fs::read_to_string(b_path)
///
/// * If true, return (a_path_into_string, b_path_into_string).
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
/// let a = "alfa.txt";
/// let b = "bravo.txt";
/// assert_fs_read_to_string_ne!(&a, &b);
///
/// # let result = panic::catch_unwind(|| {
/// // This will panic
/// let a = "alfa.txt";
/// let b = "alfa.txt";
/// assert_fs_read_to_string_ne!(&a, &b);
/// // assertion failed: `assert_fs_read_to_string_ne!(a_path, b_path)`
/// // https://docs.rs/assertables/9.2.0/assertables/macro.assert_fs_read_to_string_ne.html
/// //  a_path label: `&a`,
/// //  a_path debug: `\"alfa.txt\"`,
/// //  b_path label: `&b`,
/// //  b_path debug: `\"alfa.txt\"`,
/// //      a string: `\"alfa\\n\"`,
/// //      b string: `\"alfa\\n\"`
/// # });
/// # let actual = result.unwrap_err().downcast::<String>().unwrap().to_string();
/// # let expect = concat!(
/// #     "assertion failed: `assert_fs_read_to_string_ne!(a_path, b_path)`\n",
/// #     "https://docs.rs/assertables/9.2.0/assertables/macro.assert_fs_read_to_string_ne.html\n",
/// #     " a_path label: `&a`,\n",
/// #     " a_path debug: `\"alfa.txt\"`,\n",
/// #     " b_path label: `&b`,\n",
/// #     " b_path debug: `\"alfa.txt\"`,\n",
/// #     "     a string: `\"alfa\\n\"`,\n",
/// #     "     b string: `\"alfa\\n\"`"
/// # );
/// # assert_eq!(actual, expect);
/// # }
/// ```
///
/// # Module macros
///
/// * [`assert_fs_read_to_string_ne`](macro@crate::assert_fs_read_to_string_ne)
/// * [`assert_fs_read_to_string_ne_as_result`](macro@crate::assert_fs_read_to_string_ne_as_result)
/// * [`debug_assert_fs_read_to_string_ne`](macro@crate::debug_assert_fs_read_to_string_ne)
///
#[macro_export]
macro_rules! assert_fs_read_to_string_ne {
    ($a_path:expr, $b_path:expr $(,)?) => {{
        match $crate::assert_fs_read_to_string_ne_as_result!($a_path, $b_path) {
            Ok(x) => x,
            Err(err) => panic!("{}", err),
        }
    }};
    ($a_path:expr, $b_path:expr, $($message:tt)+) => {{
        match $crate::assert_fs_read_to_string_ne_as_result!($a_path, $b_path) {
            Ok(x) => x,
            Err(_err) => panic!("{}", $($message)+),
        }
    }};
}

/// Assert a ::std::fs::read_to_string(path) is not equal to another.
///
/// Pseudocode:<br>
/// std::fs::read_to_string(a_path) ≠ std::fs::read_to_string(b_path)
///
/// This macro provides the same statements as [`assert_fs_read_to_string_ne`](macro.assert_fs_read_to_string_ne.html),
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
/// [`::std::debug_assert`](https://doc.rust-lang.org/std/macro.debug_assert.html).
///
/// # Module macros
///
/// * [`assert_fs_read_to_string_ne`](macro@crate::assert_fs_read_to_string_ne)
/// * [`assert_fs_read_to_string_ne`](macro@crate::assert_fs_read_to_string_ne)
/// * [`debug_assert_fs_read_to_string_ne`](macro@crate::debug_assert_fs_read_to_string_ne)
///
#[macro_export]
macro_rules! debug_assert_fs_read_to_string_ne {
    ($($arg:tt)*) => {
        if $crate::cfg!(debug_assertions) {
            $crate::std::fs::read_to_string_ne!($($arg)*);
        }
    };
}
