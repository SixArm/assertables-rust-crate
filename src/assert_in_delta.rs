/// Assert a number is within delta of another number.
///
/// * If true, return Result `Ok(())`.
///
/// * When false, return [`Err`] with a message and the values of the
///   expressions with their debug representations.
///
/// This macro provides the same statements as [`assert_`], except this macro
/// returns a Result, rather than doing a panic.
///
/// This macro is useful for runtime checks, such as checking parameters, or
/// santizing inputs, or handling different results in different ways.
///
/// # Related
///
/// * [`assert_in_delta`]
/// * [`assert_in_delta_as_result`]
/// * [`debug_assert_in_delta`]
///
#[macro_export]
macro_rules! assert_in_delta_as_result {
    ($a:expr, $b:expr, $delta:expr $(,)?) => ({
        match (&$a, &$b, &$delta) {
            (a_val, b_val, delta_val) => {
                if a_val == b_val || 
                    ((a_val < b_val) && (b_val - a_val) <= *delta_val) ||
                    ((a_val > b_val) && (a_val - b_val) <= *delta_val)
                {
                    Ok(())
                } else {
                    Err(format!(
                        concat!(
                            "assertion failed: `assert_in_delta!(left, right, delta)`\n",
                            "  left label: `{}`,\n",
                            "  left debug: `{:?}`,\n",
                            " right label: `{}`,\n",
                            " right debug: `{:?}`,\n",
                            " delta label: `{}`,\n",
                            " delta debug: `{:?}`,\n",
                            "        left: `{:?}`,\n",
                            "       right: `{:?}`"
                        ),
                        stringify!($a), $a,
                        stringify!($b), $b,
                        stringify!($delta), $delta,
                        a_val,
                        b_val
                    ))
                }
            }
        }
    });
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_assert_in_delta_as_result_x_success() {
        let a: i8 = 10;
        let b: i8 = 11;
        let delta: i8 = 1;
        let x = assert_in_delta_as_result!(a, b, delta);
        assert_eq!(x, Ok(()));
    }

    #[test]
    fn test_assert_in_delta_as_result_x_failure() {
        let a: i8 = 10;
        let b: i8 = 12;
        let delta: i8 = 1;
        let x = assert_in_delta_as_result!(a, b, delta);
        assert!(x.is_err());
        assert_eq!(
            x.unwrap_err(),
            concat!(
                "assertion failed: `assert_in_delta!(left, right, delta)`\n",
                "  left label: `a`,\n",
                "  left debug: `10`,\n",
                " right label: `b`,\n",
                " right debug: `12`,\n",
                " delta label: `delta`,\n",
                " delta debug: `1`,\n",
                "        left: `10`,\n",
                "       right: `12`"
            )
        );
    }
}

/// Assert a number is within delta of another number.
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
/// let a: i8 = 10;
/// let b: i8 = 11;
/// let delta: i8 = 1;
/// assert_in_delta!(a, b, delta);
/// //-> ()
///
/// let a: i8 = 10;
/// let b: i8 = 12;
/// let delta: i8 = 1;
/// // Panic with error message
/// let result = panic::catch_unwind(|| {
/// assert_in_delta!(a, b, delta);
/// //-> panic!
/// });
/// assert!(result.is_err());
/// let actual = result.unwrap_err().downcast::<String>().unwrap().to_string();
/// let expect = concat!(
///     "assertion failed: `assert_in_delta!(left, right, delta)`\n",
///     "  left label: `a`,\n",
///     "  left debug: `10`,\n",
///     " right label: `b`,\n",
///     " right debug: `12`,\n",
///     " delta label: `delta`,\n",
///     " delta debug: `1`,\n",
///     "        left: `10`,\n",
///     "       right: `12`"
/// );
/// assert_eq!(actual, expect);
///
/// // Panic with error message
/// let result = panic::catch_unwind(|| {
/// assert_in_delta!(a, b, delta, "message");
/// //-> panic!
/// });
/// assert!(result.is_err());
/// let actual = result.unwrap_err().downcast::<String>().unwrap().to_string();
/// let expect = "message";
/// assert_eq!(actual, expect);
/// # }
/// ```
///
/// The macros `assert_in_delta` and `assert_in_epsilon` can test
/// approximations:
/// 
/// * For an approximation, the absolute error (i.e. delta) is the magnitude of
///   the difference between the exact value and the approximation. For this,
///  use the macro
///
/// * For an approximation, the relative error (i.e. epsilon) is the absolute
///   error divided by the magnitude of the exact value. This can be used to
///   compare approximations of numbers of wildly differing size.
/// 
/// * For example, approximating the number 1,000 with an absolute error of 3
///   is, in most applications, much worse than approximating the number
///   1,000,000 with an absolute error of 3; in the first case the relative
///   error is 0.003 and in the second it is only 0.000003.
///
/// * Thanks to Ruby minitest for the example and documentation.
/// 
/// # Related
///
/// * [`assert_in_delta`]
/// * [`assert_in_delta_as_result`]
/// * [`debug_assert_in_delta`]
///
#[macro_export]
macro_rules! assert_in_delta {
    ($a:expr, $b:expr, $delta:expr $(,)?) => ({
        match assert_in_delta_as_result!($a, $b, $delta) {
            Ok(()) => (),
            Err(err) => panic!("{}", err),
        }
    });
    ($a:expr, $b:expr, $delta:expr, $($message:tt)+) => ({
        match assert_in_delta_as_result!($a, $b, $delta) {
            Ok(()) => (),
            Err(_err) => panic!("{}", $($message)+),
        }
    });
}

/// Assert a number is within delta of another number.
///
/// This macro provides the same statements as [`assert_in_delta`],
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
/// * [`assert_in_delta`]
/// * [`assert_in_delta`]
/// * [`debug_assert_in_delta`]
///
#[macro_export]
macro_rules! debug_assert_in_delta {
    ($($arg:tt)*) => {
        if $crate::cfg!(debug_assertions) {
            $crate::assert_in_delta!($($arg)*);
        }
    };
}
