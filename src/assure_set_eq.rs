/// Assure two sets are equal.
///
/// This implementation uses [`HashSet`] and [`assure_eq!`].
///
/// On error, this macro will print the values of the expressions with their
/// debug representations.
///
/// Like [`assure!`], this macro has a second form, 
/// where a custom message can be provided.
///
/// # Example with arrays
///
/// ```rust
/// # #[macro_use] extern crate assure; fn main() {
/// let a = [1, 2];
/// let b = [2, 1];
/// assure_set_eq!(a, b);
/// # }
/// ```
///
/// # Example with linked lists
///
/// ```rust
/// # #[macro_use] extern crate assure; fn main() {
/// use std::collections::LinkedList;
/// let mut a = LinkedList::new();
/// a.push_back(1);
/// a.push_back(2);
/// let mut b = LinkedList::new();
/// b.push_back(2);
/// b.push_back(1);
/// assure_set_eq!(a, b);
/// # }
/// ```
#[macro_export]
macro_rules! assure_set_eq {
    ($left:expr, $right:expr $(,)?) => ({
        match (&$left, &$right) {
            (left_val, right_val) => {
                let left_set: ::std::collections::HashSet<_> = left_val.into_iter().collect();
                let right_set: std::collections::HashSet<_> = right_val.into_iter().collect();
                if left_set == right_set {
                    Ok(true)
                } else {
                    Err(format!("assure_set_eq left:{:?} right:{:?}", left_set, right_set))
                }
           }
        }
    });
    ($left:expr, $right:expr, $($arg:tt)+) => ({
        match (&($left), &($right)) {
            (left_val, right_val) => {
                let left_set: std::collections::HashSet<_> = left_val.into_iter().collect();
                let right_set: std::collections::HashSet<_> = right_val.into_iter().collect();
                if left_set == right_set {
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
    use std::collections::LinkedList;

    #[test]
    fn test_assure_set_eq_x_array_arity_2_return_ok() {
        assert_eq!(
            assure_set_eq!([1, 2], [1, 2]).unwrap(),
            true
        );
    } 

    #[test]
    fn test_assure_set_eq_x_array_arity_3_return_ok() {
        assert_eq!(
            assure_set_eq!([1, 2], [1, 2], "message").unwrap(),
            true
        )
    } 

    #[test]
    fn test_assure_set_eq_x_vec_arity_2_return_ok() {
        assert_eq!(
            assure_set_eq!(vec![1, 2], vec![1, 2]).unwrap(),
            true
        );
    } 

    #[test]
    fn test_assure_set_eq_x_vec_arity_3_return_ok() {
        assert_eq!(
            assure_set_eq!(vec![1, 2], vec![1, 2], "message").unwrap(),
            true
        )
    } 

    #[test]
    fn test_assure_set_eq_x_list_arity_2_return_ok() {
        let mut a: LinkedList<u8> = LinkedList::new();
        a.push_back(1);
        a.push_back(2);
        let mut b: LinkedList<u8> = LinkedList::new();
        b.push_back(2);
        b.push_back(1);
        assert_eq!(
            assure_set_eq!(a, b).unwrap(),
            true
        );
    } 

    #[test]
    fn test_assure_set_eq_x_list_arity_3_return_ok() {
        let mut a: LinkedList<u8> = LinkedList::new();
        a.push_back(1);
        a.push_back(2);
        let mut b: LinkedList<u8> = LinkedList::new();
        b.push_back(2);
        b.push_back(1);
        assert_eq!(
            assure_set_eq!(a, b, "message").unwrap(),
            true
        );
    } 

    #[test]
    fn test_assure_set_eq_x_array_arity_2_return_err() {
        assert_eq!(
            assure_set_eq!([1, 2], [3, 4])
            .unwrap_err()
            .chars()
            .into_iter()
            .filter(|x| !x.is_digit(10))
            .collect::<String>(),
            "assure_set_eq left:{, } right:{, }"
        );
    } 

    #[test]
    fn test_assure_set_eq_x_array_arity_3_return_err() {
        assert_eq!(
            assure_set_eq!([1, 2], [3, 4], "message").unwrap_err(),
            "message"
        )
    } 

    #[test]
    fn test_assure_set_eq_x_vec_arity_2_return_err() {
        assert_eq!(
            assure_set_eq!(vec![1, 2], vec![3, 4])
            .unwrap_err()
            .chars()
            .into_iter()
            .filter(|x| !x.is_digit(10))
            .collect::<String>(),
            "assure_set_eq left:{, } right:{, }"
        );
    } 

    #[test]
    fn test_assure_set_eq_x_vec_arity_3_return_err() {
        assert_eq!(
            assure_set_eq!(vec![1, 2], vec![3, 4], "message").unwrap_err(),
            "message"
        )
    } 
    #[test]
    fn test_assure_set_eq_x_list_arity_2_return_err() {
        let mut a: LinkedList<u8> = LinkedList::new();
        a.push_back(1);
        a.push_back(2);
        let mut b: LinkedList<u8> = LinkedList::new();
        b.push_back(3);
        b.push_back(4);
        assert_eq!(
            assure_set_eq!(a, b)
            .unwrap_err()
            .chars()
            .into_iter()
            .filter(|x| !x.is_digit(10))
            .collect::<String>(),
            "assure_set_eq left:{, } right:{, }"
        );
    } 

    #[test]
    fn test_assure_set_eq_x_list_arity_3_return_err() {
        let mut a: LinkedList<u8> = LinkedList::new();
        a.push_back(1);
        a.push_back(2);
        let mut b: LinkedList<u8> = LinkedList::new();
        b.push_back(3);
        b.push_back(4);
        assert_eq!(
            assure_set_eq!(a, b, "message").unwrap_err(),
            "message"
        );
    } 

}
