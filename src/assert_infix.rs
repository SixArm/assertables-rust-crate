//! Assert a infix operator, such as assert_infix!(a == b).
//!
//! * If true, return `()`.
//!
//! * Otherwise, call [`panic!`] with a message and the values of the
//!   expressions with their debug representations.
//! 
//! Compare values via infix value operator:
//! 
//! * `assert_infix!(a == b)` ≈ a == b
//! 
//! * `assert_infix!(a != b)` ≈ a ≠ b
//! 
//! * `assert_infix!(a < b)` ≈ a < b
//! 
//! * `assert_infix!(a <= b)` ≈ a ≤ b
//! 
//! * `assert_infix!(a > b)` ≈ a > b
//! 
//! * `assert_infix!(a >= b)` ≈ a ≥ b
//! 
//! Relate values via infix logical operator:
//! 
//! * `assert_infix!(a & b)` ≈ a ∧ b ≈ a AND b
//! 
//! * `assert_infix!(a | b)` ≈ a ∨ b ≈ a OR b
//! 
//! * `assert_infix!(a ^ b)` ≈ a ⊻ b ≈ a XOR b
//! 
//! * `assert_infix!(a && b)` ≈ a …∧ b ≈ a lazy AND b
//! 
//! * `assert_infix!(a || b)` ≈ a …∨ b ≈ a lazy OR b
//! 
//! 
//! # Examples
//!
//! ```rust
//! # #[macro_use] extern crate assertables;
//! # use std::panic;
//! # fn main() {
//! // Return Ok
//! let a = 1;
//! let b = 1;
//! assert_infix!(a == b);
//! //-> ()
//!
//! let a = 1;
//! let b = 2;
//! // Panic with error message
//! let result = panic::catch_unwind(|| {
//! assert_infix!(a == b);
//! //-> panic!
//! });
//! assert!(result.is_err());
//! let actual = result.unwrap_err().downcast::<String>().unwrap().to_string();
//! let expect = concat!(
//!     "assertion failed: `assert_infix!(x == y)`\n",
//!     " x label: `a`,\n",
//!     " x debug: `1`,\n",
//!     " y label: `b`,\n",
//!     " y debug: `2`\n",
//! );
//! assert_eq!(actual, expect);
//!
//! // Panic with error message
//! let result = panic::catch_unwind(|| {
//! assert_infix!(a == b, "message");
//! //-> panic!
//! });
//! assert!(result.is_err());
//! let actual = result.unwrap_err().downcast::<String>().unwrap().to_string();
//! let expect = "message";
//! assert_eq!(actual, expect);
//! # }
//! ```
//!
//! # Infix operators
//!
//! For values:
//!
//! * `==`  equal
//! * `!=`  not equal
//! * `<`	less than
//! * `<=`	less than or equal to
//! * `>`	greater than
//! * `>=`	greater than or equal to
//!
//! For booleans:
//!
//! * `^`	logical XOR
//! * `!`	logical NOT
//! * `&`	logical AND
//! * `|`	logical OR
//! * `&&`	logical lazy AND
//! * `||`	logical lazy OR
//!
//! # Module macros
//!
//! * [`assert_infix`](macro@crate::assert_infix)
//! * [`assert_infix_as_result`](macro@crate::assert_infix_as_result)
//! * [`debug_assert_infix`](macro@crate::debug_assert_infix)
//!

