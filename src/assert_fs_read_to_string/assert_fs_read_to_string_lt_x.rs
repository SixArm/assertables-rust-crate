//! Assert a ::std::fs::read_to_string(path) value is less than an expression.
//!
//! Pseudocode:<br>
//! std::fs::read_to_string(path) < expr
//!
//! # Example
//!
//! ```rust
//! use assertables::*;
//!
//! let path = "alfa.txt";
//! let value = String::from("bravo\n");
//! assert_fs_read_to_string_lt_x!(&path, &value);
//! ```
//!
//! # Module macros
//!
//! * [`assert_fs_read_to_string_lt_x`](macro@crate::assert_fs_read_to_string_lt_x)
//! * [`assert_fs_read_to_string_lt_x_as_result`](macro@crate::assert_fs_read_to_string_lt_x_as_result)
//! * [`debug_assert_fs_read_to_string_lt_x`](macro@crate::debug_assert_fs_read_to_string_lt_x)

/// Assert a ::std::fs::read_to_string(path) value is less than an expression.
///
/// Pseudocode:<br>
/// std::fs::read_to_string(path) < expr
///
/// * If true, return Result `Ok(path_into_string)`.
///
/// * Otherwise, return Result `Err(message)`.
///
/// This macro is useful for runtime checks, such as checking parameters,
/// or sanitizing inputs, or handling different results in different ways.
///
/// # Module macros
///
/// * [`assert_fs_read_to_string_lt_x`](macro@crate::assert_fs_read_to_string_lt_x)
/// * [`assert_fs_read_to_string_lt_x_as_result`](macro@crate::assert_fs_read_to_string_lt_x_as_result)
/// * [`debug_assert_fs_read_to_string_lt_x`](macro@crate::debug_assert_fs_read_to_string_lt_x)
///
#[macro_export]
macro_rules! assert_fs_read_to_string_lt_x_as_result {
    ($a_path:expr, $b_expr:expr $(,)?) => {{
        match (&$a_path, &$b_expr) {
            (a_path, b_expr) => {
                match (::std::fs::read_to_string(a_path)) {
                    Ok(a_string) => {
                        let b_string = String::from($b_expr);
                        if a_string < b_string {
                            Ok(a_string)
                        } else {
                            Err(
                                format!(
                                    concat!(
                                        "assertion failed: `assert_fs_read_to_string_lt_x!(a_path, b_expr)`\n",
                                        "https://docs.rs/assertables/9.5.0/assertables/macro.assert_fs_read_to_string_lt_x.html\n",
                                        " a_path label: `{}`,\n",
                                        " a_path debug: `{:?}`,\n",
                                        " b_expr label: `{}`,\n",
                                        " b_expr debug: `{:?}`,\n",
                                        "     a string: `{:?}`,\n",
                                        "     b string: `{:?}`",
                                    ),
                                    stringify!($a_path),
                                    a_path,
                                    stringify!($b_expr),
                                    b_expr,
                                    a_string,
                                    b_string
                                )
                            )
                        }
                    },
                    Err(err) => {
                        Err(
                            format!(
                                concat!(
                                    "assertion failed: `assert_fs_read_to_string_lt_x!(a_path, b_expr)`\n",
                                    "https://docs.rs/assertables/9.5.0/assertables/macro.assert_fs_read_to_string_lt_x.html\n",
                                    " a_path label: `{}`,\n",
                                    " a_path debug: `{:?}`,\n",
                                    " b_expr label: `{}`,\n",
                                    " b_expr debug: `{:?}`,\n",
                                    "          err: `{:?}`"
                                ),
                                stringify!($a_path),
                                a_path,
                                stringify!($b_expr),
                                b_expr,
                                err
                            )
                        )
                    }
                }
            }
        }
    }};
}

