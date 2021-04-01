/// Assure one value is less than another value.
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
/// assure_io_gt!(2, 1);
/// assure_io_gt!(2, 1, "message");
/// # }
/// ```
#[macro_export]
macro_rules! assure_io_gt {
    ($left:expr, $right:expr $(,)?) => ({
        match (&$left, &$right) {
            (left_val, right_val) => {
                if (left_val > right_val) {
                    Ok($left)
                } else {
                    Err(std::io::Error::new(std::io::ErrorKind::InvalidInput, format!("assure_io_gt left:{:?} right:{:?}",  left_val, right_val)))
                }
            }
        }
    });
    ($left:expr, $right:expr, $($arg:tt)+) => ({
        match (&($left), &($right)) {
            (left_val, right_val) => {
                if (left_val > right_val) {
                    Ok($left)
                } else {
                    Err(std::io::Error::new(std::io::ErrorKind::InvalidInput, $($arg)+))
                }
            }
        }
    });
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_assure_io_gt_x_arity_2_return_ok() {
        let a = 2;
        let b = 1;
        let x = assure_io_gt!(a, b);
        assert!(x.is_ok());
        assert_eq!(
            x.unwrap(),
            a
        );
    }

    #[test]
    fn test_assure_io_gt_x_arity_2_return_err() {
        let a = 1;
        let b = 2;
        let x = assure_io_gt!(a, b);
        assert!(x.is_err());
        assert_eq!(
            x.unwrap_err().get_ref().unwrap().to_string(),
            "assure_io_gt left:1 right:2"
        );
    }

    #[test]
    fn test_assure_io_gt_x_arity_3_return_ok() {
        let a = 2;
        let b = 1;
        let x = assure_io_gt!(a, b, "message");
        assert!(x.is_ok());
        assert_eq!(
            x.unwrap(),
            a
        );
    }

    #[test]
    fn test_assure_io_gt_x_arity_3_return_err() {
        let a = 1;
        let b = 2;
        let x = assure_io_gt!(a, b, "message");
        assert!(x.is_err());
        assert_eq!(
            x.unwrap_err().get_ref().unwrap().to_string(),
            "message"
        );
    }

}
