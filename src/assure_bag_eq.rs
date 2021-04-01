/// Assure two bags are equal.
///
/// This implementation uses [`HashMap`] to count items.
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
/// let a = [1, 1];
/// let b = [1, 1];
/// assure_bag_eq!(&a, &b);
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
/// a.push_back(1);
/// let mut b = LinkedList::new();
/// b.push_back(1);
/// b.push_back(1);
/// assure_bag_eq!(&a, &b);
/// # }
/// ```
#[macro_export]
macro_rules! assure_bag_eq {
    ($left:expr, $right:expr $(,)?) => ({
        match (&$left, &$right) {
            (left_val, right_val) => {
                let mut left_map: ::std::collections::HashMap<_, usize> = ::std::collections::HashMap::new();
                let mut right_map: ::std::collections::HashMap<_, usize> = ::std::collections::HashMap::new();
                for x in left_val.into_iter() {
                    let n = left_map.entry(x).or_insert(0);
                    *n += 1;
                }
                for x in right_val.into_iter() {
                    let n = right_map.entry(x).or_insert(0);
                    *n += 1;
                }
                if left_map == right_map {
                    Ok($left)
                } else {
                    Err(format!("assure_bag_eq left:{:?} right:{:?}", left_map, right_map))
                }
           }
        }
    });
    ($left:expr, $right:expr, $($arg:tt)+) => ({
        match (&($left), &($right)) {
            (left_val, right_val) => {
                let mut left_map: ::std::collections::HashMap<_, usize> = ::std::collections::HashMap::new();
                let mut right_map: ::std::collections::HashMap<_, usize> = ::std::collections::HashMap::new();
                for x in left_val.into_iter() {
                    let n = left_map.entry(x).or_insert(0);
                    *n += 1;
                }
                for x in right_val.into_iter() {
                    let n = right_map.entry(x).or_insert(0);
                    *n += 1;
                }
                if left_map == right_map {
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
    fn test_assure_bag_eq_x_array_arity_2_return_ok() {
        let a = [1, 1];
        let b = [1, 1];
        let x = assure_bag_eq!(&a, &b);
        assert!(x.is_ok());
        assert_eq!(
            x.unwrap(),
            &a
        );
    }

    #[test]
    fn test_assure_bag_eq_x_array_arity_2_return_err() {
        let a = [1, 1];
        let b = [1, 1, 1];
        let x = assure_bag_eq!(&a, &b);
        assert!(x.is_err());
        assert_eq!(
            x
            .unwrap_err()
            .chars()
            .into_iter()
            .take_while(|x| !x.is_whitespace())
            .collect::<String>(),
            "assure_bag_eq"
        );
    }

    #[test]
    fn test_assure_bag_eq_x_array_arity_3_return_ok() {
        let a = [1, 1];
        let b = [1, 1];
        let x = assure_bag_eq!(&a, &b, "message");
        assert!(x.is_ok());
        assert_eq!(
            x.unwrap(),
            &a
        )
    }

    #[test]
    fn test_assure_bag_eq_x_array_arity_3_return_err() {
        let a = [1, 1];
        let b = [1, 1, 1];
        let x = assure_bag_eq!(&a, &b, "message");
        assert!(x.is_err());
        assert_eq!(
            x.unwrap_err(),
            "message"
        );
    }

    #[test]
    fn test_assure_bag_eq_x_vec_arity_2_return_ok() {
        let a = vec![1, 1];
        let b = vec![1, 1];
        let x = assure_bag_eq!(&a, &b);
        assert!(x.is_ok());
        assert_eq!(
            x.unwrap(),
            &a
        );
    }

    #[test]
    fn test_assure_bag_eq_x_vec_arity_2_return_err() {
        let a = vec![1, 1];
        let b = vec![1, 1, 1];
        let x =  assure_bag_eq!(&a, &b);
        assert!(x.is_err());
        assert_eq!(
            x
            .unwrap_err()
            .chars()
            .into_iter()
            .take_while(|x| !x.is_whitespace())
            .collect::<String>(),
            "assure_bag_eq"
        );
    }

    #[test]
    fn test_assure_bag_eq_x_vec_arity_3_return_ok() {
        let a = vec![1, 1];
        let b = vec![1, 1];
        let x = assure_bag_eq!(&a, &b, "message");
        assert!(x.is_ok());
        assert_eq!(
            x.unwrap(),
            &a
        )
    }

    #[test]
    fn test_assure_bag_eq_x_vec_arity_3_return_err() {
        let a = vec![1, 1];
        let b = vec![1, 1, 1];
        let x = assure_bag_eq!(&a, &b, "message");
        assert!(x.is_err());
        assert_eq!(
            x.unwrap_err(),
            "message"
        );
    }

    #[test]
    fn test_assure_bag_eq_x_list_arity_2_return_ok() {
        let mut a: LinkedList<u8> = LinkedList::new();
        a.push_back(1);
        a.push_back(1);
        let mut b: LinkedList<u8> = LinkedList::new();
        b.push_back(1);
        b.push_back(1);
        let x = assure_bag_eq!(&a, &b);
        assert!(x.is_ok());
        assert_eq!(
            x.unwrap(),
            &a
        );
    }

    #[test]
    fn test_assure_bag_eq_x_list_arity_2_return_err() {
        let mut a: LinkedList<u8> = LinkedList::new();
        a.push_back(1);
        a.push_back(1);
        let mut b: LinkedList<u8> = LinkedList::new();
        b.push_back(1);
        b.push_back(1);
        b.push_back(1);
        let x = assure_bag_eq!(&a, &b);
        assert!(x.is_err());
        assert_eq!(
            x
            .unwrap_err()
            .chars()
            .into_iter()
            .take_while(|x| !x.is_whitespace())
            .collect::<String>(),
            "assure_bag_eq"
        );
    }

    #[test]
    fn test_assure_bag_eq_x_list_arity_3_return_ok() {
        let mut a: LinkedList<u8> = LinkedList::new();
        a.push_back(1);
        a.push_back(1);
        let mut b: LinkedList<u8> = LinkedList::new();
        b.push_back(1);
        b.push_back(1);
        let x = assure_bag_eq!(&a, &b, "message");
        assert!(x.is_ok());
        assert_eq!(
            x.unwrap(),
            &a
        );
    }

    #[test]
    fn test_assure_bag_eq_x_list_arity_3_return_err() {
        let mut a: LinkedList<u8> = LinkedList::new();
        a.push_back(1);
        a.push_back(1);
        let mut b: LinkedList<u8> = LinkedList::new();
        b.push_back(1);
        b.push_back(1);
        b.push_back(1);
        let x = assure_bag_eq!(&a, &b, "message");
        assert!(x.is_err());
        assert_eq!(
            x.unwrap_err(),
            "message"
        );
    }

}
