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
            Ok(true)
        } else {
            Err("assure")
        }
    });
    ($x:expr, $($arg:tt)+) => ({
        if ($x) {
            Ok(true)
        } else {
            Err($($arg)+)
        }
    });
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_assure_x_arity_2_return_ok() {
        assert_eq!(
            assure!(true).unwrap(), 
            true
        );
    } 

    #[test]
    fn test_assure_x_arity_3_return_ok() {
        assert_eq!(
            assure!(true, "message").unwrap(), 
            true
        );
    } 

    #[test]
    fn test_assure_x_arity_2_return_err() {
        assert_eq!(
            assure!(false).unwrap_err(), 
            "assure"
        );
    } 

    #[test]
    fn test_assure_x_arity_3_return_err() {
        assert_eq!(
            assure!(false, "message").unwrap_err(), 
            "message"
        );
    } 

}
