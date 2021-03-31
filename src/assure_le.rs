/// Assure one value is less than or equal to another value.
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
/// assure_le!(1, 2);
/// assure_le!(1, 2, "message");
/// # }
/// ```
#[macro_export]
macro_rules! assure_le {
    ($left:expr, $right:expr $(,)?) => ({
        match (&$left, &$right) {
            (left_val, right_val) => {
                if (left_val <= right_val) {
                    Ok($left)
                } else {
                    Err(format!("assure_le left:{:?} right:{:?}", left_val, right_val))
                }
            }
        }
    });
    ($left:expr, $right:expr, $($arg:tt)+) => ({
        match (&($left), &($right)) {
            (left_val, right_val) => {
                if (left_val <= right_val) {
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
    fn test_assure_le_x_arity_2_return_ok() {
        let a = 1;
        let b = 2;
        let x = assure_le!(a, b);
        assert!(x.is_ok());
        assert_eq!(
            x.unwrap(), 
            a
        );
    } 

    #[test]
    fn test_assure_le_x_arity_2_return_err() {
        let a = 2;
        let b = 1;
        let x = assure_le!(a, b);
        assert!(x.is_err());
        assert_eq!(
            x.unwrap_err(), 
            "assure_le left:2 right:1"
        );
    } 

    #[test]
    fn test_assure_le_x_arity_3_return_ok() {
        let a = 1;
        let b = 2;
        let x = assure_le!(a, b, "message");
        assert!(x.is_ok());
        assert_eq!(
            x.unwrap(),
            a
        );
    } 

    #[test]
    fn test_assure_le_x_arity_3_return_err() {
        let a = 2;
        let b = 1;
        let x = assure_le!(a, b, "message");
        assert!(x.is_err());
        assert_eq!(
            x.unwrap_err(), 
            "message"
        );
    } 

}
