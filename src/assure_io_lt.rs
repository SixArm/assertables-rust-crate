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
/// assure_io_lt!(1, 2);
/// assure_io_lt!(1, 2, "message");
/// # }
/// ```
#[macro_export]
macro_rules! assure_io_lt {
    ($left:expr, $right:expr $(,)?) => ({
        match (&$left, &$right) {
            (left_val, right_val) => {
                if (left_val < right_val) {
                    Ok(true)
                } else {
                    Err(std::io::Error::new(std::io::ErrorKind::InvalidInput, format!("assure_io_lt left:{:?} right:{:?}",  left_val, right_val)))
                }
            }
        }
    });
    ($left:expr, $right:expr, $($arg:tt)+) => ({
        match (&($left), &($right)) {
            (left_val, right_val) => {
                if (left_val < right_val) {
                    Ok(true)
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
    fn test_assure_io_lt_x_arity_2_return_ok() {
        assert_eq!(
            assure_io_lt!(1, 2).unwrap(), 
            true
        );
    } 

    #[test]
    fn test_assure_io_lt_x_arity_3_return_ok() {
        assert_eq!(
            assure_io_lt!(1, 2, "message").unwrap(),
            true
        );
    } 

    #[test]
    fn test_assure_io_lt_x_arity_2_return_err() {
        assert_eq!(
            assure_io_lt!(2, 1).unwrap_err().get_ref().unwrap().to_string(), 
            "assure_io_lt left:2 right:1"
        );
    } 

    #[test]
    fn test_assure_io_lt_x_arity_3_return_err() {
        assert_eq!(
            assure_io_lt!(2, 1, "message").unwrap_err().get_ref().unwrap().to_string(),
            "message"
        );
    } 

}
