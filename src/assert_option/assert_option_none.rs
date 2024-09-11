//! Assert expression.is_none() is true.
//!
//! # Example
//!
//! ```rust
//! # #[macro_use] extern crate assertables;
//! # fn main() {
//! let a: Option<i8> = Option::None;
//! assert_option_none!(a);
//! # }
//! ```
//!
//! # Module macros
//!
//! * [`assert_option_none`](macro@crate::assert_option_none)
//! * [`assert_option_none_as_result`](macro@crate::assert_option_none_as_result)
//! * [`debug_assert_option_none`](macro@crate::debug_assert_option_none)

/// Assert an expression.is_none() is true.
///
/// * If true, return Result `Ok(())`.
///
/// * Otherwise, return Result `Err` with a diagnostic message.
///
/// This macro provides the same statements as [`assert_option_none`](macro.assert_option_none.html),
/// except this macro returns a Result, rather than doing a panic.
///
/// This macro is useful for runtime checks, such as checking parameters,
/// or sanitizing inputs, or handling different results in different ways.
///
/// # Module macros
///
/// * [`assert_option_none`](macro@crate::assert_option_none)
/// * [`assert_option_none_as_result`](macro@crate::assert_option_none_as_result)
/// * [`debug_assert_option_none`](macro@crate::debug_assert_option_none)
///
#[macro_export]
macro_rules! assert_option_none_as_result {
    ($a:expr $(,)?) => {{
        match (&$a) {
            a_val => {
                let is_none = a_val.is_none();
                if is_none {
                    Ok(())
                } else {
                    Err(format!(
                        concat!(
                            "assertion failed: `assert_option_none!(expr)`\n",
                            "     expr label: `{}`,\n",
                            "     expr debug: `{:?}`,\n",
                            " expr.is_none(): `{:?}`",
                        ),
                        stringify!($a),
                        $a,
                        is_none,
                    ))
                }
            }
        }
    }};
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_assert_option_none_as_result_x_success() {
        let a: Option<i8> = Option::None;
        let result = assert_option_none_as_result!(a);
        assert_eq!(result, Ok(()));
    }

    #[test]
    fn test_assert_option_none_as_result_x_failure() {
        let a: Option<i8> = Option::Some(1);
        let result = assert_option_none_as_result!(a);
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            concat!(
                "assertion failed: `assert_option_none!(expr)`\n",
                "     expr label: `a`,\n",
                "     expr debug: `Some(1)`,\n",
                " expr.is_none(): `false`"
            )
        );
    }
}

/// Assert expression.is_none() is true.
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
/// # fn main() {
/// let a: Option<i8> = Option::None;
/// assert_option_none!(a);
/// //-> ()
///
/// let a: Option<i8> = Option::Some(1);
/// // Panic with error message
/// let result = panic::catch_unwind(|| {
/// assert_option_none!(a);
/// //-> panic!
/// });
/// assert!(result.is_err());
/// let actual = result.unwrap_err().downcast::<String>().unwrap().to_string();
/// let expect = concat!(
///     "assertion failed: `assert_option_none!(expr)`\n",
///     "     expr label: `a`,\n",
///     "     expr debug: `Some(1)`,\n",
///     " expr.is_none(): `false`",
/// );
/// assert_eq!(actual, expect);
///
/// // Panic with error message
/// let result = panic::catch_unwind(|| {
/// assert_option_none!(a, "message");
/// //-> panic!
/// });
/// assert!(result.is_err());
/// let actual = result.unwrap_err().downcast::<String>().unwrap().to_string();
/// let expect = "message";
/// assert_eq!(actual, expect);
/// # }
/// ```
///
/// # Module macros
///
/// * [`assert_option_none`](macro@crate::assert_option_none)
/// * [`assert_option_none_as_result`](macro@crate::assert_option_none_as_result)
/// * [`debug_assert_option_none`](macro@crate::debug_assert_option_none)
///
#[macro_export]
macro_rules! assert_option_none {
    ($a:expr $(,)?) => ({
        match assert_option_none_as_result!($a) {
            Ok(()) => (),
            Err(err) => panic!("{}", err),
        }
    });
    ($a:expr, $($message:tt)+) => ({
        match assert_option_none_as_result!($a) {
            Ok(()) => (),
            Err(_err) => panic!("{}", $($message)+),
        }
    });
}

/// Assert expression.is_none() is true.
///
/// This macro provides the same statements as [`assert_option_none`](macro.assert_option_none.html),
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
/// * [`assert_option_none`](macro@crate::assert_option_none)
/// * [`assert_option_none`](macro@crate::assert_option_none)
/// * [`debug_assert_option_none`](macro@crate::debug_assert_option_none)
///
#[macro_export]
macro_rules! debug_assert_option_none {
    ($($arg:tt)*) => {
        if $crate::cfg!(debug_assertions) {
            $crate::assert_option_none!($($arg)*);
        }
    };
}
