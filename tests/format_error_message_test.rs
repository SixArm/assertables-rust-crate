use ::assertable::*;

fn foo_assert(a: i32, b: i32) -> () {
    assert_lt!(a, b, format!("message {} and {}", a, b))
}

fn foo_assume(a: i32, b: i32) -> Result<bool, String> {
    assume_lt!(a, b, format!("message {} and {}", a, b))?;
    Ok(true)
}

fn foo_assure(a: i32, b: i32) -> Result<bool, String> {
    return assure_lt!(a, b, format!("message {} and {}", a, b))
}

#[test]
#[should_panic (expected = "message 2 and 1")]
fn test_foo_assert_x_failure() {
    let a = 2;
    let b = 1;
    let _ = foo_assert(a, b);
}

#[test]
fn test_foo_assume_x_failure() {
    let a = 2;
    let b = 1;
    let x = foo_assume(a, b);
    assert_eq!(
        x.unwrap_err(),
        "message 2 and 1"
    );
}

#[test]
fn test_foo_assure_x_failure() {
    let a = 2;
    let b = 1;
    let x = foo_assure(a, b);
    assert_eq!(
        x.unwrap(),
        false
    );
}
