/// Assure two sets are not equal.
///
/// This implementation uses [`HashSet`] and [`assert_eq!`].
///
/// On error, this macro will print the values of the expressions with their
/// debug representations.
///
/// Like [`assert!`], this macro has a second form, where a custom
/// panic message can be provided.
///
/// # Examples
///
/// ```
/// # #[macro_use] extern crate assure; fn main() {
/// let a = vec![1, 2, 3];
/// let b = vec![4, 5, 6];
/// assure_set_ne!(a, b);
/// assure_set_ne!(a, b, "message");
/// # }
/// ```
#[macro_export]
macro_rules! assure_set_ne {
    ($left:expr, $right:expr $(,)?) => ({
        match (&$left, &$right) {
            (left_val, right_val) => {
                let left_set: ::std::collections::HashSet<_> = left_val.into_iter().collect();
                let right_set: std::collections::HashSet<_> = right_val.into_iter().collect();
                if left_set != right_set {
                    Ok(true)
                } else {
                    Err(format!("assure_set_ne left:{:?} right:{:?}", left_set, right_set))
                }
            }
        }
    });
    ($left:expr, $right:expr, $($arg:tt)+) => ({
        match (&($left), &($right)) {
            (left_val, right_val) => {
                let left_set: std::collections::HashSet<_> = left_val.into_iter().collect();
                let right_set: std::collections::HashSet<_> = right_val.into_iter().collect();
                if left_set != right_set {
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
    fn test_assure_set_ne_with_arity_2_return_ok() {
        assert_eq!(
            assure_set_ne!(vec![1, 2], vec![3, 4]).unwrap(),
            true
        );
    } 

    #[test]
    fn test_assure_set_ne_with_arity_3_return_ok() {
        assert_eq!(
            assure_set_ne!(vec![1, 2], vec![3, 4], "message").unwrap(),
            true
        );
    } 

    #[test]
    fn test_assure_set_ne_with_arity_2_return_err() {
        assert_eq!(
            assure_set_ne!(vec![1, 2], vec![1, 2])
            .unwrap_err()
            .chars()
            .into_iter()
            .filter(|x| !x.is_digit(10))
            .collect::<String>(),
            "assure_set_ne left:{, } right:{, }"
        );
    } 

    #[test]
    fn test_assure_set_ne_with_arity_3_return_err() {
        assert_eq!(
            assure_set_ne!(vec![1, 2], vec![1, 2], "message").unwrap_err(),
            "message"
        );
    } 

}
