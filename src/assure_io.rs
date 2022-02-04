/// Assure a condition is true.
///
/// * When true, return `Ok(true)`.
///
/// * When false, return `Ok(false)`.
///
/// # Examples
///
/// ```rust
/// # #[macro_use] extern crate assertables;
/// # use std::panic;
/// # fn main() {
/// let x = assure_io!(true);
/// //-> Ok(true)
///
/// let x = assure_io!(false);
/// //-> Ok(false)
/// # }
/// ```
///
/// This macro has a second form where a custom message can be provided.
#[macro_export]
macro_rules! assure_io {
    ($x:expr $(,)?) => ({
        if ($x) {
            Ok(true)
        } else {
            Ok(false)
        }
    } as Result<bool, std::io::Error>);
    ($x:expr, $($arg:tt)+) => ({
        if ($x) {
            Ok(true)
        } else {
            Ok(false)
        }
    } as Result<bool, std::io::Error>);
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_assure_io_x_arity_2_success() {
        let a = true;
        let x = assure_io!(a);
        assert_eq!(
            x.unwrap(),
            true
        );
    }

    #[test]
    fn test_assure_io_x_arity_2_failure() {
        let a = false;
        let x = assure_io!(a);
        assert_eq!(
            x.unwrap(),
            false
            //x.unwrap_err().get_ref().unwrap().to_string(),
            //"assure_io condition:false"
        );
    }

    #[test]
    fn test_assure_io_x_arity_3_success() {
        let a = true;
        let x = assure_io!(a, "message");
        assert_eq!(
            x.unwrap(),
            true
        );
    }

    #[test]
    fn test_assure_io_x_arity_3_failure() {
        let a = false;
        let x = assure_io!(a, "message");
        assert_eq!(
            x.unwrap(),
            false
            //x.unwrap_err().get_ref().unwrap().to_string(),
            //"assure_io condition:false"
        );
    }

}
