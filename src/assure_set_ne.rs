/// Assure two sets are not equal.
///
/// This implementation uses [`HashSet`] ro count items.
///
/// On error, this macro will print the values of the expressions with their
/// debug representations.
///
/// Like [`assert!`], this macro has a second form, where a custom
/// panic message can be provided.
///
/// # Example with arrays
///
/// ```rust
/// # #[macro_use] extern crate assure; fn main() {
/// let a = [1, 2];
/// let b = [3, 4];
/// assure_set_ne!(&a, &b);
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
/// b.push_back(3);
/// b.push_back(4);
/// assure_set_ne!(&a, &b);
/// # }
/// ```
#[macro_export]
macro_rules! assure_set_ne {
    ($left:expr, $right:expr $(,)?) => ({
        match (&$left, &$right) {
            (left_val, right_val) => {
                let left_set: ::std::collections::HashSet<_> = left_val.into_iter().collect();
                let right_set: ::std::collections::HashSet<_> = right_val.into_iter().collect();
                if left_set != right_set {
                    Ok($left)
                } else {
                    Err(format!("assure_set_ne left:{:?} right:{:?}", left_set, right_set))
                }
            }
        }
    });
    ($left:expr, $right:expr, $($arg:tt)+) => ({
        match (&($left), &($right)) {
            (left_val, right_val) => {
                let left_set: ::std::collections::HashSet<_> = left_val.into_iter().collect();
                let right_set: ::std::collections::HashSet<_> = right_val.into_iter().collect();
                if left_set != right_set {
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
    use std::collections::LinkedList;

    #[test]
    fn test_assure_set_ne_x_array_arity_2_return_ok() {
        let a = [1, 2];
        let b = [3, 4];
        let x = assure_set_ne!(&a, &b);
        assert!(x.is_ok());
        assert_eq!(
            x.unwrap(),
            &a
        );
    }

    #[test]
    fn test_assure_set_ne_x_array_arity_2_return_err() {
        let a = [1, 2];
        let b = [1, 2];
        let x = assure_set_ne!(&a, &b);
        assert!(x.is_err());
        assert_eq!(
            x
            .unwrap_err()
            .chars()
            .into_iter()
            .take_while(|x| !x.is_whitespace())
            .collect::<String>(),
            "assure_set_ne"
        );
    } 

    #[test]
    fn test_assure_set_ne_x_array_arity_3_return_ok() {
        let a = [1, 2];
        let b = [3, 4];
        let x = assure_set_ne!(&a, &b, "message");
        assert!(x.is_ok());
        assert_eq!(
            x.unwrap(),
            &a
        );
    } 

    #[test]
    fn test_assure_set_ne_x_array_arity_3_return_err() {
        let a = [1, 2];
        let b = [1, 2];
        let x = assure_set_ne!(&a, &b, "message");
        assert!(x.is_err());
        assert_eq!(
            x.unwrap_err(),
            "message"
        );
    } 

    #[test]
    fn test_assure_set_ne_x_vec_arity_2_return_ok() {
        let a = vec![1, 2];
        let b = vec![3, 4];
        let x = assure_set_ne!(&a, &b);
        assert!(x.is_ok());
        assert_eq!(
            x.unwrap(),
            &a
        );
    } 

    #[test]
    fn test_assure_set_ne_x_vac_arity_2_return_err() {
        let a = vec![1, 2];
        let b = vec![1, 2];
        let x = assure_set_ne!(&a, &b);
        assert!(x.is_err());
        assert_eq!(
            x
            .unwrap_err()
            .chars()
            .into_iter()
            .take_while(|x| !x.is_whitespace())
            .collect::<String>(),
            "assure_set_ne"
        );
    } 

    #[test]
    fn test_assure_set_ne_x_vec_arity_3_return_ok() {
        let a = vec![1, 2];
        let b = vec![3, 4];
        let x = assure_set_ne!(&a, &b, "message");
        assert!(x.is_ok());
        assert_eq!(
            x.unwrap(),
            &a
        );
    } 

    #[test]
    fn test_assure_set_ne_x_vec_arity_3_return_err() {
        let a = vec![1, 2];
        let b = vec![1, 2];
        let x = assure_set_ne!(&a, &b, "message");
        assert!(x.is_err());
        assert_eq!(
            x.unwrap_err(),
            "message"
        );
    } 

    #[test]
    fn test_assure_set_ne_x_list_arity_2_return_ok() {
        let mut a: LinkedList<u8> = LinkedList::new();
        a.push_back(1);
        a.push_back(2);
        let mut b: LinkedList<u8> = LinkedList::new();
        b.push_back(3);
        b.push_back(4);    
        let x = assure_set_ne!(&a, &b);
        assert!(x.is_ok());
        assert_eq!(
            x.unwrap(),
            &a
        );
    } 

    #[test]
    fn test_assure_set_ne_x_list_arity_2_return_err() {
        let mut a: LinkedList<u8> = LinkedList::new();
        a.push_back(1);
        a.push_back(2);
        let mut b: LinkedList<u8> = LinkedList::new();
        b.push_back(2);
        b.push_back(1);
        let x = assure_set_ne!(&a, &b);
        assert!(x.is_err());
        assert_eq!(
            x
            .unwrap_err()
            .chars()
            .into_iter()
            .take_while(|x| !x.is_whitespace())
            .collect::<String>(),
            "assure_set_ne"
        );
    } 

    #[test]
    fn test_assure_set_ne_x_list_arity_3_return_ok() {
        let mut a: LinkedList<u8> = LinkedList::new();
        a.push_back(1);
        a.push_back(2);
        let mut b: LinkedList<u8> = LinkedList::new();
        b.push_back(3);
        b.push_back(4);
        let x = assure_set_ne!(&a, &b, "message");
        assert!(x.is_ok());
        assert_eq!(
            x.unwrap(),
            &a
        );
    } 

    #[test]
    fn test_assure_set_ne_x_list_arity_3_return_err() {
        let mut a: LinkedList<u8> = LinkedList::new();
        a.push_back(1);
        a.push_back(2);
        let mut b: LinkedList<u8> = LinkedList::new();
        b.push_back(2);
        b.push_back(1);
        let x = assure_set_ne!(&a, &b, "message");
        assert!(x.is_err());
        assert_eq!(
            x.unwrap_err(),
            "message"
        );
    } 

}
