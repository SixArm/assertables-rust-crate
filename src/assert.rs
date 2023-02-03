/// Assert a condition is true.
///
/// * If true, return Result `Ok(())`.
///
/// * Otherwise, return Result `Err` with a diagnostic message.
///
/// This macro provides the same statements as [`assert_`],
/// except this macro returns a Result, rather than doing a panic.
///
/// This macro is useful for runtime checks, such as checking parameters,
/// or santizing inputs, or handling different results in different ways.
///
/// # Related
///
/// * [`assert`]
/// * [`assert_as_result`]
/// * [`debug_assert`]
///
#[macro_export]
macro_rules! assert_as_result {
    ($x:expr $(,)?) => ({
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
    });
}

#[cfg(test)]
mod test_x_result {

    #[test]
    fn test_assert_x_success_as_result() {
        let a = true;
        let x = assert_as_result!(a);
        assert!(x.is_ok());
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
/// let expect = concat!(
///     "assertion failed: false"
/// );
/// assert_eq!(actual, expect);
/// # }
/// ```

// `assert_eq` macro is provided by Rust `std`.

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
/// This macro provides the same statements as [`assert_zzz`],
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
/// This macro is intendend to work in a similar way to
/// [`std::debug_assert`](https://doc.rust-lang.org/std/macro.debug_assert.html).
///
/// # Related
///
/// * [`assert_zzz`]
/// * [`assert_zzz`]
/// * [`debug_assert_zzz`]
///
#[macro_export]
macro_rules! debug_assert_zzz {
    ($($arg:tt)*) => {
        if $crate::cfg!(debug_assertions) {
            $crate::assert_zzz!($($arg)*);
        }
    };
}
