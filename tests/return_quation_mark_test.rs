use ::assure::*;

fn sum_postive_numbers(a: i32, b: i32) -> Result<i32, String> {
    assure_gt!(a, 0)?;
    assure_gt!(b, 0)?;
    Ok(a + b)
}


#[test]
fn test_x_positive_postive_return_ok() {
    let a = 1;
    let b = 2;
    let x = sum_postive_numbers(a, b);
    assert!(x.is_ok());
    assert_eq!(
        x.unwrap(),
        a + b
    );
}

#[test]
fn test_x_positive_negative_return_err() {
    let a = 1;
    let b = -2;
    let x = sum_postive_numbers(a, b);
    assert!(x.is_err());
    assert_eq!(
        x.unwrap_err(),
        "assure_gt left:-2 right:0"
    );
}
#[test]
fn test_x_negative_positive_return_err() {
    let a = -1;
    let b = 2;
    let x = sum_postive_numbers(a, b);
    assert!(x.is_err());
    assert_eq!(
        x.unwrap_err(),
        "assure_gt left:-1 right:0"
    );
}
