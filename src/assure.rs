/// Assure a condition is true.
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
/// assure!(true);
/// assure!(true, "message");
/// # }
/// ```
#[macro_export]
macro_rules! assure {
    ($x:expr $(,)?) => ({
        if ($x) {
            Ok($x)
        } else {
            Err("assure")
        }
    });
    ($x:expr, $($arg:tt)+) => ({
        if ($x) {
            Ok($x)
        } else {
            Err($($arg)+)
        }
    });
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_assure_x_arity_2_return_ok() {
        let a = true;
        let x = assure!(a);
        assert!(x.is_ok());
        assert_eq!(
            x.unwrap(),
            a
        );
    }

    #[test]
    fn test_assure_x_arity_2_return_err() {
        let a = false;
        let x = assure!(a);
        assert!(x.is_err());
        assert_eq!(
            x.unwrap_err(),
            "assure"
        );
    }

    #[test]
    fn test_assure_x_arity_3_return_ok() {
        let a = true;
        let x = assure!(a, "message");
        assert!(x.is_ok());
        assert_eq!(
            x.unwrap(),
            a
        );
    }

    #[test]
    fn test_assure_x_arity_3_return_err() {
        let a = false;
        let x = assure!(a, "message");
        assert!(x.is_err());
        assert_eq!(
            x.unwrap_err(),
            "message"
        );
    }

}
