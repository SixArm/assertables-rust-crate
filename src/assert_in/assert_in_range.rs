//! Assert an item is in a range.
//!
//! Pseudocode:<br>
//! a is in range
//!
//! # Example
//!
//! ```rust
//! use assertables::*;
//!
//! let a = 1;
//! let b = 0..2;
//! assert_in_range!(a, b);
//! ```
//!
//! # Module macros
//!
//! * [`assert_in_range`](macro@crate::assert_in_range)
//! * [`assert_in_range_as_result`](macro@crate::assert_in_range_as_result)
//! * [`debug_assert_in_range`](macro@crate::debug_assert_in_range)

/// Assert an item is in a range.
///
/// Pseudocode:<br>
/// a is in range
///
/// * If true, return Result `Ok(())`.
///
/// * Otherwise, return Result `Err(message)`.
///
/// This macro provides the same statements as [`assert_in_range`](macro.assert_in_range.html),
/// except this macro returns a Result, rather than doing a panic.
///
/// This macro is useful for runtime checks, such as checking parameters,
/// or sanitizing inputs, or handling different results in different ways.
///
/// # Module macros
///
/// * [`assert_in_range`](macro@crate::assert_in_range)
/// * [`assert_in_range_as_result`](macro@crate::assert_in_range_as_result)
/// * [`debug_assert_in_range`](macro@crate::debug_assert_in_range)
///
#[macro_export]
macro_rules! assert_in_range_as_result {
    ($a:expr, $range:expr $(,)?) => {{
        if $range.contains(&$a) {
            Ok(())
        } else {
            Err(format!(
                concat!(
                    "assertion failed: `assert_in_range!(a, range)`\n",
                    "https://docs.rs/assertables/9.4.0/assertables/macro.assert_in_range.html\n",
                    "     a label: `{}`,\n",
                    "     a debug: `{:?}`,\n",
                    " range label: `{}`,\n",
                    " range debug: `{:?}`,\n",
                    " range start: `{:?}`,\n",
                    "   range end: `{:?}`",
                ),
                stringify!($a),
                $a,
                stringify!($range),
                $range,
                $range.start,
                $range.end,
            ))
        }
    }};
}

#[cfg(test)]
mod tests {

    #[test]
    fn success() {
        let a = 1;
        let b = 0..2;
        let result = assert_in_range_as_result!(a, b);
        assert_eq!(result.unwrap(), ());
    }

    #[test]
    fn failure() {
        let a = 1;
        let b = 2..4;
        let result = assert_in_range_as_result!(a, b);
        let actual = result.unwrap_err();
        let expect = concat!(
            "assertion failed: `assert_in_range!(a, range)`\n",
            "https://docs.rs/assertables/9.4.0/assertables/macro.assert_in_range.html\n",
            "     a label: `a`,\n",
            "     a debug: `1`,\n",
            " range label: `b`,\n",
            " range debug: `2..4`,\n",
            " range start: `2`,\n",
            "   range end: `4`"
        );
        assert_eq!(actual, expect);
    }
}

/// Assert an item is in a range.
///
/// Pseudocode:<br>
/// a is in range
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
/// let a = 1;
/// let b = 0..2;
/// assert_in_range!(a, b);
///
/// # let result = panic::catch_unwind(|| {
/// // This will panic
/// let a = 1;
/// let b = 2..4;
/// assert_in_range!(a, b);
/// # });
/// // assertion failed: `assert_in_range!(a, range)`
/// // https://docs.rs/assertables/9.4.0/assertables/macro.assert_in_range.html
/// //  a label: `a`,
/// //  a debug: `1`,
/// //  range label: `b`,
/// //  range debug: `2..4`,
/// //  range start: `2`,
/// //    range end: `4`"
/// # let actual = result.unwrap_err().downcast::<String>().unwrap().to_string();
/// # let expect = concat!(
/// #     "assertion failed: `assert_in_range!(a, range)`\n",
/// #     "https://docs.rs/assertables/9.4.0/assertables/macro.assert_in_range.html\n",
/// #     "     a label: `a`,\n",
/// #     "     a debug: `1`,\n",
/// #     " range label: `b`,\n",
/// #     " range debug: `2..4`,\n",
/// #     " range start: `2`,\n",
/// #     "   range end: `4`"
/// # );
/// # assert_eq!(actual, expect);
/// # }
/// ```
///
/// # Module macros
///
/// * [`assert_in_range`](macro@crate::assert_in_range)
/// * [`assert_in_range_as_result`](macro@crate::assert_in_range_as_result)
/// * [`debug_assert_in_range`](macro@crate::debug_assert_in_range)
///
#[macro_export]
macro_rules! assert_in_range {
    ($a:expr, $range:expr $(,)?) => {{
        match $crate::assert_in_range_as_result!($a, $range) {
            Ok(()) => (),
            Err(err) => panic!("{}", err),
        }
    }};
    ($a:expr, $range:expr, $($message:tt)+) => {{
        match $crate::assert_in_range_as_result!($a, $range) {
            Ok(()) => (),
            Err(err) => panic!("{}\n{}", format_args!($($message)+), err),
        }
    }};
}

/// Assert an item is in a range.
///
/// Pseudocode:<br>
/// a is in range
///
/// This macro provides the same statements as [`assert_in_range`](macro.assert_in_range.html),
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
/// * [`assert_in_range`](macro@crate::assert_in_range)
/// * [`assert_in_range`](macro@crate::assert_in_range)
/// * [`debug_assert_in_range`](macro@crate::debug_assert_in_range)
///
#[macro_export]
macro_rules! debug_assert_in_range {
    ($($arg:tt)*) => {
        if $crate::cfg!(debug_assertions) {
            $crate::assert_in_range!($($arg)*);
        }
    };
}
