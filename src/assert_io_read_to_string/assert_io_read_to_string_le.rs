//! Assert a ::std::io::Read read_to_string() value is less than or equal to another.
//!
//! Pseudocode:<br>
//! (a_reader.read_to_string(a_string) ⇒ a_string) ≤ (b_reader.read_to_string(b_string) ⇒ b_string)
//!
//! # Example
//!
//! ```rust
//! use assertables::*;
//! use std::io::Read;
//!
//! let mut a = "alfa".as_bytes();
//! let mut b = "zz".as_bytes();
//! assert_io_read_to_string_le!(a, b);
//! ```
//!
//! # Module macros
//!
//! * [`assert_io_read_to_string_le`](macro@crate::assert_io_read_to_string_le)
//! * [`assert_io_read_to_string_le_as_result`](macro@crate::assert_io_read_to_string_le_as_result)
//! * [`debug_assert_io_read_to_string_le`](macro@crate::debug_assert_io_read_to_string_le)

/// Assert a ::std::io::Read read_to_string() value is less than or equal to another.
///
/// Pseudocode:<br>
/// (a_reader.read_to_string(a_string) ⇒ a_string) ≤ (b_reader.read_to_string(b_string) ⇒ b_string)
///
/// * If true, return Result `Ok((a_string, b_string))`.
///
/// * Otherwise, return Result `Err(message)`.
///
/// This macro is useful for runtime checks, such as checking parameters,
/// or sanitizing inputs, or handling different results in different ways.
///
/// # Module macros
///
/// * [`assert_io_read_to_string_le`](macro@crate::assert_io_read_to_string_le)
/// * [`assert_io_read_to_string_le_as_result`](macro@crate::assert_io_read_to_string_le_as_result)
/// * [`debug_assert_io_read_to_string_le`](macro@crate::debug_assert_io_read_to_string_le)
///
#[macro_export]
macro_rules! assert_io_read_to_string_le_as_result {
    ($a_reader:expr, $b_reader:expr $(,)?) => {{
        let mut a_string = String::new();
        let mut b_string = String::new();
        match(
            $a_reader.read_to_string(&mut a_string),
            $b_reader.read_to_string(&mut b_string)
        ) {
            (Ok(_a_size), Ok(_b_size)) => {
                if a_string <= b_string {
                    Ok((a_string, b_string))
                } else {
                    Err(
                        format!(
                            concat!(
                                "assertion failed: `assert_io_read_to_string_le!(a_reader, b_reader)`\n",
                                "https://docs.rs/assertables/9.6.1/assertables/macro.assert_io_read_to_string_le.html\n",
                                " a label: `{}`,\n",
                                " a debug: `{:?}`,\n",
                                " b label: `{}`,\n",
                                " b debug: `{:?}`,\n",
                                "       a: `{:?}`,\n",
                                "       b: `{:?}`"
                            ),
                            stringify!($a_reader),
                            $a_reader,
                            stringify!($b_reader),
                            $b_reader,
                            a_string,
                            b_string
                        )
                    )
                }
            },
            _ => {
                Err(
                    format!(
                        concat!(
                            "assertion failed: `assert_io_read_to_string_le!(a_reader, b_reader)`\n",
                            "https://docs.rs/assertables/9.6.1/assertables/macro.assert_io_read_to_string_le.html\n",
                            "  a label: `{}`,\n",
                            "  a debug: `{:?}`,\n",
                            "  b label: `{}`,\n",
                            "  b debug: `{:?}`",
                        ),
                        stringify!($a_reader),
                        $a_reader,
                        stringify!($b_reader),
                        $b_reader,
                    )
                )
            }
        }
    }};
}

#[cfg(test)]
mod test_assert_io_read_to_string_le_as_result {
    #[allow(unused_imports)]
    use std::io::Read;
    use std::sync::Once;

    #[test]
    fn lt() {
        let mut a = "alfa".as_bytes();
        let mut b = "zz".as_bytes();
        let actual = assert_io_read_to_string_le_as_result!(a, b);
        assert_eq!(actual.unwrap(), (String::from("alfa"), String::from("zz")));
    }

