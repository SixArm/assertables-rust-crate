//! Assert a.is_some() and a.unwrap() are equal to another.
//!
//! # Example
//!
//! ```rust
//! # #[macro_use] extern crate assertables;
//! # fn main() {
//! let a: Option<i8> = Option::Some(1);
//! let b: Option<i8> = Option::Some(1);
//! assert_option_some_eq!(a, b);
//! # }
//! ```
//!
//! # Module macros
//!
//! * [`assert_option_some_eq`](macro@crate::assert_option_some_eq)
//! * [`assert_option_some_eq_as_result`](macro@crate::assert_option_some_eq_as_result)
//! * [`debug_assert_option_some_eq`](macro@crate::debug_assert_option_some_eq)

/// Assert a.is_some() and a.unwrap() are equal to another.
///
/// * If true, return Option `Some(())`.
///
/// * Otherwise, return Option `Err` with a diagnostic message.
///
/// This macro provides the same statements as [`assert_option_some_eq`](macro.assert_option_some_eq.html),
/// except this macro returns a Option, rather than doing a panic.
///
/// This macro is useful for runtime checks, such as checking parameters,
/// or sanitizing inputs, or handling different results in different ways.
///
/// # Module macros
///
/// * [`assert_option_some_eq`](macro@crate::assert_option_some_eq)
/// * [`assert_option_some_eq_as_result`](macro@crate::assert_option_some_eq_as_result)
/// * [`debug_assert_option_some_eq`](macro@crate::debug_assert_option_some_eq)
///
#[macro_export]
macro_rules! assert_option_some_eq_as_result {
    ($a_option:expr, $b_option:expr $(,)?) => {{
        match (&$a_option, &$b_option) {
            (a_option, b_option) => {
                if a_option.is_none() || b_option.is_none() {
                    Err(format!(
                        concat!(
                            "assertion failed: `assert_option_some_eq!(a, b)`\n",
                            " a label: `{}`,\n",
                            " a debug: `{:?}`,\n",
                            " b label: `{}`,\n",
                            " b debug: `{:?}`",
                        ),
                        stringify!($a_option),
                        $a_option,
                        stringify!($b_option),
                        $b_option,
                    ))
                } else {
                    let a = a_option.unwrap();
                    let b = b_option.unwrap();
                    if a == b {
                        Ok(())
                    } else {
                        Err(format!(
                            concat!(
                                "assertion failed: `assert_option_some_eq!(a, b)`\n",
                                " a label: `{}`,\n",
                                " a debug: `{:?}`,\n",
                                " b label: `{}`,\n",
                                " b debug: `{:?}`,\n",
                                "       a: `{:?}`,\n",
                                "       b: `{:?}`"
                            ),
                            stringify!($a_option),
                            $a_option,
                            stringify!($b_option),
                            $b_option,
                            a,
                            b
                        ))
                    }
                }
            }
        }
    }};
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_assert_option_some_eq_as_result_x_success() {
        let a: Option<i8> = Option::Some(1);
        let b: Option<i8> = Option::Some(1);
        let result = assert_option_some_eq_as_result!(a, b);
        assert_eq!(result, Ok(()));
    }

    #[test]
    fn test_assert_option_some_eq_as_result_x_failure_because_ne() {
        let a: Option<i8> = Option::Some(1);
        let b: Option<i8> = Option::Some(2);
        let result = assert_option_some_eq_as_result!(a, b);
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            concat!(
                "assertion failed: `assert_option_some_eq!(a, b)`\n",
                " a label: `a`,\n",
                " a debug: `Some(1)`,\n",
                " b label: `b`,\n",
                " b debug: `Some(2)`,\n",
                "       a: `1`,\n",
                "       b: `2`",
            )
        );
    }

    #[test]
    fn test_assert_option_some_eq_as_result_x_failure_because_none() {
        let a: Option<i8> = Option::Some(1);
        let b: Option<i8> = Option::None;
        let result = assert_option_some_eq_as_result!(a, b);
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            concat!(
                "assertion failed: `assert_option_some_eq!(a, b)`\n",
                " a label: `a`,\n",
                " a debug: `Some(1)`,\n",
                " b label: `b`,\n",
                " b debug: `None`",
            )
        );
    }

}

/// Assert a.is_some() and a.unwrap() are equal to another.
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
/// let b: Option<i8> = Option::Some(1);
/// assert_option_some_eq!(a, b);
/// //-> ()
///
/// // Panic with error message
/// let a: Option<i8> = Option::Some(1);
/// let b: Option<i8> = Option::Some(2);
/// let result = panic::catch_unwind(|| {
/// assert_option_some_eq!(a, b);
/// //-> panic!
/// });
/// assert!(result.is_err());
/// let actual = result.unwrap_err().downcast::<String>().unwrap().to_string();
/// let expect = concat!(
///     "assertion failed: `assert_option_some_eq!(a, b)`\n",
///     " a label: `a`,\n",
///     " a debug: `Some(1)`,\n",
///     " b label: `b`,\n",
///     " b debug: `Some(2)`,\n",
///     "       a: `1`,\n",
///     "       b: `2`",
/// );
/// assert_eq!(actual, expect);
///
/// // Panic with error message
/// let result = panic::catch_unwind(|| {
/// assert_option_some_eq!(a, b, "message");
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
/// * [`assert_option_some_eq`](macro@crate::assert_option_some_eq)
/// * [`assert_option_some_eq_as_result`](macro@crate::assert_option_some_eq_as_result)
/// * [`debug_assert_option_some_eq`](macro@crate::debug_assert_option_some_eq)
///
#[macro_export]
macro_rules! assert_option_some_eq {
    ($a:expr, $b:expr $(,)?) => ({
        match assert_option_some_eq_as_result!($a, $b) {
            Ok(()) => (),
            Err(err) => panic!("{}", err),
        }
    });
    ($a:expr, $b:expr, $($message:tt)+) => ({
        match assert_option_some_eq_as_result!($a, $b) {
            Ok(()) => (),
            Err(_err) => panic!("{}", $($message)+),
        }
    });
}

/// Assert a.is_some() and a.unwrap() are equal to another.
///
/// This macro provides the same statements as [`assert_option_some_eq`](macro.assert_option_some_eq.html),
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
/// * [`assert_option_some_eq`](macro@crate::assert_option_some_eq)
/// * [`assert_option_some_eq`](macro@crate::assert_option_some_eq)
/// * [`debug_assert_option_some_eq`](macro@crate::debug_assert_option_some_eq)
///
#[macro_export]
macro_rules! debug_assert_option_some_eq {
    ($($arg:tt)*) => {
        if $crate::cfg!(debug_assertions) {
            $crate::assert_option_some_eq!($($arg)*);
        }
    };
}