#[cfg(test)]
mod test_assert_fs_read_to_string_lt_x_as_result {
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
        let path = DIR.join("alfa.txt");
        let value = String::from("bravo\n");
        let actual = assert_fs_read_to_string_lt_x_as_result!(&path, &value);
        assert_eq!(actual.unwrap(), String::from("alfa\n"));
    }

    #[test]
    fn eq() {
        let path = DIR.join("alfa.txt");
        let value = String::from("alfa\n");
        let actual = assert_fs_read_to_string_lt_x_as_result!(&path, &value);
        let message = format!(
            concat!(
                "assertion failed: `assert_fs_read_to_string_lt_x!(a_path, b_expr)`\n",
                "https://docs.rs/assertables/9.5.0/assertables/macro.assert_fs_read_to_string_lt_x.html\n",
                " a_path label: `&path`,\n",
                " a_path debug: `{:?}`,\n",
                " b_expr label: `&value`,\n",
                " b_expr debug: `\"alfa\\n\"`,\n",
                "     a string: `\"alfa\\n\"`,\n",
                "     b string: `\"alfa\\n\"`"
            ),
            path
        );
        assert_eq!(actual.unwrap_err(), message);
    }

    #[test]
    fn gt() {
        let path = DIR.join("bravo.txt");
        let value = String::from("alfa\n");
        let actual = assert_fs_read_to_string_lt_x_as_result!(&path, &value);
        let message = format!(
            concat!(
                "assertion failed: `assert_fs_read_to_string_lt_x!(a_path, b_expr)`\n",
                "https://docs.rs/assertables/9.5.0/assertables/macro.assert_fs_read_to_string_lt_x.html\n",
                " a_path label: `&path`,\n",
                " a_path debug: `{:?}`,\n",
                " b_expr label: `&value`,\n",
                " b_expr debug: `\"alfa\\n\"`,\n",
                "     a string: `\"bravo\\n\"`,\n",
                "     b string: `\"alfa\\n\"`"
            ),
            path
        );
        assert_eq!(actual.unwrap_err(), message);
    }
}

/// Assert a ::std::fs::read_to_string(path) value is less than an expression.
///
/// Pseudocode:<br>
/// std::fs::read_to_string(path) < expr
///
/// * If true, return (path_into_string, expr_into_string).
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
/// let value = String::from("bravo\n");
/// assert_fs_read_to_string_lt_x!(&path, &value);
///
/// # let result = panic::catch_unwind(|| {
/// // This will panic
/// let path = "bravo.txt";
/// let value = String::from("alfa\n");
/// assert_fs_read_to_string_lt_x!(&path, &value);
/// # });
/// // assertion failed: `assert_fs_read_to_string_lt_x!(a_path, b_expr)`
/// // https://docs.rs/assertables/9.5.0/assertables/macro.assert_fs_read_to_string_lt_x.html
/// //  a_path label: `&path`,
/// //  a_path debug: `\"bravo.txt\"`,
/// //  b_expr label: `&value`,
/// //  b_expr debug: `\"alfa\\n\"`,
/// //      a string: `\"bravo\\n\"`,
/// //      b string: `\"alfa\\n\"`
/// # let actual = result.unwrap_err().downcast::<String>().unwrap().to_string();
/// # let message = concat!(
/// #     "assertion failed: `assert_fs_read_to_string_lt_x!(a_path, b_expr)`\n",
/// #     "https://docs.rs/assertables/9.5.0/assertables/macro.assert_fs_read_to_string_lt_x.html\n",
/// #     " a_path label: `&path`,\n",
/// #     " a_path debug: `\"bravo.txt\"`,\n",
/// #     " b_expr label: `&value`,\n",
/// #     " b_expr debug: `\"alfa\\n\"`,\n",
/// #     "     a string: `\"bravo\\n\"`,\n",
/// #     "     b string: `\"alfa\\n\"`"
/// # );
/// # assert_eq!(actual, message);
/// # }
/// ```
///
/// # Module macros
///
/// * [`assert_fs_read_to_string_lt_x`](macro@crate::assert_fs_read_to_string_lt_x)
/// * [`assert_fs_read_to_string_lt_x_as_result`](macro@crate::assert_fs_read_to_string_lt_x_as_result)
/// * [`debug_assert_fs_read_to_string_lt_x`](macro@crate::debug_assert_fs_read_to_string_lt_x)
///
#[macro_export]
macro_rules! assert_fs_read_to_string_lt_x {
    ($a_path:expr,  $b_expr:expr $(,)?) => {{
        match $crate::assert_fs_read_to_string_lt_x_as_result!($a_path, $b_expr) {
            Ok(x) => x,
            Err(err) => panic!("{}", err),
        }
    }};
    ($a_path:expr, $b_expr:expr, $($message:tt)+) => {{
        match $crate::assert_fs_read_to_string_lt_x_as_result!($a_path, $b_expr) {
            Ok(x) => x,
            Err(err) => panic!("{}\n{}", format_args!($($message)+), err),
        }
    }};
}

