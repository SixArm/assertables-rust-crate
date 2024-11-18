//! Assert an expression is greater than another.
//!
//! Pseudocode:<br>
//! a > b
//!
//! # Example
//!
//! ```rust
//! use assertables::*;
//!
//! let a = 2;
//! let b = 1;
//! assert_gt!(a, b);
//! ```
//!
//! # Module macros
//!
//! * [`assert_gt`](macro@crate::assert_gt)
//! * [`assert_gt_as_result`](macro@crate::assert_gt_as_result)
//! * [`debug_assert_gt`](macro@crate::debug_assert_gt)

/// Assert an expression is greater than another.
///
/// Pseudocode:<br>
/// a > b
///
/// * If true, return Result `Ok(())`.
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
/// * [`assert_gt`](macro@crate::assert_gt)
/// * [`assert_gt_as_result`](macro@crate::assert_gt_as_result)
/// * [`debug_assert_gt`](macro@crate::debug_assert_gt)
///
#[macro_export]
macro_rules! assert_gt_as_result {
    ($a:expr, $b:expr $(,)?) => {{
        match (&$a, &$b) {
            (a, b) => {
                if a > b {
                    Ok(())
                } else {
                    Err(format!(
                        concat!(
                            "assertion failed: `assert_gt!(a, b)`\n",
                            "https://docs.rs/assertables/9.4.0/assertables/macro.assert_gt.html\n",
                            " a label: `{}`,\n",
                            " a debug: `{:?}`,\n",
                            " b label: `{}`,\n",
                            " b debug: `{:?}`",
                        ),
                        stringify!($a),
                        a,
                        stringify!($b),
                        b
                    ))
                }
            }
        }
    }};
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_assert_gt_as_result_success() {
        let a: i32 = 2;
        let b: i32 = 1;
        let result = assert_gt_as_result!(a, b);
        assert_eq!(result, Ok(()));
    }

    #[test]
    fn test_assert_gt_as_result_failure() {
        let a: i32 = 1;
        let b: i32 = 2;
        let result = assert_gt_as_result!(a, b);
        assert_eq!(
            result.unwrap_err(),
            concat!(
                "assertion failed: `assert_gt!(a, b)`\n",
                "https://docs.rs/assertables/9.4.0/assertables/macro.assert_gt.html\n",
                " a label: `a`,\n",
                " a debug: `1`,\n",
                " b label: `b`,\n",
                " b debug: `2`",
            )
        );
    }
}

/// Assert an expression is greater than another.
///
/// Pseudocode:<br>
/// a > b
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
///
/// # fn main() {
/// let a = 2;
/// let b = 1;
/// assert_gt!(a, b);
///
/// # let result = panic::catch_unwind(|| {
/// // This will panic
/// let a = 1;
/// let b = 2;
/// assert_gt!(a, b);
/// # });
/// // assertion failed: `assert_gt!(a, b)`
/// // https://docs.rs/assertables/9.4.0/assertables/macro.assert_gt.html
/// //  a label: `a`,
/// //  a debug: `1`,
/// //  b label: `b`,
/// //  b debug: `2`
/// # let actual = result.unwrap_err().downcast::<String>().unwrap().to_string();
/// # let expect = concat!(
/// #     "assertion failed: `assert_gt!(a, b)`\n",
/// #     "https://docs.rs/assertables/9.4.0/assertables/macro.assert_gt.html\n",
/// #     " a label: `a`,\n",
/// #     " a debug: `1`,\n",
/// #     " b label: `b`,\n",
/// #     " b debug: `2`",
/// # );
/// # assert_eq!(actual, expect);
/// # }
/// ```
///
/// # Module macros
///
/// * [`assert_gt`](macro@crate::assert_gt)
/// * [`assert_gt_as_result`](macro@crate::assert_gt_as_result)
/// * [`debug_assert_gt`](macro@crate::debug_assert_gt)
///
#[macro_export]
macro_rules! assert_gt {
    ($a:expr, $b:expr $(,)?) => {{
        match $crate::assert_gt_as_result!($a, $b) {
            Ok(()) => (),
            Err(err) => panic!("{}", err),
        }
    }};
    ($a:expr, $b:expr, $($message:tt)+) => {{
        match $crate::assert_gt_as_result!($a, $b) {
            Ok(()) => (),
            Err(err) => panic!("{}\n{}", format_args!($($message)+), err),
        }
    }};
}

/// Assert an expression is greater than another.
///
/// Pseudocode:<br>
/// a > b
///
/// This macro provides the same statements as [`assert_gt`](macro.assert_gt.html),
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
/// * [`assert_gt`](macro@crate::assert_gt)
/// * [`assert_gt`](macro@crate::assert_gt)
/// * [`debug_assert_gt`](macro@crate::debug_assert_gt)
///
#[macro_export]
macro_rules! debug_assert_gt {
    ($($arg:tt)*) => {
        if $crate::cfg!(debug_assertions) {
            $crate::assert_gt!($($arg)*);
        }
    };
}
