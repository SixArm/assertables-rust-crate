# Forms


## Forms for panic! versus Err()

The assert macros have three forms that you can use depending on your goals:


```rust
assert_gt!(a, b); // return () or panic!(…), for typical compile-time testing

debug_assert_gt!(a, b); // return () or panic!(…), for a non-optimized runtime

assert_gt_as_result!(a, b); // return Result Ok(()) or Err(…), for any runtime
```


## Forms for messages

The assert macros have forms for default messages versus custom messages.

```rust
assert_gt!(1, 2); // panic!("assertion failed: assert_gt(1, 2)…")

assert_gt!(1, 2, "message"); // panic!("message")
```


## Forms for comparing an other versus an expression

Some assert macros have forms for comparing an other versus an expression:

```rust
assert_read_to_string_eq!(reader1, reader2); // reader1.read_to_string() = reader2.read_to_string()

assert_read_to_string_eq_expr!(reader, expr); // reader1.read_to_string() = expr
```
