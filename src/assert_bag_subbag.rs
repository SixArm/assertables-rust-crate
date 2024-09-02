/// Assert a bag is a subbag of another.
///
/// * If true, return Result `Ok(())`.
///
/// * Otherwise, return Result `Err` with a diagnostic message.
///
/// This macro provides the same statements as [`assert_bag_subbag`],
/// except this macro returns a Result, rather than doing a panic.
///
/// This macro is useful for runtime checks, such as checking parameters,
/// or sanitizing inputs, or handling different results in different ways.
///
/// # Related
///
/// * [`assert_bag_subbag`]
/// * [`assert_bag_subbag_as_result`]
/// * [`debug_assert_bag_subbag`]
///
#[macro_export]
macro_rules! assert_bag_subbag_as_result {
    ($a:expr, $b:expr $(,)?) => {{
        match (&$a, &$b) {
            (a_val, b_val) => {
                let mut a_bag: ::std::collections::BTreeMap<_, usize> =
                    ::std::collections::BTreeMap::new();
                let mut b_bag: ::std::collections::BTreeMap<_, usize> =
                    ::std::collections::BTreeMap::new();
                for x in a_val.into_iter() {
                    let n = a_bag.entry(x).or_insert(0);
                    *n += 1;
                }
                for x in b_val.into_iter() {
                    let n = b_bag.entry(x).or_insert(0);
                    *n += 1;
                }
                if a_val.into_iter().all(|key| {
                    a_bag.contains_key(&key)
                        && b_bag.contains_key(&key)
                        && a_bag.get_key_value(&key) <= b_bag.get_key_value(&key)
                }) {
                    Ok(())
                } else {
                    Err(format!(
                        concat!(
                            "assertion failed: `assert_bag_subbag!(left_bag, right_bag)`\n",
                            "  left_bag label: `{}`,\n",
                            "  left_bag debug: `{:?}`,\n",
                            " right_bag label: `{}`,\n",
                            " right_bag debug: `{:?}`,\n",
                            "            left: `{:?}`,\n",
                            "           right: `{:?}`"
                        ),
                        stringify!($a),
                        $a,
                        stringify!($b),
                        $b,
                        &a_bag,
                        &b_bag
                    ))
                }
            }
        }
    }};
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_assert_bag_subbag_as_result_x_success() {
        let a = [1, 1];
        let b = [1, 1, 1];
        let x = assert_bag_subbag_as_result!(&a, &b);
        assert_eq!(x, Ok(()));
    }

    #[test]
    fn test_assert_bag_subbag_as_result_x_failure_because_key_is_missing() {
        let a = [1, 1];
        let b = [2, 2];
        let x = assert_bag_subbag_as_result!(&a, &b);
        assert!(x.is_err());
        assert_eq!(
            x.unwrap_err(),
            concat!(
                "assertion failed: `assert_bag_subbag!(left_bag, right_bag)`\n",
                "  left_bag label: `&a`,\n",
                "  left_bag debug: `[1, 1]`,\n",
                " right_bag label: `&b`,\n",
                " right_bag debug: `[2, 2]`,\n",
                "            left: `{1: 2}`,\n",
                "           right: `{2: 2}`"
            )
        );
    }

    #[test]
    fn test_assert_bag_subbag_as_result_x_failure_because_val_count_is_excessive() {
        let a = [1, 1, 1];
        let b = [1, 1];
        let x = assert_bag_subbag_as_result!(&a, &b);
        assert!(x.is_err());
        assert_eq!(
            x.unwrap_err(),
            concat!(
                "assertion failed: `assert_bag_subbag!(left_bag, right_bag)`\n",
                "  left_bag label: `&a`,\n",
                "  left_bag debug: `[1, 1, 1]`,\n",
                " right_bag label: `&b`,\n",
                " right_bag debug: `[1, 1]`,\n",
                "            left: `{1: 3}`,\n",
                "           right: `{1: 2}`"
            )
        );
    }
}

