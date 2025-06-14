use assertables::*;
use std::sync::Mutex;

struct Foo {
    s: String,
}

#[test]

fn f() {
    let s = Mutex::new(Foo {
        s: "alfa".to_string(),
    });
    let inner = s.lock().unwrap();
    assert_contains!(inner.s, "lf");
}