    #[test]
    fn lt_once() {
        static A: Once = Once::new();
        fn a() -> &'static [u8] {
            if A.is_completed() {
                panic!("A.is_completed()")
            } else {
                A.call_once(|| {})
            }
            "alfa".as_bytes()
        }

        static B: Once = Once::new();
        fn b() -> &'static [u8] {
            if B.is_completed() {
                panic!("B.is_completed()")
            } else {
                B.call_once(|| {})
            }
            "zz".as_bytes()
        }

        assert_eq!(A.is_completed(), false);
        assert_eq!(B.is_completed(), false);
        let result = assert_io_read_to_string_le_as_result!(a(), b());
        assert!(result.is_ok());
        assert_eq!(A.is_completed(), true);
        assert_eq!(B.is_completed(), true);
    }

    #[test]
    fn eq() {
        let mut a = "alfa".as_bytes();
        let mut b = "alfa".as_bytes();
        let actual = assert_io_read_to_string_le_as_result!(a, b);
        assert_eq!(
            actual.unwrap(),
            (String::from("alfa"), String::from("alfa"))
        );
    }

    #[test]
    fn eq_once() {
        static A: Once = Once::new();
        fn a() -> &'static [u8] {
            if A.is_completed() {
                panic!("A.is_completed()")
            } else {
                A.call_once(|| {})
            }
            "alfa".as_bytes()
        }

        static B: Once = Once::new();
        fn b() -> &'static [u8] {
            if B.is_completed() {
                panic!("B.is_completed()")
            } else {
                B.call_once(|| {})
            }
            "alfa".as_bytes()
        }

        assert_eq!(A.is_completed(), false);
        assert_eq!(B.is_completed(), false);
        let result = assert_io_read_to_string_le_as_result!(a(), b());
        assert!(result.is_ok());
        assert_eq!(A.is_completed(), true);
        assert_eq!(B.is_completed(), true);
    }

    #[test]
    fn gt() {
        let mut a = "alfa".as_bytes();
        let mut b = "aa".as_bytes();
        let actual = assert_io_read_to_string_le_as_result!(a, b);
        let message = concat!(
            "assertion failed: `assert_io_read_to_string_le!(a_reader, b_reader)`\n",
            "https://docs.rs/assertables/9.6.1/assertables/macro.assert_io_read_to_string_le.html\n",
            " a label: `a`,\n",
            " a debug: `[]`,\n",
            " b label: `b`,\n",
            " b debug: `[]`,\n",
            "       a: `\"alfa\"`,\n",
            "       b: `\"aa\"`"
        );
        assert_eq!(actual.unwrap_err(), message);
    }
}