#[cfg(test)]
mod test_assert_fs_read_to_string_lt_x {
    #[allow(unused_imports)]
    use std::io::Read;
    use std::panic;
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
        let path = DIR.join("alfa.txt");
        let value = String::from("bravo\n");
        let actual = assert_fs_read_to_string_lt_x!(&path, &value);
        assert_eq!(actual, String::from("alfa\n"));
    }

    #[test]
    fn eq() {
        let path = DIR.join("alfa.txt");
        let value = String::from("alfa\n");
        let result = panic::catch_unwind(|| {
            let _actual = assert_fs_read_to_string_lt_x!(&path, &value);
        });
        let message = format!(
            concat!(
                "assertion failed: `assert_fs_read_to_string_lt_x!(a_path, b_expr)`\n",
                "https://docs.rs/assertables/9.5.0/assertables/macro.assert_fs_read_to_string_lt_x.html\n",
                " a_path label: `&path`,\n",
                " a_path debug: `{:?}`,\n",
                " b_expr label: `&value`,\n",
                " b_expr debug: `\"alfa\\n\"`,\n",
                "     a string: `\"alfa\\n\"`,\n",
                "     b string: `\"alfa\\n\"`"
            ),
            path
        );
        assert_eq!(
            result
                .unwrap_err()
                .downcast::<String>()
                .unwrap()
                .to_string(),
            message
        );
    }

    #[test]
    fn gt() {
        let path = DIR.join("bravo.txt");
        let value = String::from("alfa\n");
        let result = panic::catch_unwind(|| {
            let _actual = assert_fs_read_to_string_lt_x!(&path, &value);
        });
        let message = format!(
            concat!(
                "assertion failed: `assert_fs_read_to_string_lt_x!(a_path, b_expr)`\n",
                "https://docs.rs/assertables/9.5.0/assertables/macro.assert_fs_read_to_string_lt_x.html\n",
                " a_path label: `&path`,\n",
                " a_path debug: `{:?}`,\n",
                " b_expr label: `&value`,\n",
                " b_expr debug: `\"alfa\\n\"`,\n",
                "     a string: `\"bravo\\n\"`,\n",
                "     b string: `\"alfa\\n\"`"
            ),
            path
        );
        assert_eq!(
            result
                .unwrap_err()
                .downcast::<String>()
                .unwrap()
                .to_string(),
            message
        );
    }
}

/// Assert a ::std::fs::read_to_string(path) value is less than an expression.
///
/// Pseudocode:<br>
/// std::fs::read_to_string(path) < expr
///
/// This macro provides the same statements as [`assert_fs_read_to_string_lt_x`](macro.assert_fs_read_to_string_lt_x.html),
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
/// * [`assert_fs_read_to_string_lt_x`](macro@crate::assert_fs_read_to_string_lt_x)
/// * [`assert_fs_read_to_string_lt_x`](macro@crate::assert_fs_read_to_string_lt_x)
/// * [`debug_assert_fs_read_to_string_lt_x`](macro@crate::debug_assert_fs_read_to_string_lt_x)
///
#[macro_export]
macro_rules! debug_assert_fs_read_to_string_lt_x {
    ($($arg:tt)*) => {
        if $crate::cfg!(debug_assertions) {
            $crate::std::fs::read_to_string_lt_expr!($($arg)*);
        }
    };
}
