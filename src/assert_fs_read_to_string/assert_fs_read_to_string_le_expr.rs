//! Assert a std::fs::read_to_string() value is less than or equal to an expression.
//!
//! # Example
//!
//! ```rust
//! # #[macro_use] extern crate assertables;
//! # fn main() {
//! let path = "alfa.txt";
//! let value = String::from("bravo\n");
//! assert_fs_read_to_string_le_expr!(&path, &value);
//! # }
//! ```
//!
//! # Module macros
//!
//! * [`assert_fs_read_to_string_le_expr`](macro@crate::assert_fs_read_to_string_le_expr)
//! * [`assert_fs_read_to_string_le_expr_as_result`](macro@crate::assert_fs_read_to_string_le_expr_as_result)
//! * [`debug_assert_fs_read_to_string_le_expr`](macro@crate::debug_assert_fs_read_to_string_le_expr)

/// Assert a std::fs::read_to_string() value is less than or equal to an expression.
///
/// * If true, return Result `Ok(())`.
///
/// * Otherwise, return Result `Err` with a diagnostic message.
///
/// This macro provides the same statements as [`assert_fs_read_to_string_le_expr`](macro.assert_fs_read_to_string_le_expr.html),
/// except this macro returns a Result, rather than doing a panic.
///
/// This macro is useful for runtime checks, such as checking parameters,
/// or sanitizing inputs, or handling different results in different ways.
///
/// # Module macros
///
/// * [`assert_fs_read_to_string_le_expr`](macro@crate::assert_fs_read_to_string_le_expr)
/// * [`assert_fs_read_to_string_le_expr_as_result`](macro@crate::assert_fs_read_to_string_le_expr_as_result)
/// * [`debug_assert_fs_read_to_string_le_expr`](macro@crate::debug_assert_fs_read_to_string_le_expr)
///
#[macro_export]
macro_rules! assert_fs_read_to_string_le_expr_as_result {
    ($a_path:expr, $b_expr:expr $(,)?) => ({
        let a_result = ::std::fs::read_to_string($a_path);
        if let Err(a_err) = a_result {
            Err(format!(
                concat!(
                    "assertion failed: `assert_fs_read_to_string_le_expr!(left_path, right_expr)`\n",
                    "  left_path label: `{}`,\n",
                    "  left_path debug: `{:?}`,\n",
                    " right_expr label: `{}`,\n",
                    " right_expr debug: `{:?}`,\n",
                    "         left err: `{:?}`"
                ),
                stringify!($a_path), $a_path,
                stringify!($b_expr), $b_expr,
                a_err
            ))
        } else {
            let a_string = a_result.unwrap();
            let b_string = String::from($b_expr);
            if a_string <= b_string {
                Ok(())
            } else {
                Err(format!(
                    concat!(
                        "assertion failed: `assert_fs_read_to_string_le_expr!(left_path, right_expr)`\n",
                        "  left_path label: `{}`,\n",
                        "  left_path debug: `{:?}`,\n",
                        " right_expr label: `{}`,\n",
                        " right_expr debug: `{:?}`,\n",
                        "             left: `{:?}`,\n",
                        "            right: `{:?}`",
                    ),
                    stringify!($a_path), $a_path,
                    stringify!($b_expr), $b_expr,
                    a_string,
                    b_string
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
    fn test_read_to_string_le_expr_as_result_x_success() {
        let path = DIR.join("alfa.txt");
        let value = String::from("bravo\n");
        let x = assert_fs_read_to_string_le_expr_as_result!(&path, &value);
        assert_eq!(x, Ok(()));
    }

    #[test]
    fn test_read_to_string_le_expr_as_result_x_failure() {
        let path = DIR.join("bravo.txt");
        let value = String::from("alfa\n");
        let x = assert_fs_read_to_string_le_expr_as_result!(&path, &value);
        assert!(x.is_err());
        assert_eq!(
            x.unwrap_err(),
            format!(
                "{}{}{}{}{}{}{}{}{}",
                "assertion failed: `assert_fs_read_to_string_le_expr!(left_path, right_expr)`\n",
                "  left_path label: `&path`,\n",
                "  left_path debug: `\"",
                path.to_string_lossy(),
                "\"`,\n",
                " right_expr label: `&value`,\n",
                " right_expr debug: `\"alfa\\n\"`,\n",
                "             left: `\"bravo\\n\"`,\n",
                "            right: `\"alfa\\n\"`"
            )
        );
    }
}

/// Assert a std::fs::read_to_string() value is less than or equal to an expression.
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
/// let value = String::from("bravo\n");
/// assert_fs_read_to_string_le_expr!(&path, &value);
/// //-> ()
///
/// // Panic with error message
/// let result = panic::catch_unwind(|| {
/// let path = "bravo.txt";
/// let value = String::from("alfa\n");
/// assert_fs_read_to_string_le_expr!(&path, &value);
/// //-> panic!
/// });
/// assert!(result.is_err());
/// let actual = result.unwrap_err().downcast::<String>().unwrap().to_string();
/// let expect = concat!(
///     "assertion failed: `assert_fs_read_to_string_le_expr!(left_path, right_expr)`\n",
///     "  left_path label: `&path`,\n",
///     "  left_path debug: `\"bravo.txt\"`,\n",
///     " right_expr label: `&value`,\n",
///     " right_expr debug: `\"alfa\\n\"`,\n",
///     "             left: `\"bravo\\n\"`,\n",
///     "            right: `\"alfa\\n\"`"
/// );
/// assert_eq!(actual, expect);
/// # }
/// ```
///
/// # Module macros
///
/// * [`assert_fs_read_to_string_le_expr`](macro@crate::assert_fs_read_to_string_le_expr)
/// * [`assert_fs_read_to_string_le_expr_as_result`](macro@crate::assert_fs_read_to_string_le_expr_as_result)
/// * [`debug_assert_fs_read_to_string_le_expr`](macro@crate::debug_assert_fs_read_to_string_le_expr)
///
#[macro_export]
macro_rules! assert_fs_read_to_string_le_expr {
    ($a_path:expr,  $b_expr:expr $(,)?) => ({
        match assert_fs_read_to_string_le_expr_as_result!($a_path, $b_expr) {
            Ok(()) => (),
            Err(err) => panic!("{}", err),
        }
    });
    ($a_path:expr, $b_expr:expr, $($message:tt)+) => ({
        match assert_fs_read_to_string_le_expr_as_result!($a_path, $b_expr) {
            Ok(()) => (),
            Err(_err) => panic!("{}", $($message)+),
        }
    });
}

/// Assert a std::fs::read_to_string() value is less than or equal to an expression.
///
/// This macro provides the same statements as [`assert_fs_read_to_string_le_expr`](macro.assert_fs_read_to_string_le_expr.html),
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
/// * [`assert_fs_read_to_string_le_expr`](macro@crate::assert_fs_read_to_string_le_expr)
/// * [`assert_fs_read_to_string_le_expr`](macro@crate::assert_fs_read_to_string_le_expr)
/// * [`debug_assert_fs_read_to_string_le_expr`](macro@crate::debug_assert_fs_read_to_string_le_expr)
///
#[macro_export]
macro_rules! debug_assert_fs_read_to_string_le_expr {
    ($($arg:tt)*) => {
        if $crate::cfg!(debug_assertions) {
            $crate::std::fs::read_to_string_le_expr!($($arg)*);
        }
    };
}
