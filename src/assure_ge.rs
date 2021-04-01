/// Assure one value is greater than or equal to another value.
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
/// assure_ge!(2, 1);
/// assure_ge!(2, 1, "message");
/// # }
/// ```
#[macro_export]
macro_rules! assure_ge {
    ($left:expr, $right:expr $(,)?) => ({
        match (&$left, &$right) {
            (left_val, right_val) => {
                if (left_val >= right_val) {
                    Ok($left)
                } else {
                    Err(format!("assure_ge left:{:?} right:{:?}", left_val, right_val))
                }
            }
        }
    });
    ($left:expr, $right:expr, $($arg:tt)+) => ({
        match (&($left), &($right)) {
            (left_val, right_val) => {
                if (left_val >= right_val) {
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
    fn test_assure_ge_x_arity_2_return_ok() {
        let a = 2;
        let b = 1;
        let x = assure_ge!(a, b);
        assert!(x.is_ok());
        assert_eq!(
            x.unwrap(),
            a
        );
    }

    #[test]
    fn test_assure_ge_x_arity_2_return_err() {
        let a = 1;
        let b = 2;
        let x = assure_ge!(a, b);
        assert!(x.is_err());
        assert_eq!(
            x.unwrap_err(),
            "assure_ge left:1 right:2"
        );
    }

    #[test]
    fn test_assure_ge_x_arity_3_return_ok() {
        let a = 2;
        let b = 1;
        let x = assure_ge!(a, b, "message");
        assert!(x.is_ok());
        assert_eq!(
            x.unwrap(),
            a
        );
    }

    #[test]
    fn test_assure_ge_x_arity_3_return_err() {
        let a = 1;
        let b = 2;
        let x = assure_ge!(a, b, "message");
        assert!(x.is_err());
        assert_eq!(
            x.unwrap_err(),
            "message"
        );
    }

}
