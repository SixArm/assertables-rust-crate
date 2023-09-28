/// Assert an expression (such as a regex) is not a match for an expression (such as a string).
///
/// * If true, return Result `Ok(())`.
///
/// * Otherwise, return Result `Err` with a diagnostic message.
///
/// This macro provides the same statements as [`assert_not_match`],
/// except this macro returns a Result, rather than doing a panic.
///
/// This macro is useful for runtime checks, such as checking parameters,
/// or sanitizing inputs, or handling different results in different ways.
///
/// # Related
///
/// * [`assert_not_match`]
/// * [`assert_not_match_as_result`]
/// * [`debug_assert_not_match`]
///
#[macro_export]
macro_rules! assert_not_match_as_result {
    ($matcher:expr, $matchee:expr $(,)?) => ({
        if !($matcher.is_match($matchee)) {
            Ok(())
        } else {
            Err(format!(
                concat!(
                    "assertion failed: `assert_not_match!(matcher, matchee)`\n",
                    " matcher label: `{}`,\n",
                    " matcher debug: `{:?}`,\n",
                    " matchee label: `{}`,\n",
                    " matchee debug: `{:?}`",
                ),
                stringify!($matcher), $matcher,
                stringify!($matchee), $matchee,
            ))
        }
    });
}

#[cfg(test)]
mod tests {

    use regex::Regex;

    #[test]
    fn test_assert_not_match_as_result_x_success() {
        let a = Regex::new(r"foo").unwrap();
        let b = "yoohoo";
        let x = assert_not_match_as_result!(a, b);
        assert_eq!(x.unwrap(), ());
    }

    #[test]
    fn test_assert_not_match_as_result_x_failure() {
        let a = Regex::new(r"foo").unwrap();
        let b = "foogoo";
        let x = assert_not_match_as_result!(a, b);
        let actual = x.unwrap_err();
        let expect = concat!(
            "assertion failed: `assert_not_match!(matcher, matchee)`\n",
            " matcher label: `a`,\n",
            " matcher debug: `foo`,\n",
            " matchee label: `b`,\n",
            " matchee debug: `\"foogoo\"`"
        );
        assert_eq!(actual, expect);
    }
}

/// Assert a matcher is a match for an expression.
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
/// use std::process::Command;
/// use regex::Regex;
///
/// # fn main() {
/// // Return Ok
/// let a = Regex::new(r"foo").unwrap();
/// let b = "yoohoo";
/// assert_not_match!(a, b);
/// //-> ()
///
/// // Panic with error message
/// let result = panic::catch_unwind(|| {
/// let a = Regex::new(r"foo").unwrap();
/// let b = "foogoo";
/// assert_not_match!(a, b);
/// //-> panic!
/// });
/// assert!(result.is_err());
/// let actual = result.unwrap_err().downcast::<String>().unwrap().to_string();
/// let expect = concat!(
///     "assertion failed: `assert_not_match!(matcher, matchee)`\n",
///     " matcher label: `a`,\n",
///     " matcher debug: `foo`,\n",
///     " matchee label: `b`,\n",
///     " matchee debug: `\"foogoo\"`"
/// );
/// assert_eq!(actual, expect);
/// # }
/// ```
///
/// # Related
///
/// * [`assert_not_match`]
/// * [`assert_not_match_as_result`]
/// * [`debug_assert_not_match`]
///
#[macro_export]
macro_rules! assert_not_match {
    ($matcher:expr, $matchee:expr $(,)?) => ({
        match assert_not_match_as_result!($matcher, $matchee) {
            Ok(()) => (),
            Err(err) => panic!("{}", err),
        }
    });
    ($matcher:expr, $matchee:expr, $($message:tt)+) => ({
        match assert_not_match_as_result!($matcher, $matchee) {
            Ok(()) => (),
            Err(_err) => panic!("{}", $($message)+),
        }
    });
}

/// Assert a matcher is a match for an expression.
///
/// This macro provides the same statements as [`assert_not_match`],
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
/// * [`assert_not_match`]
/// * [`assert_not_match`]
/// * [`debug_assert_not_match`]
///
#[macro_export]
macro_rules! debug_assert_not_match {
    ($($arg:tt)*) => {
        if $crate::cfg!(debug_assertions) {
            $crate::assert_not_match!($($arg)*);
        }
    };
}