/// Assert a ::std::io::Read read_to_string() value is less than or equal to another.
///
/// Pseudocode:<br>
/// (a_reader.read_to_string(a_string) ⇒ a_string) ≤ (b_reader.read_to_string(b_string) ⇒ b_string)
///
/// * If true, return `(a_string, b_string)`.
///
/// * Otherwise, call [`panic!`] with a message and the values of the
///   expressions with their debug representations.
///
/// # Examples
///
/// ```rust
/// use assertables::*;
/// # use std::panic;
/// use std::io::Read;
///
/// # fn main() {
/// let mut a = "alfa".as_bytes();
/// let mut b = "zz".as_bytes();
/// assert_io_read_to_string_le!(a, b);
///
/// # let result = panic::catch_unwind(|| {
/// // This will panic
/// let mut a = "alfa".as_bytes();
/// let mut b = "aa".as_bytes();
/// assert_io_read_to_string_le!(a, b);
/// # });
/// // assertion failed: `assert_io_read_to_string_le!(a_reader, b_reader)`
/// // https://docs.rs/assertables/9.6.1/assertables/macro.assert_io_read_to_string_le.html
/// //  a label: `a`,
/// //  a debug: `[]`,
/// //  b label: `b`,
/// //  b debug: `[]`,
/// //        a: `\"alfa\"`,
/// //        b: `\"aa\"`
/// # let actual = result.unwrap_err().downcast::<String>().unwrap().to_string();
/// # let message = concat!(
/// #     "assertion failed: `assert_io_read_to_string_le!(a_reader, b_reader)`\n",
/// #     "https://docs.rs/assertables/9.6.1/assertables/macro.assert_io_read_to_string_le.html\n",
/// #     " a label: `a`,\n",
/// #     " a debug: `[]`,\n",
/// #     " b label: `b`,\n",
/// #     " b debug: `[]`,\n",
/// #     "       a: `\"alfa\"`,\n",
/// #     "       b: `\"aa\"`"
/// # );
/// # assert_eq!(actual, message);
/// # }
/// ```
///
/// # Module macros
///
/// * [`assert_io_read_to_string_le`](macro@crate::assert_io_read_to_string_le)
/// * [`assert_io_read_to_string_le_as_result`](macro@crate::assert_io_read_to_string_le_as_result)
/// * [`debug_assert_io_read_to_string_le`](macro@crate::debug_assert_io_read_to_string_le)
///
#[macro_export]
macro_rules! assert_io_read_to_string_le {
    ($a_reader:expr, $b:expr $(,)?) => {
        match $crate::assert_io_read_to_string_le_as_result!($a_reader, $b) {
            Ok(x) => x,
            Err(err) => panic!("{}", err),
        }
    };
    ($a_reader:expr, $b:expr, $($message:tt)+) => {
        match $crate::assert_io_read_to_string_le_as_result!($a_reader, $b) {
            Ok(x) => x,
            Err(err) => panic!("{}\n{}", format_args!($($message)+), err),
        }
    };
}

#[cfg(test)]
mod test_assert_io_read_to_string_le {
    #[allow(unused_imports)]
    use std::io::Read;
    use std::panic;

    #[test]
    fn lt() {
        let mut a = "alfa".as_bytes();
        let mut b = "zz".as_bytes();
        for _ in 0..1 {
            let actual = assert_io_read_to_string_le!(a, b);
            assert_eq!(actual, (String::from("alfa"), String::from("zz")));
        }
    }

    #[test]
    fn eq() {
        let mut a = "alfa".as_bytes();
        let mut b = "alfa".as_bytes();
        for _ in 0..1 {
            let actual = assert_io_read_to_string_le!(a, b);
            assert_eq!(actual, (String::from("alfa"), String::from("alfa")));
        }
    }

    #[test]
    fn gt() {
        let result = panic::catch_unwind(|| {
            let mut a = "alfa".as_bytes();
            let mut b = "aa".as_bytes();
            let _actual = assert_io_read_to_string_le!(a, b);
        });
        let message = concat!(
            "assertion failed: `assert_io_read_to_string_le!(a_reader, b_reader)`\n",
            "https://docs.rs/assertables/9.6.1/assertables/macro.assert_io_read_to_string_le.html\n",
            " a label: `a`,\n",
            " a debug: `[]`,\n",
            " b label: `b`,\n",
            " b debug: `[]`,\n",
            "       a: `\"alfa\"`,\n",
            "       b: `\"aa\"`"
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

/// Assert a ::std::io::Read read_to_string() value is less than or equal to another.
///
/// Pseudocode:<br>
/// (a_reader.read_to_string(a_string) ⇒ a_string) ≤ (b_reader.read_to_string(b_string) ⇒ b_string)
///
/// This macro provides the same statements as [`assert_io_read_to_string_le`](macro.assert_io_read_to_string_le.html),
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
/// * [`assert_io_read_to_string_le`](macro@crate::assert_io_read_to_string_le)
/// * [`assert_io_read_to_string_le`](macro@crate::assert_io_read_to_string_le)
/// * [`debug_assert_io_read_to_string_le`](macro@crate::debug_assert_io_read_to_string_le)
///
#[macro_export]
macro_rules! debug_assert_io_read_to_string_le {
    ($($arg:tt)*) => {
        if $crate::cfg!(debug_assertions) {
            $crate::assert_io_read_to_string_le!($($arg)*);
        }
    };
}
