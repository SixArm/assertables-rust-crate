//! Assert expression.is_some() is true.
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
//! # fn main() {
//! let a: Option<i8> = Option::Some(1);
//! assert_some!(a);
//! //-> ()
//!
//! let a: Option<i8> = Option::None;
//! // Panic with error message
//! let result = panic::catch_unwind(|| {
//! assert_some!(a);
//! //-> panic!
//! });
//! assert!(result.is_err());
//! let actual = result.unwrap_err().downcast::<String>().unwrap().to_string();
//! let expect = concat!(
//!     "assertion failed: `assert_some!(expr)`\n",
//!     "     expr label: `a`,\n",
//!     "     expr debug: `None`,\n",
//!     " expr.is_some(): `false`",
//! );
//! assert_eq!(actual, expect);
//!
//! // Panic with error message
//! let result = panic::catch_unwind(|| {
//! assert_some!(a, "message");
//! //-> panic!
//! });
//! assert!(result.is_err());
//! let actual = result.unwrap_err().downcast::<String>().unwrap().to_string();
//! let expect = "message";
//! assert_eq!(actual, expect);
//! # }
//! ```
//!
//! # Module macros
//!
//! * [`assert_some`](macro@crate::assert_some)
//! * [`assert_some_as_result`](macro@crate::assert_some_as_result)
//! * [`debug_assert_some`](macro@crate::debug_assert_some)

/// Assert an expression.is_some() is true.
///
/// * If true, return Result `Ok(())`.
///
/// * Otherwise, return Result `Err` with a diagnostic message.
///
/// This macro provides the same statements as [`assert_some`](macro.assert_some.html),
/// except this macro returns a Result, rather than doing a panic.
///
/// This macro is useful for runtime checks, such as checking parameters,
/// or sanitizing inputs, or handling different results in different ways.
///
/// # Module macros
///
/// * [`assert_some`](macro@crate::assert_some)
/// * [`assert_some_as_result`](macro@crate::assert_some_as_result)
/// * [`debug_assert_some`](macro@crate::debug_assert_some)
///
#[macro_export]
macro_rules! assert_some_as_result {
    ($a:expr $(,)?) => {{
        match (&$a) {
            a_val => {
                let is_some = a_val.is_some();
                if is_some {
                    Ok(())
                } else {
                    Err(format!(
                        concat!(
                            "assertion failed: `assert_some!(expr)`\n",
                            "     expr label: `{}`,\n",
                            "     expr debug: `{:?}`,\n",
                            " expr.is_some(): `{:?}`",
                        ),
                        stringify!($a),
                        $a,
                        is_some,
                    ))
                }
            }
        }
    }};
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_assert_some_as_result_x_success() {
        let a: Option<i8> = Option::Some(1);
        let x = assert_some_as_result!(a);
        assert_eq!(x, Ok(()));
    }

    #[test]
    fn test_assert_some_as_result_x_failure() {
        let a: Option<i8> = Option::None;
        let x = assert_some_as_result!(a);
        assert!(x.is_err());
        assert_eq!(
            x.unwrap_err(),
            concat!(
                "assertion failed: `assert_some!(expr)`\n",
                "     expr label: `a`,\n",
                "     expr debug: `None`,\n",
                " expr.is_some(): `false`"
            )
        );
    }
}

/// Assert expression.is_some() is true.
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
/// let a: Option<i8> = Option::Some(1);
/// assert_some!(a);
/// //-> ()
///
/// let a: Option<i8> = Option::None;
/// // Panic with error message
/// let result = panic::catch_unwind(|| {
/// assert_some!(a);
/// //-> panic!
/// });
/// assert!(result.is_err());
/// let actual = result.unwrap_err().downcast::<String>().unwrap().to_string();
/// let expect = concat!(
///     "assertion failed: `assert_some!(expr)`\n",
///     "     expr label: `a`,\n",
///     "     expr debug: `None`,\n",
///     " expr.is_some(): `false`",
/// );
/// assert_eq!(actual, expect);
///
/// // Panic with error message
/// let result = panic::catch_unwind(|| {
/// assert_some!(a, "message");
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
/// * [`assert_some`](macro@crate::assert_some)
/// * [`assert_some_as_result`](macro@crate::assert_some_as_result)
/// * [`debug_assert_some`](macro@crate::debug_assert_some)
/// 
#[macro_export]
macro_rules! assert_some {
    ($a:expr $(,)?) => ({
        match assert_some_as_result!($a) {
            Ok(()) => (),
            Err(err) => panic!("{}", err),
        }
    });
    ($a:expr, $($message:tt)+) => ({
        match assert_some_as_result!($a) {
            Ok(()) => (),
            Err(_err) => panic!("{}", $($message)+),
        }
    });
}

/// Assert expression.is_some() is true.
///
/// This macro provides the same statements as [`assert_some`](macro.assert_some.html),
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
/// * [`assert_some`](macro@crate::assert_some)
/// * [`assert_some`](macro@crate::assert_some)
/// * [`debug_assert_some`](macro@crate::debug_assert_some)
///
#[macro_export]
macro_rules! debug_assert_some {
    ($($arg:tt)*) => {
        if $crate::cfg!(debug_assertions) {
            $crate::assert_some!($($arg)*);
        }
    };
}