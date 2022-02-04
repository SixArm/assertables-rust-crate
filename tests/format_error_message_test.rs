use ::assertables::*;

fn foo_assert(a: i32, b: i32) -> () {
    assert_lt!(a, b, format!("message {} and {}", a, b))
}

fn foo_assure(a: i32, b: i32) -> Result<(), String> {
    assertable_lt!(a, b, format!("message {} and {}", a, b))?;
    Ok(())
}

#[test]
#[should_panic (expected = "message 2 and 1")]
fn test_foo_assert_x_failure() {
    let a = 2;
    let b = 1;
    let _ = foo_assert(a, b);
}

#[test]
fn test_foo_assertable_x_failure() {
    let a = 2;
    let b = 1;
    let x = foo_assure(a, b);
    assert_eq!(
        x.unwrap_err(),
        "message 2 and 1"
    );
}
