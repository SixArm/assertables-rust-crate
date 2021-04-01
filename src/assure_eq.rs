/// Assure two values are equal.
///
/// If true, then return Ok(true).
///
/// Otherwise, return Err(…).
///
/// This macro has a second form, where a custom
/// message can be provided.
///
/// # Examples
///
/// ```
/// # #[macro_use] extern crate assure; fn main() {
/// assure_eq!(1, 1);
/// assure_eq!(1, 1, "message");
/// # }
/// ```
#[macro_export]
macro_rules! assure_eq {
    ($left:expr, $right:expr $(,)?) => ({
        match (&$left, &$right) {
            (left_val, right_val) => {
                if (left_val == right_val) {
                    Ok($left)
                } else {
                    Err(format!("assure_eq left:{:?} right:{:?}", left_val, right_val))
                }
            }
        }
    });
    ($left:expr, $right:expr, $($arg:tt)+) => ({
        match (&($left), &($right)) {
            (left_val, right_val) => {
                if (left_val == right_val) {
                    Ok($left)
                } else {
                    Err($($arg)+)
                }
            }
        }
    });
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_assure_eq_x_i32_arity_2_return_ok() {
        let a = 1;
        let b = 1;
        let x = assure_eq!(a, b);
        assert!(x.is_ok());
        assert_eq!(
            x.unwrap(),
            a
        );
    }

    #[test]
    fn test_assure_eq_x_i32_arity_2_return_err() {
        let a = 1;
        let b = 2;
        let x =  assure_eq!(a, b);
        assert!(x.is_err());
        assert_eq!(
            x.unwrap_err(),
            "assure_eq left:1 right:2"
        );
    }

    #[test]
    fn test_assure_eq_x_i32_arity_3_return_ok() {
        let a = 1;
        let b = 1;
        let x = assure_eq!(a, b, "message");
        assert!(x.is_ok());
        assert_eq!(
            x.unwrap(),
            a
        );
    }

    #[test]
    fn test_assure_eq_x_i32_arity_3_return_err() {
        let a = 1;
        let b = 2;
        let x = assure_eq!(a, b, "message");
        assert!(x.is_err());
        assert_eq!(
            x.unwrap_err(),
            "message"
        );
    }

    #[test]
    fn test_assure_eq_x_str_arity_2_return_ok() {
        let a = "aa";
        let b = "aa";
        let x = assure_eq!(a, b);
        assert!(x.is_ok());
        assert_eq!(
            x.unwrap(),
            a
        );
    }

    #[test]
    fn test_assure_eq_x_str_arity_2_return_err() {
        let a = "aa";
        let b = "bb";
        let x = assure_eq!(a, b);
        assert!(x.is_err());
        assert_eq!(
            x.unwrap_err(),
            "assure_eq left:\"aa\" right:\"bb\""
        );
    }

    #[test]
    fn test_assure_eq_x_str_arity_3_return_ok() {
        let a = "aa";
        let b = "aa";
        let x = assure_eq!(a, b, "message");
        assert!(x.is_ok());
        assert_eq!(
            x.unwrap(),
            a
        );
    }

    #[test]
    fn test_assure_eq_x_str_arity_3_return_err() {
        let a = "aa";
        let b = "bb";
        let x = assure_eq!(a, b, "message");
        assert!(x.is_err());
        assert_eq!(
            x.unwrap_err(),
            "message"
        );
    }

}
