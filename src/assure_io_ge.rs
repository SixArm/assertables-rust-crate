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
/// assure_io_ge!(2, 1);
/// assure_io_ge!(2, 1, "message");
/// # }
/// ```
#[macro_export]
macro_rules! assure_io_ge {
    ($left:expr, $right:expr $(,)?) => ({
        match (&$left, &$right) {
            (left_val, right_val) => {
                if (left_val >= right_val) {
                    Ok(true)
                } else {
                    Err(std::io::Error::new(std::io::ErrorKind::InvalidInput, format!("assure_io_ge left:{:?} right:{:?}",  left_val, right_val)))
                }
            }
        }
    });
    ($left:expr, $right:expr, $($arg:tt)+) => ({
        match (&($left), &($right)) {
            (left_val, right_val) => {
                if (left_val >= right_val) {
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
    fn test_assure_io_ge_x_arity_2_return_ok() {
        assert_eq!(
            assure_io_ge!(2, 1).unwrap(), 
            true
        );
    } 

    #[test]
    fn test_assure_io_ge_x_arity_3_return_ok() {
        assert_eq!(
            assure_io_ge!(2, 1, "message").unwrap(),
            true
        );
    } 

    #[test]
    fn test_assure_io_ge_x_arity_2_return_err() {
        assert_eq!(
            assure_io_ge!(1, 2).unwrap_err().get_ref().unwrap().to_string(), 
            "assure_io_ge left:1 right:2"
        );
    } 

    #[test]
    fn test_assure_io_ge_x_arity_3_return_err() {
        assert_eq!(
            assure_io_ge!(1, 2, "message").unwrap_err().get_ref().unwrap().to_string(),
            "message"
        );
    } 

}