/// Assert a bag is a subbag of another.
///
/// * If true, return `()`.
///
/// * Otherwise, call [`panic!`] in order to print the values of the
///   expressions with their debug representations.
///
/// # Examples
///
/// ```rust
/// # #[macro_use] extern crate assertables;
/// # use std::panic;
/// # fn main() {
/// // Return Ok
/// let a = [1, 1];
/// let b = [1, 1, 1];
/// assert_bag_subbag!(&a, &b);
/// //-> ()
///
/// // Panic with error message
/// let result = panic::catch_unwind(|| {
/// let a = [1, 1];
/// let b = [2, 2];
/// assert_bag_subbag!(&a, &b);
/// //-> panic!
/// });
/// assert!(result.is_err());
/// let actual = result.unwrap_err().downcast::<String>().unwrap().to_string();
/// let expect = concat!(
///     "assertion failed: `assert_bag_subbag!(left_bag, right_bag)`\n",
///     "  left_bag label: `&a`,\n",
///     "  left_bag debug: `[1, 1]`,\n",
///     " right_bag label: `&b`,\n",
///     " right_bag debug: `[2, 2]`,\n",
///     "            left: `{1: 2}`,\n",
///     "           right: `{2: 2}`"
/// );
/// assert_eq!(actual, expect);
///
/// // Panic with custom message
/// let result = panic::catch_unwind(|| {
/// let a = [1, 1];
/// let b = [2, 2];
/// assert_bag_subbag!(&a, &b, "message");
/// //-> panic!
/// });
/// assert!(result.is_err());
/// let actual = result.unwrap_err().downcast::<String>().unwrap().to_string();
/// let expect = "message";
/// assert_eq!(actual, expect);
//
/// // Panic with error message
/// let a = [1, 1, 1];
/// let b = [1, 1];
/// // Panic with error message
/// let result = panic::catch_unwind(|| {
/// assert_bag_subbag!(&a, &b);
/// //-> panic!
/// });
/// assert!(result.is_err());
/// let actual = result.unwrap_err().downcast::<String>().unwrap().to_string();
/// let expect = concat!(
///     "assertion failed: `assert_bag_subbag!(left_bag, right_bag)`\n",
///     "  left_bag label: `&a`,\n",
///     "  left_bag debug: `[1, 1, 1]`,\n",
///     " right_bag label: `&b`,\n",
///     " right_bag debug: `[1, 1]`,\n",
///     "            left: `{1: 3}`,\n",
///     "           right: `{1: 2}`"
/// );
/// assert_eq!(actual, expect);
///
/// // Panic with custom message
/// let result = panic::catch_unwind(|| {
/// let a = [1, 1, 1];
/// let b = [1, 1];
/// assert_bag_subbag!(&a, &b, "message");
/// //-> panic!
/// });
/// assert!(result.is_err());
/// let actual = result.unwrap_err().downcast::<String>().unwrap().to_string();
/// let expect = "message";
/// assert_eq!(actual, expect);
/// # }
/// ```
///
/// This implementation uses [`BTreeMap`] to count items and sort them.
///
/// # Related
///
/// * [`assert_bag_subbag`]
/// * [`assert_bag_subbag_as_result`]
/// * [`debug_assert_bag_subbag`]
///
#[macro_export]
macro_rules! assert_bag_subbag {
    ($a:expr, $b:expr $(,)?) => ({
        match assert_bag_subbag_as_result!($a, $b) {
            Ok(()) => (),
            Err(err) => panic!("{}", err),
        }
    });
    ($a:expr, $b:expr, $($message:tt)+) => ({
        match assert_bag_subbag_as_result!($a, $b) {
            Ok(()) => (),
            Err(_err) => panic!("{}", $($message)+),
        }
    });
}

/// Assert a bag is a subbag of another.
///
/// This macro provides the same statements as [`assert_bag_subbag`],
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
/// * [`assert_bag_subbag`]
/// * [`assert_bag_subbag`]
/// * [`debug_assert_bag_subbag`]
///
#[macro_export]
macro_rules! debug_assert_bag_subbag {
    ($($arg:tt)*) => {
        if $crate::cfg!(debug_assertions) {
            $crate::assert_bag_subbag!($($arg)*);
        }
    };
}
