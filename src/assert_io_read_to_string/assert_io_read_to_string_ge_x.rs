//! Assert a ::std::io::Read read_to_string() value is greater than or equal to an expression.
//!
//! Pseudocode:<br>
//! (reader.read_to_string(a_string) ⇒ a_string) ≥ (expr ⇒ b_string)
//!
//! # Example
//!
//! ```rust
//! use assertables::*;
//! use std::io::Read;
//!
//! # fn main() {
//! let mut reader = "bravo".as_bytes();
//! let value = String::from("alfa");
//! assert_io_read_to_string_ge_x!(reader, &value);
//! # }
//! ```
//!
//! # Module macros
//!
//! * [`assert_io_read_to_string_ge_x`](macro@crate::assert_io_read_to_string_ge_x)
//! * [`assert_io_read_to_string_ge_x_as_result`](macro@crate::assert_io_read_to_string_ge_x_as_result)
//! * [`debug_assert_io_read_to_string_ge_x`](macro@crate::debug_assert_io_read_to_string_ge_x)

/// Assert a ::std::io::Read read_to_string() value is greater than or equal to an expression.
///
/// Pseudocode:<br>
/// (reader.read_to_string(a_string) ⇒ a_string) ≥ (expr ⇒ b_string)
///
/// * If true, return Result `Ok(a_string)`.
///
/// * Otherwise, return Result `Err(message)`.
///
/// This macro provides the same statements as [`assert_`](macro.assert_.html),
/// except this macro returns a Result, rather than doing a panic.
///
/// This macro is useful for runtime checks, such as checking parameters,
/// or sanitizing inputs, or handling different results in different ways.
///
/// # Module macros
///
/// * [`assert_io_read_to_string_ge_x`](macro@crate::assert_io_read_to_string_ge_x)
/// * [`assert_io_read_to_string_ge_x_as_result`](macro@crate::assert_io_read_to_string_ge_x_as_result)
/// * [`debug_assert_io_read_to_string_ge_x`](macro@crate::debug_assert_io_read_to_string_ge_x)
///
#[macro_export]
macro_rules! assert_io_read_to_string_ge_x_as_result {
    ($a_reader:expr, $b_expr:expr $(,)?) => {{
        match (/*&$reader,*/ &$b_expr) {
            b_expr => {
                let mut a_string = String::new();
                match ($a_reader.read_to_string(&mut a_string)) {
                    Ok(_a_size) => {
                        let b_string = String::from($b_expr);
                        if (a_string >= b_string) {
                            Ok(a_string)
                        } else {
                            Err(
                                format!(
                                    concat!(
                                        "assertion failed: `assert_io_read_to_string_ge_x!(a_reader, b_expr)`\n",
                                        "https://docs.rs/assertables/9.2.0/assertables/macro.assert_io_read_to_string_ge_x.html\n",
                                        " a_reader label: `{}`,\n",
                                        " a_reader debug: `{:?}`,\n",
                                        "   b_expr label: `{}`,\n",
                                        "   b_expr debug: `{:?}`,\n",
                                        "              a: `{:?}`,\n",
                                        "              b: `{:?}`",
                                    ),
                                    stringify!($a_reader),
                                    $a_reader,
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
                                    "assertion failed: `assert_io_read_to_string_ge_x!(a_reader, b_expr)`\n",
                                    "https://docs.rs/assertables/9.2.0/assertables/macro.assert_io_read_to_string_ge_x.html\n",
                                    " a_reader label: `{}`,\n",
                                    " a_reader debug: `{:?}`,\n",
                                    "   b_expr label: `{}`,\n",
                                    "   b_expr debug: `{:?}`,\n",
                                    "            err: `{:?}`"
                                ),
                                stringify!($a_reader),
                                $a_reader,
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
mod tests {
    #[allow(unused_imports)]
    use std::io::Read;

    #[test]
    fn gt() {
        let mut reader = "bravo".as_bytes();
        let value = String::from("alfa");
        let result = assert_io_read_to_string_ge_x_as_result!(reader, &value);
        assert_eq!(result.unwrap(), String::from("bravo"));
    }

    #[test]
    fn eq() {
        let mut reader = "alfa".as_bytes();
        let value = String::from("alfa");
        let result = assert_io_read_to_string_ge_x_as_result!(reader, &value);
        assert_eq!(result.unwrap(), String::from("alfa"));
    }

    #[test]
    fn lt() {
        let mut reader = "alfa".as_bytes();
        let value = String::from("bravo");
        let result = assert_io_read_to_string_ge_x_as_result!(reader, &value);
        assert_eq!(
            result.unwrap_err(),
            concat!(
                "assertion failed: `assert_io_read_to_string_ge_x!(a_reader, b_expr)`\n",
                "https://docs.rs/assertables/9.2.0/assertables/macro.assert_io_read_to_string_ge_x.html\n",
                " a_reader label: `reader`,\n",
                " a_reader debug: `[]`,\n",
                "   b_expr label: `&value`,\n",
                "   b_expr debug: `\"bravo\"`,\n",
                "              a: `\"alfa\"`,\n",
                "              b: `\"bravo\"`"
            )
        );
    }
}

/// Assert a ::std::io::Read read_to_string() value is greater than or equal to an expression.
///
/// Pseudocode:<br>
/// (reader.read_to_string(a_string) ⇒ a_string) ≥ (expr ⇒ b_string)
///
/// * If true, return `a_string`.
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
/// let mut reader = "bravo".as_bytes();
/// let value = String::from("alfa");
/// assert_io_read_to_string_ge_x!(reader, &value);
///
/// # let result = panic::catch_unwind(|| {
/// // This will panic
/// let mut reader = "alfa".as_bytes();
/// let value = String::from("bravo");
/// assert_io_read_to_string_ge_x!(reader, &value);
/// # });
/// // assertion failed: `assert_io_read_to_string_ge_x!(a_reader, b_expr)`
/// // https://docs.rs/assertables/9.2.0/assertables/macro.assert_io_read_to_string_ge_x.html
/// //  a_reader label: `reader`,
/// //  a_reader debug: `[]`,
/// //    b_expr label: `&value`,
/// //    b_expr debug: `\"bravo\"`,
/// //               a: `\"alfa\"`,
/// //               b: `\"bravo\"`
/// # let actual = result.unwrap_err().downcast::<String>().unwrap().to_string();
/// # let expect = concat!(
/// #     "assertion failed: `assert_io_read_to_string_ge_x!(a_reader, b_expr)`\n",
/// #     "https://docs.rs/assertables/9.2.0/assertables/macro.assert_io_read_to_string_ge_x.html\n",
/// #     " a_reader label: `reader`,\n",
/// #     " a_reader debug: `[]`,\n",
/// #     "   b_expr label: `&value`,\n",
/// #     "   b_expr debug: `\"bravo\"`,\n",
/// #     "              a: `\"alfa\"`,\n",
/// #     "              b: `\"bravo\"`"
/// # );
/// # assert_eq!(actual, expect);
/// # }
/// ```
///
/// # Module macros
///
/// * [`assert_io_read_to_string_ge_x`](macro@crate::assert_io_read_to_string_ge_x)
/// * [`assert_io_read_to_string_ge_x_as_result`](macro@crate::assert_io_read_to_string_ge_x_as_result)
/// * [`debug_assert_io_read_to_string_ge_x`](macro@crate::debug_assert_io_read_to_string_ge_x)
///
#[macro_export]
macro_rules! assert_io_read_to_string_ge_x {
    ($a_reader:expr,  $b_expr:expr $(,)?) => {{
        match $crate::assert_io_read_to_string_ge_x_as_result!($a_reader, $b_expr) {
            Ok(x) => x,
            Err(err) => panic!("{}", err),
        }
    }};
    ($a_reader:expr, $b_expr:expr, $($message:tt)+) => {{
        match $crate::assert_io_read_to_string_ge_x_as_result!($a_reader, $b_expr) {
            Ok(x) => x,
            Err(_err) => panic!("{}", $($message)+),
        }
    }};
}

/// Assert zzz.
///
/// Pseudocode:<br>
/// (reader.read_to_string(a_string) ⇒ a_string) ≥ (expr ⇒ b_string)
///
/// This macro provides the same statements as [`assert_io_read_to_string_ge_x`](macro.assert_io_read_to_string_ge_x.html),
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
/// * [`assert_io_read_to_string_ge_x`](macro@crate::assert_io_read_to_string_ge_x)
/// * [`assert_io_read_to_string_ge_x`](macro@crate::assert_io_read_to_string_ge_x)
/// * [`debug_assert_io_read_to_string_ge_x`](macro@crate::debug_assert_io_read_to_string_ge_x)
///
#[macro_export]
macro_rules! debug_assert_io_read_to_string_ge_x {
    ($($arg:tt)*) => {
        if $crate::cfg!(debug_assertions) {
            $crate::assert_io_read_to_string_ge_x!($($arg)*);
        }
    };
}
