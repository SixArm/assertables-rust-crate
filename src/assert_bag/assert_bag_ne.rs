//! Assert a bag is not equal to another.
//!
//! Pseudocode:<br>
//! (a_collection ⇒ a_bag) ≠ (b_collection ⇒ b_bag)
//!
//! # Example
//!
//! ```rust
//! use assertables::*;
//!
//! let a = [1, 1];
//! let b = [1, 1, 1];
//! assert_bag_ne!(a, b);
//! ```
//!
//! This implementation uses [`::std::collections::BTreeMap`](https://doc.rust-lang.org/std/collections/struct.BTreeMap.html) to count items and sort them.
//!
//! # Module macros
//!
//! * [`assert_bag_ne`](macro@crate::assert_bag_ne)
//! * [`assert_bag_ne_as_result`](macro@crate::assert_bag_ne_as_result)
//! * [`debug_assert_bag_ne`](macro@crate::debug_assert_bag_ne)

/// Assert a bag is not equal to another.
///
/// Pseudocode:<br>
/// (a_collection ⇒ a_bag) ≠ (b_collection ⇒ b_bag)
///
/// * If true, return Result `Ok((a, b))`.
///
/// * Otherwise, return Result `Err(message)`.
///
/// This macro is useful for runtime checks, such as checking parameters,
/// or sanitizing inputs, or handling different results in different ways.
///
/// # Module macros
///
/// * [`assert_bag_ne`](macro@crate::assert_bag_ne)
/// * [`assert_bag_ne_as_result`](macro@crate::assert_bag_ne_as_result)
/// * [`debug_assert_bag_ne`](macro@crate::debug_assert_bag_ne)
///
#[macro_export]
macro_rules! assert_bag_ne_as_result {
    ($a_collection:expr, $b_collection:expr $(,)?) => {
        match (&$a_collection, &$b_collection) {
            (a_collection, b_collection) => {
                let a = assert_bag_impl_prep!(a_collection);
                let b = assert_bag_impl_prep!(b_collection);
                if a != b {
                    Ok((a, b))
                } else {
                    Err(format!(
                        concat!(
                            "assertion failed: `assert_bag_ne!(a_collection, b_collection)`\n",
                            "https://docs.rs/assertables/9.8.1/assertables/macro.assert_bag_ne.html\n",
                            " a label: `{}`,\n",
                            " a debug: `{:?}`,\n",
                            " b label: `{}`,\n",
                            " b debug: `{:?}`,\n",
                            "   a bag: `{:?}`,\n",
                            "   b bag: `{:?}`"
                        ),
                        stringify!($a_collection),
                        a_collection,
                        stringify!($b_collection),
                        b_collection,
                        a,
                        b
                    ))
                }
            }
        }
    };
}

#[cfg(test)]
mod test_assert_bag_ne_as_result {
    use std::collections::BTreeMap;
    // use std::sync::Once;

    #[test]
    fn lt() {
        let a = [1, 1];
        let b = [1, 1, 1];
        for _ in 0..1 {
            let actual = assert_bag_ne_as_result!(a, b);
            assert_eq!(
                actual.unwrap(),
                (BTreeMap::from([(&1, 2)]), BTreeMap::from([(&1, 3)]))
            );
        }
    }

    //TODO
    // #[test]
    // fn lt_once() {
    //     static A: Once = Once::new();
    //     fn a() -> [i32; 2] {
    //         if A.is_completed() {
    //             panic!("A.is_completed()")
    //         } else {
    //             A.call_once(|| {})
    //         }
    //         [1, 1]
    //     }

    //     static B: Once = Once::new();
    //     fn b() -> [i32; 3] {
    //         if B.is_completed() {
    //             panic!("B.is_completed()")
    //         } else {
    //             B.call_once(|| {})
    //         }
    //         [1, 1, 1]
    //     }

    //     assert_eq!(A.is_completed(), false);
    //     assert_eq!(B.is_completed(), false);
    //     let result = assert_bag_ne_as_result!(a(), b());
    //     assert!(result.is_ok());
    //     assert_eq!(A.is_completed(), true);
    //     assert_eq!(B.is_completed(), true);
    // }

    #[test]
    fn gt() {
        let a = [1, 1, 1];
        let b = [1, 1];
        for _ in 0..1 {
            let actual = assert_bag_ne_as_result!(a, b);
            assert_eq!(
                actual.unwrap(),
                (BTreeMap::from([(&1, 3)]), BTreeMap::from([(&1, 2)]))
            );
        }
    }

    //TODO
    // #[test]
    // fn gt_once() {
    //     static A: Once = Once::new();
    //     fn a() -> [i32; 3] {
    //         if A.is_completed() {
    //             panic!("A.is_completed()")
    //         } else {
    //             A.call_once(|| {})
    //         }
    //         [1, 1, 1]
    //     }

    //     static B: Once = Once::new();
    //     fn b() -> [i32; 2] {
    //         if B.is_completed() {
    //             panic!("B.is_completed()")
    //         } else {
    //             B.call_once(|| {})
    //         }
    //         [1, 1]
    //     }

    //     assert_eq!(A.is_completed(), false);
    //     assert_eq!(B.is_completed(), false);
    //     let result = assert_bag_ne_as_result!(a(), b());
    //     assert!(result.is_ok());
    //     assert_eq!(A.is_completed(), true);
    //     assert_eq!(B.is_completed(), true);
    // }

