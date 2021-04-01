use ::assure::*;

fn equality(a: i32, b: i32) -> Result<bool, String> {
    assure_eq!(a, b, format!("error {} and {}", a, b))?;
    Ok(true)
}


#[test]
fn test_x_return_err() {
    let a = 1;
    let b = 2;
    let x = equality(a, b);
    assert!(x.is_err());
    assert_eq!(
        x.unwrap_err(),
        "error 1 and 2"
    );
}
