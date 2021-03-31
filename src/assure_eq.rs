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
                    Ok(true)
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
                    Ok(true)
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
    fn test_assure_eq_x_arity_2_return_ok() {
        assert_eq!(
            assure_eq!(1, 1).unwrap(), 
            true
        );
    } 

    #[test]
    fn test_assure_eq_x_arity_3_return_ok() {
        assert_eq!(
            assure_eq!(1, 1, "message").unwrap(),
            true
        );
    } 

    #[test]
    fn test_assure_eq_x_arity_2_return_err() {
        assert_eq!(
            assure_eq!(1, 2).unwrap_err(), 
            "assure_eq left:1 right:2"
        );
    } 

    #[test]
    fn test_assure_eq_x_arity_3_return_err() {
        assert_eq!(
            assure_eq!(1, 2, "message").unwrap_err(), 
            "message"
        );
    } 

}
