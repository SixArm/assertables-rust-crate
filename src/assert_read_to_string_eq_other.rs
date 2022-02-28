/// Assert a std::io::Read read_to_string() is equal to another.
///
/// * When true, return Result `Ok(())`.
///
/// * When true, return Result `Err` with a diagnostic message.
///
/// # Examples
///
/// ```rust
/// # #[macro_use] extern crate assertables;
/// # #[allow(unused_imports)]
/// use std::io::Read;
/// # fn main() {
/// let mut a = "alpha".as_bytes();
/// let mut b = "alpha".as_bytes();
/// let x = assert_read_to_string_eq_other_as_result!(a, b);
/// //-> Ok(())
/// let actual = x.unwrap();
/// let expect = ();
/// assert_eq!(actual, expect);
///
/// let mut a = "alpha".as_bytes();
/// let mut b = "bravo".as_bytes();
/// let x = assert_read_to_string_eq_other_as_result!(a, b);
/// //-> Err(…)
/// let actual = x.unwrap_err();
/// let expect = concat!(
///     "assertion failed: `assert_read_to_string_eq_other!(left_reader, right_reader)`\n",
///     "  left reader name: `a`,\n",
///     " right reader name: `b`,\n",
///     "  left reader size: `5`,\n",
///     " right reader size: `5`,\n",
///     "  left reader data: `\"alpha\"`,\n",
///     " right reader data: `\"bravo\"`"
/// );
/// assert_eq!(actual, expect);
/// # }
/// ```
///
#[macro_export]
macro_rules! assert_read_to_string_eq_other_as_result {
    ($a_reader:expr, $b_reader:expr $(,)?) => ({
        let mut a_string = String::new();
        let mut b_string = String::new();
        let a_result = $a_reader.read_to_string(&mut a_string);
        let b_result = $b_reader.read_to_string(&mut b_string);
        if let Err(a_err) = a_result {
            Err(msg_with_left_reader_and_right_reader_and_err!(
                "assertion failed",
                "assert_read_to_string_eq_other!",
                stringify!($a_reader),
                stringify!($b_reader),
                a_err
            ))
        } else {
            if let Err(b_err) = b_result {
                Err(msg_with_left_reader_and_right_reader_and_err!(
                    "assertion failed",
                    "assert_read_to_string_eq_other!",
                    stringify!($a_reader),
                    stringify!($b_reader),
                    b_err
                ))
            } else {
                let a_size = a_result.unwrap();
                let b_size = b_result.unwrap();
                if a_string == b_string {
                    Ok(())
                } else {
                    Err(msg_with_left_reader_and_right_reader!(
                        "assertion failed",
                        "assert_read_to_string_eq_other!",
                        stringify!($a_reader),
                        stringify!($b_reader),
                        a_size,
                        b_size,
                        a_string,
                        b_string
                    ))
                }
            }
        }
    });
}

#[cfg(test)]
mod test_x_result {
    #[allow(unused_imports)]
    use std::io::Read;

    #[test]
    fn test_assert_read_to_string_eq_other_as_result_x_arity_2_success() {
        let mut a = "alpha".as_bytes();
        let mut b = "alpha".as_bytes();
        let x = assert_read_to_string_eq_other_as_result!(a, b);
        assert_eq!(
            x.unwrap(),
            ()
        );
    }

    #[test]
    fn test_assert_read_to_string_eq_other_as_result_x_arity_2_failure() {
        let mut a = "alpha".as_bytes();
        let mut b = "bravo".as_bytes();
        let x = assert_read_to_string_eq_other_as_result!(a, b);
        assert_eq!(
            x.unwrap_err(),
            concat!(
                "assertion failed: `assert_read_to_string_eq_other!(left_reader, right_reader)`\n",
                "  left reader name: `a`,\n",
                " right reader name: `b`,\n",
                "  left reader size: `5`,\n",
                " right reader size: `5`,\n",
                "  left reader data: `\"alpha\"`,\n",
                " right reader data: `\"bravo\"`"
            )
        );
    }
}

/// Assert a read_to_string() value is equal to another.
///
/// * When true, return `()`.
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
///
/// # fn main() {
/// let mut a = "alpha".as_bytes();
/// let mut b = "alpha".as_bytes();
/// assert_read_to_string_eq_other!(a, b);
/// //-> ()
///
/// let result = panic::catch_unwind(|| {
/// let mut a = "alpha".as_bytes();
/// let mut b = "bravo".as_bytes();
/// assert_read_to_string_eq_other!(a, b);
/// //-> panic!
/// });
/// let actual = result.unwrap_err().downcast::<String>().unwrap().to_string();
/// let expect = concat!(
///     "assertion failed: `assert_read_to_string_eq_other!(left_reader, right_reader)`\n",
///     "  left reader name: `a`,\n",
///     " right reader name: `b`,\n",
///     "  left reader size: `5`,\n",
///     " right reader size: `5`,\n",
///     "  left reader data: `\"alpha\"`,\n",
///     " right reader data: `\"bravo\"`"
/// );
/// assert_eq!(actual, expect);
/// # }
/// ```
///
#[macro_export]
macro_rules! assert_read_to_string_eq_other {
    ($a_reader:expr, $b_reader:expr $(,)?) => ({
        match assert_read_to_string_eq_other_as_result!($a_reader, $b_reader) {
            Ok(()) => (),
            Err(err) => panic!("{}", err),
        }
    });
    ($a_reader:expr, $b_reader:expr, $($arg:tt)+) => ({
        match assert_read_to_string_eq_other_as_result!($a_reader, $b_reader) {
            Ok(()) => (),
            Err(_err) => panic!($($arg)+),
        }
    });
}

#[cfg(test)]
mod test_x_panic {
    use std::io::Read;

    #[test]
    fn test_assert_read_to_string_eq_other_x_arity_2_success() {
        let mut a = "alpha".as_bytes();
        let mut b = "alpha".as_bytes();
        let x = assert_read_to_string_eq_other!(a, b);
        assert_eq!(x, ());
    }

    #[test]
    #[should_panic (expected = "assertion failed: `assert_read_to_string_eq_other!(left_reader, right_reader)`\n  left reader name: `a`,\n right reader name: `b`,\n  left reader size: `5`,\n right reader size: `5`,\n  left reader data: `\"alpha\"`,\n right reader data: `\"bravo\"`")]
    fn test_assert_read_to_string_eq_other_x_arity_2_failure() {
        let mut a = "alpha".as_bytes();
        let mut b = "bravo".as_bytes();
        let _x = assert_read_to_string_eq_other!(a, b);
    }

    #[test]
    fn test_assert_read_to_string_eq_other_x_arity_3_success() {
        let mut a = "alpha".as_bytes();
        let mut b = "alpha".as_bytes();
        let x = assert_read_to_string_eq_other!(a, b, "message");
        assert_eq!(x, ());
    }

    #[test]
    #[should_panic (expected = "message")]
    fn test_assert_read_to_string_eq_other_x_arity_3_failure() {
        let mut a = "alpha".as_bytes();
        let mut b = "bravo".as_bytes();
        let _x = assert_read_to_string_eq_other!(a, b, "message");
    }

}
