use ::assertables::*;

fn foo_assume(a: i32, b: i32) -> Result<i32, String> {
    assume_lt!(a, b)?;
    Ok(a)
}


#[test]
fn test_x_success() {
    let a = 1;
    let b = 2;
    let x = foo_assume(a, b);
    assert_eq!(
        x.unwrap(),
        a
    );
}

#[test]
fn test_x_failure() {
    let a = 2;
    let b = 1;
    let x = foo_assume(a, b);
    assert_eq!(
        x.unwrap_err(),
        "assumption failed: `assume_lt(left, right)`\n  left: `2`\n right: `1`"
    );
}
