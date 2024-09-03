//! Assert a condition is true.
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
//! // Return Ok
//! assert!(true);
//! //-> ()
//! # }
//! ```
//!
//! ```rust
//! # #[macro_use] extern crate assertables;
//! # use std::panic;
//! # fn main() {
//! // Panic with error message
//! let result = panic::catch_unwind(|| {
//! assert!(false);
//! //-> panic!
//! });
//! assert!(result.is_err());
//! # }
//! ```
//! 
//! The `assert` macro is a Rust built-in.

/// Assert a condition is true.
///
/// * If true, return Result `Ok(())`.
///
/// * Otherwise, return Result `Err` with a diagnostic message.
///
/// This macro provides the same statements as [`assert_`](macro.assert_.html),
/// except this macro returns a Result, rather than doing a panic.
///
/// This macro is useful for runtime checks, such as checking parameters,
/// or sanitizing inputs, or handling different results in different ways.
///
/// # Module macros
///
/// * [`assert`]
/// * [`assert_as_result`](macro.assert_as_result.html)
/// * [`debug_assert`]
///
#[macro_export]
macro_rules! assert_as_result {
    ($x:expr $(,)?) => {{
        if $x {
            Ok(())
        } else {
            Err(format!(
                concat!(
                    "assertion failed: `assertable!(condition)`\n",
                    " condition: `{:?}`"
                ),
                $x
            ))
        }
    }};
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_assert_x_success_as_result() {
        let a = true;
        let x = assert_as_result!(a);
        assert_eq!(x, Ok(()));
    }
}

/// Assert a condition is true.
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
/// // Return Ok
/// assert!(true);
/// //-> ()
///
/// // Panic with error message
/// let result = panic::catch_unwind(|| {
/// assert!(false);
/// //-> panic!
/// });
/// assert!(result.is_err());
/// let actual = result.unwrap_err().downcast::<String>().unwrap().to_string();
/// let expect = "assertion failed: false";
/// assert_eq!(actual, expect);
/// # }
/// ```
///
/// The `assert` macro is a Rust built-in.

#[cfg(test)]
mod test_assert_x_result {

    #[test]
    fn test_assert_x_success() {
        let a = true;
        let x = assert!(a);
        assert_eq!(x, ());
    }

    #[test]
    #[should_panic]
    fn test_assert_x_failure() {
        let a = false;
        let _x = assert!(a);
    }

    #[test]
    fn test_assert_x_arity_3_success() {
        let a = true;
        let x = assert!(a, "message");
        assert_eq!(x, ());
    }

    #[test]
    #[should_panic]
    fn test_assert_x_arity_3_failure() {
        let a = false;
        let _x = assert!(a, "message");
    }
}

/// Assert zzz.
///
/// This macro provides the same statements as [`assert_zzz`](macro.assert_zzz.html),
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
/// * [`assert_zzz`](macro.assert_zzz.html)
/// * [`assert_zzz`](macro.assert_zzz.html)
/// * [`debug_assert_zzz`](macro.debug_assert_zzz.html)
///
#[macro_export]
macro_rules! debug_assert_zzz {
    ($($arg:tt)*) => {
        if $crate::cfg!(debug_assertions) {
            $crate::assert_zzz!($($arg)*);
        }
    };
}