    #[test]
    fn eq() {
        let a = [1, 1];
        let b = [1, 1];
        let actual = assert_bag_ne_as_result!(a, b);
        let message = concat!(
            "assertion failed: `assert_bag_ne!(a_collection, b_collection)`\n",
            "https://docs.rs/assertables/9.8.1/assertables/macro.assert_bag_ne.html\n",
            " a label: `a`,\n",
            " a debug: `[1, 1]`,\n",
            " b label: `b`,\n",
            " b debug: `[1, 1]`,\n",
            "   a bag: `{1: 2}`,\n",
            "   b bag: `{1: 2}`"
        );
        assert_eq!(actual.unwrap_err(), message);
    }
}

/// Assert a bag is not equal to another.
///
/// Pseudocode:<br>
/// (a_collection ⇒ a_bag) ≠ (b_collection ⇒ b_bag)
///
/// * If true, return `(a, b)`.
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
/// let a = [1, 1];
/// let b = [1, 1, 1];
/// assert_bag_ne!(a, b);
///
/// # let result = panic::catch_unwind(|| {
/// // This will panic
/// let a = [1, 1];
/// let b = [1, 1];
/// assert_bag_ne!(a, b);
/// # });
/// // assertion failed: `assert_bag_ne!(a_collection, b_collection)`
/// // https://docs.rs/assertables/…/assertables/macro.assert_bag_ne.html
/// //  a label: `a`,
/// //  a debug: `[1, 1]`,
/// //  b label: `b`,
/// //  b debug: `[1, 1]`,
/// //    a bag: `{1: 2}`,
/// //    b bag: `{1: 2}`
/// # let actual = result.unwrap_err().downcast::<String>().unwrap().to_string();
/// # let message = concat!(
/// #     "assertion failed: `assert_bag_ne!(a_collection, b_collection)`\n",
/// #     "https://docs.rs/assertables/9.8.1/assertables/macro.assert_bag_ne.html\n",
/// #     " a label: `a`,\n",
/// #     " a debug: `[1, 1]`,\n",
/// #     " b label: `b`,\n",
/// #     " b debug: `[1, 1]`,\n",
/// #     "   a bag: `{1: 2}`,\n",
/// #     "   b bag: `{1: 2}`"
/// # );
/// # assert_eq!(actual, message);
/// # }
/// ```
///
/// This implementation uses [`::std::collections::BTreeMap`](https://doc.rust-lang.org/std/collections/struct.BTreeMap.html) to count items and sort them.
///
/// # Module macros
///
/// * [`assert_bag_ne`](macro@crate::assert_bag_ne)
/// * [`assert_bag_ne_as_result`](macro@crate::assert_bag_ne_as_result)
/// * [`debug_assert_bag_ne`](macro@crate::debug_assert_bag_ne)
///
#[macro_export]
macro_rules! assert_bag_ne {
    ($a_collection:expr, $b_collection:expr $(,)?) => {
        match $crate::assert_bag_ne_as_result!($a_collection, $b_collection) {
            Ok(x) => x,
            Err(err) => panic!("{}", err),
        }
    };
    ($a_collection:expr, $b_collection:expr, $($message:tt)+) => {
        match $crate::assert_bag_ne_as_result!($a_collection, $b_collection) {
            Ok(x) => x,
            Err(err) => panic!("{}\n{}", format_args!($($message)+), err),
        }
    };
}

#[cfg(test)]
mod test_assert_bag_ne {
    use std::collections::BTreeMap;
    use std::panic;

    #[test]
    fn success() {
        let a = [1, 1];
        let b = [1, 1, 1];
        for _ in 0..1 {
            let actual = assert_bag_ne!(a, b);
            assert_eq!(
                actual,
                (BTreeMap::from([(&1, 2)]), BTreeMap::from([(&1, 3)]))
            );
        }
    }

    #[test]
    fn failure() {
        let result = panic::catch_unwind(|| {
            let a = [1, 1];
            let b = [1, 1];
            let _actual = assert_bag_ne!(a, b);
        });
        let message = concat!(
            "assertion failed: `assert_bag_ne!(a_collection, b_collection)`\n",
            "https://docs.rs/assertables/9.8.1/assertables/macro.assert_bag_ne.html\n",
            " a label: `a`,\n",
            " a debug: `[1, 1]`,\n",
            " b label: `b`,\n",
            " b debug: `[1, 1]`,\n",
            "   a bag: `{1: 2}`,\n",
            "   b bag: `{1: 2}`"
        );
        assert_eq!(
            result
                .unwrap_err()
                .downcast::<String>()
                .unwrap()
                .to_string(),
            message
        );
    }
}

/// Assert a bag is not equal to another.
///
/// Pseudocode:<br>
/// (a_collection ⇒ a_bag) ≠ (b_collection ⇒ b_bag)
///
/// This macro provides the same statements as [`assert_bag_ne`](macro.assert_bag_ne.html),
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
/// * [`assert_bag_ne`](macro@crate::assert_bag_ne)
/// * [`assert_bag_ne_as_result`](macro@crate::assert_bag_ne_as_result)
/// * [`debug_assert_bag_ne`](macro@crate::debug_assert_bag_ne)
///
#[macro_export]
macro_rules! debug_assert_bag_ne {
    ($($arg:tt)*) => {
        if $crate::cfg!(debug_assertions) {
            $crate::assert_bag_ne!($($arg)*);
        }
    };
}
