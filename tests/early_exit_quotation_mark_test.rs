use ::assertables::*;

fn foo_assure(a: i32, b: i32) -> Result<i32, String> {
    assertable_lt!(a, b)?;
    Ok(a)
}


#[test]
fn test_x_success() {
    let a = 1;
    let b = 2;
    let x = foo_assure(a, b);
    assert_eq!(
        x.unwrap(),
        a
    );
}

#[test]
fn test_x_failure() {
    let a = 2;
    let b = 1;
    let x = foo_assure(a, b);
    assert_eq!(
        x.unwrap_err(),
        "assertable failed: `assertable_lt!(left, right)`\n  left: `2`,\n right: `1`"
    );
}