/// Assert a infix operator, such as assert_infix!(a == b).
///
/// * If true, return Result `Ok(())`.
///
/// * Otherwise, return Result `Err` with a diagnostic message.
///
/// This macro provides the same statements as [`assert_infix`](macro.assert_infix.html),
/// except this macro returns a Result, rather than doing a panic.
///
/// This macro is useful for runtime checks, such as checking parameters,
/// or sanitizing inputs, or handling different results in different ways.
///
/// # Module macros
///
/// * [`assert_infix`](macro@crate::assert_infix)
/// * [`assert_infix_as_result`](macro@crate::assert_infix_as_result)
/// * [`debug_assert_infix`](macro@crate::debug_assert_infix)
///
#[macro_export]
macro_rules! assert_infix_as_result {
    ($x:tt $infix:tt $y:tt) => {{
        if $x $infix $y {
            Ok(())
        } else {
            Err(format!(
                concat!(
                    "assertion failed: `assert_infix!(x {} y)`\n",
                    " x label: `{}`,\n",
                    " x debug: `{:?}`,\n",
                    " y label: `{}`,\n",
                    " y debug: `{:?}`\n",
                ),
                stringify!($infix),
                stringify!($x),
                $x,
                stringify!($y),
                $y,
            ))
        }
    }};
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_assert_infix_as_result_x_success() {
        let a: i32 = 1;
        let b: i32 = 1;
        let result = assert_infix_as_result!(a == b);
        assert_eq!(result, Ok(()));
        let result = assert_infix_as_result!(a >= b);
        assert_eq!(result, Ok(()));
    }

    #[test]
    fn test_assert_infix_as_result_x_failure() {
        let a: i32 = 1;
        let b: i32 = 2;
        let result = assert_infix_as_result!(a == b);
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            concat!(
                "assertion failed: `assert_infix!(x == y)`\n",
                " x label: `a`,\n",
                " x debug: `1`,\n",
                " y label: `b`,\n",
                " y debug: `2`\n",
            )
        );
        let result = assert_infix_as_result!(a >= b);
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            concat!(
                "assertion failed: `assert_infix!(x >= y)`\n",
                " x label: `a`,\n",
                " x debug: `1`,\n",
                " y label: `b`,\n",
                " y debug: `2`\n",
            )
        );
    }
}

/// Assert a infix operator, such as assert_infix!(a == b).
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
/// let a = 1;
/// let b = 1;
/// assert_infix!(a == b);
/// //-> ()
///
/// let a = 1;
/// let b = 2;
/// // Panic with error message
/// let result = panic::catch_unwind(|| {
/// assert_infix!(a == b);
/// //-> panic!
/// });
/// assert!(result.is_err());
/// let actual = result.unwrap_err().downcast::<String>().unwrap().to_string();
/// let expect = concat!(
///     "assertion failed: `assert_infix!(x == y)`\n",
///     " x label: `a`,\n",
///     " x debug: `1`,\n",
///     " y label: `b`,\n",
///     " y debug: `2`\n",
/// );
/// assert_eq!(actual, expect);
///
/// // Panic with error message
/// let result = panic::catch_unwind(|| {
/// assert_infix!(a == b, "message");
/// //-> panic!
/// });
/// assert!(result.is_err());
/// let actual = result.unwrap_err().downcast::<String>().unwrap().to_string();
/// let expect = "message";
/// assert_eq!(actual, expect);
/// # }
/// ```
///
/// # Infix operators
///
/// For values:
///
/// * `==`  equal
/// * `!=`  not equal
/// * `<`	less than
/// * `<=`	less than or equal to
/// * `>`	greater than
/// * `>=`	greater than or equal to
///
/// For booleans:
///
/// * `^`	logical XOR
/// * `!`	logical NOT
/// * `&`	logical AND
/// * `|`	logical OR
/// * `&&`	logical lazy AND
/// * `||`	logical lazy OR
///
/// # Module macros
///
/// * [`assert_infix`](macro@crate::assert_infix)
/// * [`assert_infix_as_result`](macro@crate::assert_infix_as_result)
/// * [`debug_assert_infix`](macro@crate::debug_assert_infix)
///
#[macro_export]
macro_rules! assert_infix {
    ($x:tt $infix:tt $y:tt) => {
        match assert_infix_as_result!($x $infix $y) {
            Ok(()) => (),
            Err(err) => panic!("{}", err),
        }
    };
    ($x:tt $infix:tt $y:tt, $($message:tt)+) => {
        match assert_infix_as_result!($x $infix $y) {
            Ok(()) => (),
            Err(_err) => panic!("{}", $($message)+),
        }
    };
}

/// Assert a infix operator, such as assert_infix!(a == b).
///
/// This macro provides the same statements as [`assert_infix`](macro.assert_infix.html),
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
/// * [`assert_infix`](macro@crate::assert_infix)
/// * [`assert_infix`](macro@crate::assert_infix)
/// * [`debug_assert_infix`](macro@crate::debug_assert_infix)
///
#[macro_export]
macro_rules! debug_assert_infix {
    ($($arg:tt)*) => {
        if $crate::cfg!(debug_assertions) {
            $crate::assert_infix!($($arg)*);
        }
    };
}
