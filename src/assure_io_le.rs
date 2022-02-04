/// Assure one value is less than or equal to another value.
///
/// * When true, return `Ok(true)`.
///
/// * When false, return `Ok(false)`.
///
/// # Examples
///
/// ```rust
/// # #[macro_use] extern crate assertables;
/// # fn main() {
/// let x = assure_io_le!(1, 2);
/// assert_eq!(x.unwrap(), true);
/// # }
/// ```
///
/// ```rust
/// # #[macro_use] extern crate assertables;
/// # fn main() {
/// let x = assure_io_le!(2, 1);
/// assert_eq!(x.unwrap(), false);
/// # }
/// ```
///
/// This macro has a second form where a custom message can be provided.
#[macro_export]
macro_rules! assure_io_le {
    ($left:expr, $right:expr $(,)?) => ({
        match (&$left, &$right) {
            (left_val, right_val) => {
                if (left_val <= right_val) {
                    Ok(true)
                } else {
                    Ok(false)
                }
            }
        }
    } as Result<bool, String>);
    ($left:expr, $right:expr, $($arg:tt)+) => ({
        match (&($left), &($right)) {
            (left_val, right_val) => {
                if (left_val <= right_val) {
                    Ok(true)
                } else {
                    Ok(false)
                }
            }
        }
    } as Result<bool, String>);
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_assure_io_le_x_arity_2_success() {
        let a = 1;
        let b = 2;
        let x = assure_io_le!(a, b);
        assert_eq!(
            x.unwrap(),
            true
        );
    }

    #[test]
    fn test_assure_io_le_x_arity_2_failure() {
        let a = 2;
        let b = 1;
        let x = assure_io_le!(a, b);
        assert_eq!(
            x.unwrap(),
            false
            //x.unwrap_err().get_ref().unwrap().to_string(),
            //"assure_io_le left:2 right:1"
        );
    }

    #[test]
    fn test_assure_io_le_x_arity_3_success() {
        let a = 1;
        let b = 2;
        let x = assure_io_le!(a, b, "message");
        assert_eq!(
            x.unwrap(),
            true
        );
    }

    #[test]
    fn test_assure_io_le_x_arity_3_failure() {
        let a = 2;
        let b = 1;
        let x = assure_io_le!(a, b, "message");
        assert_eq!(
            x.unwrap(),
            false
            //x.unwrap_err().get_ref().unwrap().to_string(),
            //"message"
        );
    }

}
