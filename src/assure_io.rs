/// Assure a condition is true.
///
/// If true, then return Ok(true).
///
/// Otherwise, return Err(std::io::Error …).
///
/// This macro has a second form, where a custom
/// message can be provided.
///
/// # Examples
///
/// ```
/// # #[macro_use] extern crate assure; fn main() {
/// assure_io!(true);
/// assure_io!(true, "message");
/// # }
/// ```
#[macro_export]
macro_rules! assure_io {
    ($x:expr $(,)?) => ({
        if ($x) {
            Ok($x)
        } else {
            Err(std::io::Error::new(std::io::ErrorKind::InvalidInput, "assure_io"))
        }
    });
    ($x:expr, $($arg:tt)+) => ({
        if ($x) {
            Ok($x)
        } else {
            Err(std::io::Error::new(std::io::ErrorKind::InvalidInput, $($arg)+))
        }
    });
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_assure_io_x_arity_2_return_ok() {
        let a = true;
        let x = assure_io!(a);
        assert!(x.is_ok());
        assert_eq!(x.unwrap(), a);
    }

    #[test]
    fn test_assure_io_x_arity_2_return_err() {
        let a = false;
        let x = assure_io!(a);
        assert!(x.is_err());
    }

    #[test]
    fn test_assure_io_x_arity_3_return_ok() {
        let a = true;
        let x = assure_io!(a, "message");
        assert!(x.is_ok());
        assert_eq!(x.unwrap(), a);
    }

    #[test]
    fn test_assure_io_x_arity_3_return_err() {
        let a = false;
        let x = assure_io!(a, "message");
        assert!(x.is_err());
    }

}
