/// Assert a std::fs::read_to_string() is a match to a regex.
///
/// * If true, return Result `Ok(())`.
///
/// * Otherwise, return Result `Err` with a diagnostic message.
///
/// This macro provides the same statements as [`assert_`],
/// except this macro returns a Result, rather than doing a panic.
///
/// This macro is useful for runtime checks, such as checking parameters,
/// or sanitizing inputs, or handling different results in different ways.
///
/// # Module macros
///
/// * [`read_to_string_matches`]
/// * [`read_to_string_matches_as_result`]
/// * [`debug_read_to_string_matches`]
///
#[macro_export]
macro_rules! assert_fs_read_to_string_matches_as_result {
    ($a_path:expr, $b_matcher:expr $(,)?) => ({
        let a_result = ::std::fs::read_to_string($a_path);
        if let Err(a_err) = a_result {
            Err(format!(
                concat!(
                    "assertion failed: `assert_fs_read_to_string_matches!(left_path, right_matcher)`\n",
                    "     left_path label: `{}`,\n",
                    "     left_path debug: `{:?}`,\n",
                    " right_matcher label: `{}`,\n",
                    " right_matcher debug: `{:?}`,\n",
                    "            left err: `{:?}`"
                ),
                stringify!($a_path), $a_path,
                stringify!($b_matcher), $b_matcher,
                a_err
            ))
        } else {
            let a_string = a_result.unwrap();
            if $b_matcher.is_match(a_string.as_str()) {
                Ok(())
            } else {
                Err(format!(
                    concat!(
                        "assertion failed: `assert_fs_read_to_string_matches!(left_path, right_matcher)`\n",
                        "     left_path label: `{}`,\n",
                        "     left_path debug: `{:?}`,\n",
                        " right_matcher label: `{}`,\n",
                        " right_matcher debug: `{:?}`,\n",
                        "                left: `{:?}`,\n",
                        "               right: `{:?}`",
                    ),
                    stringify!($a_path), $a_path,
                    stringify!($b_matcher), $b_matcher,
                    a_string,
                    $b_matcher
                ))
            }
        }
    });
}

#[cfg(test)]
mod tests {
    use once_cell::sync::Lazy;
    use regex::Regex;
    use std::path::PathBuf;

    pub static DIR: Lazy<PathBuf> = Lazy::new(|| {
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("tests")
            .join("src")
            .join("std")
            .join("fs")
    });

    #[test]
    fn test_read_to_string_matches_as_result_x_success() {
        let path = DIR.join("alfa.txt");
        let matcher = Regex::new(r"alfa").unwrap();
        let x = assert_fs_read_to_string_matches_as_result!(&path, matcher);
        assert_eq!(x, Ok(()));
    }

    #[test]
    fn test_read_to_string_matches_as_result_x_failure() {
        let path = DIR.join("alfa.txt");
        let matcher = Regex::new(r"zzz").unwrap();
        let x = assert_fs_read_to_string_matches_as_result!(&path, matcher);
        assert!(x.is_err());
        assert_eq!(
            x.unwrap_err(),
            format!(
                "{}{}{}{}{}{}{}{}{}",
                "assertion failed: `assert_fs_read_to_string_matches!(left_path, right_matcher)`\n",
                "     left_path label: `&path`,\n",
                "     left_path debug: `\"",
                path.to_string_lossy(),
                "\"`,\n",
                " right_matcher label: `matcher`,\n",
                " right_matcher debug: `Regex(\"zzz\")`,\n",
                "                left: `\"alfa\\n\"`,\n",
                "               right: `Regex(\"zzz\")`"
            )
        );
    }
}

/// Assert a std::fs::read_to_string() is a match to a regex.
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
/// use std::io::Read;
/// use regex::Regex;
///
/// # fn main() {
/// // Return Ok
/// let path = "alfa.txt";
/// let matcher = Regex::new(r"alfa").unwrap();
/// assert_fs_read_to_string_matches!(&path, matcher);
/// //-> ()
///
/// // Panic with error message
/// let result = panic::catch_unwind(|| {
/// let path = "alfa.txt";
/// let matcher = Regex::new(r"zzz").unwrap();
/// assert_fs_read_to_string_matches!(&path, matcher);
/// //-> panic!
/// });
/// assert!(result.is_err());
/// let actual = result.unwrap_err().downcast::<String>().unwrap().to_string();
/// let expect = concat!(
///     "assertion failed: `assert_fs_read_to_string_matches!(left_path, right_matcher)`\n",
///     "     left_path label: `&path`,\n",
///     "     left_path debug: `\"alfa.txt\"`,\n",
///     " right_matcher label: `matcher`,\n",
///     " right_matcher debug: `Regex(\"zzz\")`,\n",
///     "                left: `\"alfa\\n\"`,\n",
///     "               right: `Regex(\"zzz\")`"
/// );
/// assert_eq!(actual, expect);
/// # }
/// ```
///
/// # Module macros
///
/// * [`read_to_string_matches`]
/// * [`read_to_string_matches_as_result`]
/// * [`debug_read_to_string_matches`]
///
#[macro_export]
macro_rules! assert_fs_read_to_string_matches {
    ($a_path:expr, $b_matcher:expr $(,)?) => ({
        match assert_fs_read_to_string_matches_as_result!($a_path, $b_matcher) {
            Ok(()) => (),
            Err(err) => panic!("{}", err),
        }
    });
    ($a_path:expr, $b_matcher:expr, $($message:tt)+) => ({
        match assert_fs_read_to_string_matches_as_result!($a_path, $b_matcher) {
            Ok(()) => (),
            Err(_err) => panic!("{}", $($message)+),
        }
    });
}

/// Assert a std::fs::read_to_string() is a match to a regex.
///
/// This macro provides the same statements as [`read_to_string_matches`],
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
/// # Module macros
///
/// * [`read_to_string_matches`]
/// * [`read_to_string_matches`]
/// * [`debug_read_to_string_matches`]
///
#[macro_export]
macro_rules! debug_read_to_string_matches {
    ($($arg:tt)*) => {
        if $crate::cfg!(debug_assertions) {
            $crate::std::fs::read_to_string_matches!($($arg)*);
        }
    };
}
