# Assertables: Rust crate of macros assert, assume, assure

This `assertables` Rust crate provides macros 
`assert…!`, `assume…!`, `assure…!`, all for runtime 
reliability checking, and all described below. By SixArm.com.

Crate: [https://crates.io/crates/assure](https://crates.io/crates/assure)

Docs: [https://docs.rs/assure/](https://docs.rs/assure/)

Repo: [https://github.com/joelparkerhenderson/assure-rust-crate/](https://github.com/joelparkerhenderson/assure-rust-crate/)

Contents:

* [Introduction](#introduction)
  * [Assert](#assert)
  * [Assume](#assume)
  * [Assure](#assure)
  * [Messages](#messages)
* [Macros list](#macros-list)
  * [Macros for values](#macros-for-values)
  * [Macros for set checking](#macros-for-set-checking)
  * [Macros for bag checking](#macros-for-bag-checking)
  * [Macros for IO-related checking](#macros-for-io-related-checking)


## Introduction

This Rust crate provides macros for Rust runtime checking,
and each macro comes in three flavors:

* `assert…!` returns `()` or calls [`panic!(…)`]

* `assume…!` returns [`Result`] with `Ok(true)` or `Err(…)`

* `assure…!` returns [`Result`] with `Ok(true)` or `Ok(false)` or `Err(…)`

Examples of `assert_lt!` meaning "assert less than":

```rust
assert_lt!(1, 2);
//-> ()
```

```rust
assert_lt!(2, 1);
//-> panic!("assertion failed: `assert_lt(left, right)`\n  left: `2`,\n right: `1`")
```

Examples of `assume_lt!`:

```rust
let x = assume_lt!(1, 2);
//-> Ok(true)
```

```rust
let x = assume_lt!(2, 1);
//-> Err("assumption failed: `assume_lt(left, right)`\n  left: `2`,\n right: `1`")
```

Examples of `assure_lt!`:

```rust
let x = assure_lt!(1, 2);
//-> Ok(true)
```

```rust
let x = assure_lt!(2, 1);
//-> Ok(false)
```

### Messages

When a macro fails, it generates a failure message with the
values of expressions with their debug representations, such as:

```rust
// assert_lt!(2, 1)
//-> panic!("assertion failed: `assert_lt(left, right)`\n  left: `2`,\n right: `1`")
```

These macros have a second form where a custom message can be provided,
such as:

```rust
// assert_lt!(2, 1, "my message here");
//-> panic!("my message here")
```


### Assert

The `assert…` macros can be useful with Rust testing,
such as with macros that Rust `std` does not provide.

Example:

```rust
fn duration(a: i32, b: i32) -> i32 {
    assert_le!(a, b);
    b - a
}

duration(1, 3);
//-> 2

// duration(3, 1);
//-> panic!("assertion failed: `assert_le(left, right)`\n  left: `3`,\n right: `1`")
```

### Assume

The `assume…` macros can be useful with the `?` operator, 
such as with early exits in functions.

Example:

```rust
fn duration(a: i32, b: i32) -> Result<i32, String> {
    if assert_le!(a, b).is_ok() {
        Ok(b - a)
    } else {
        Err("message")
    }
}

duration(1, 3);
//-> Ok(2)

duration(3, 1);
//-> Err("assumption failed: `assume_le(left, right)`\n  left: `3`,\n right: `1`")
```


### Assure

The `assure…` macros can be useful with chaining,
such as with gate conditions in functions.

Example:

```rust
fn duration(a: i32, b: i32) -> Result<i32, String> {
    if assure_ge!(a, 0).unwrap() {
        Ok(b - a)
    } else {
        Err("message")
    }
}

duration(1, 3);
//-> Ok(2)

duration(3, 1);
//-> Err("message")
```


## Macros list


### Macros for values

Examples:

```rust
let x = assume_lt!(1, 2);
//-> Ok(true)
```

```rust
let x = assume_lt!(2, 1);
//-> Err("assumption failed: `assert_lt(left, right)`\n  left: `2`\n right: `1`")
```

`assert…` macros:

* `assert!(a)`: assure `a` is true, provided by Rust `std`.

* `assert_eq!(a, b)`: assert `a == b`, provided by Rust `std`.

* `assert_ne!(a, b)`: assert `a != b`, provided by Rust `std`.

* `assert_lt!(a, b)`: assert `a < b`.

* `assert_le!(a, b)`: assert `a <= b`.

* `assert_gt!(a, b)`: assert `a > b`.

* `assert_ge!(a, b)`: assert `a >= b`.

`assume…` macros:

* `assume!(a)`: assume `a` is true.

* `assume_eq!(a, b)`: assume `a == b`.

* `assume_ne!(a, b)`: assume `a != b`.

* `assume_lt!(a, b)`: assume `a < b`.

* `assume_le!(a, b)`: assume `a <= b`.

* `assume_gt!(a, b)`: assume `a > b`.

* `assume_ge!(a, b)`: assume `a >= b`.

`assure…` macros:

* `assure!(a)`: assure `a` is true.

* `assure_eq!(a, b)`: assure `a == b`.

* `assure_ne!(a, b)`: assure `a != b`.

* `assure_lt!(a, b)`: assure `a < b`.

* `assure_le!(a, b)`: assure `a <= b`.

* `assure_gt!(a, b)`: assure `a > b`.

* `assure_ge!(a, b)`: assure `a >= b`.


### Macros for function checking

These macros check function outputs.

Test a function return value by comparing `f(x)`:

```rust
assert_fn_eq!(abs, 1, -1);
```

Test a function result `ok` by comparing `f(x).unwrap()`:

```rust
assert_fn_ok_eq!(i32::from_str, "1", "1");
```

Test a function result `err` by comparing `f(x).unwrap_err().to_string()"`:

```rust
assert_fn_err_string_eq!(i32::from_str, "foo", "foo");
```

`assert_fn_…` macros:

* `assert_fn_eq!(f, a, b)`: assert `f(a) == f(b)`.

* `assert_fn_ne!(f, a, b)`: assert `f(a) != f(b)`.

* `assert_fn_lt!(f, a, b)`: assert `f(a) < f(b)`.

* `assert_fn_le!(f, a, b)`: assert `f(a) <= f(b)`.

* `assert_fn_gt!(f, a, b)`: assert `f(a) > f(b)`.

* `assert_fn_ge!(f, a, b)`: assert `f(a) >= f(b)`.

`assert_fn_ok_…` macros:

* `assert_fn_ok_eq!(f, a, b)`: assert `f(a).unwrap() == f(b).unwrap()`.

* `assert_fn_ok_ne!(f, a, b)`: assert `f(a).unwrap() != f(b).unwrap()`.

* `assert_fn_ok_lt!(f, a, b)`: assert `f(a).unwrap() < f(b).unwrap()`.

* `assert_fn_ok_le!(f, a, b)`: assert `f(a).unwrap() <= f(b).unwrap()`.

* `assert_fn_ok_gt!(f, a, b)`: assert `f(a).unwrap() > f(b).unwrap()`.

* `assert_fn_ok_ge!(f, a, b)`: assert `f(a).unwrap() >= f(b).unwrap()`.

`assert_fn_err_string_…` macros:

* `assert_fn_err_string_eq!(f, a, b)`: assert `f(a).unwrap_err().to_string() == f(b).unwrap_err().to_string()`.

* `assert_fn_err_string_ne!(f, a, b)`: assert `f(a).unwrap_err().to_string() != f(b).unwrap_err().to_string()`.

* `assert_fn_err_string_lt!(f, a, b)`: assert `f(a).unwrap_err().to_string() < f(b).unwrap_err().to_string()`.

* `assert_fn_err_string_le!(f, a, b)`: assert `f(a).unwrap_err().to_string() <= f(b).unwrap_err().to_string()`.

* `assert_fn_err_string_gt!(f, a, b)`: assert `f(a).unwrap_err().to_string() > f(b).unwrap_err().to_string()`.

* `assert_fn_err_string_ge!(f, a, b)`: assert `f(a).unwrap_err().to_string() >= f(b).unwrap_err().to_string()`.

`assume_fn_…` macros:

* `assume_fn_eq!(f, a, b)`: assume `f(a) == f(b)`.

* `assume_fn_ne!(f, a, b)`: assume `f(a) != f(b)`.

* `assume_fn_lt!(f, a, b)`: assume `f(a) < f(b)`.

* `assume_fn_le!(f, a, b)`: assume `f(a) <= f(b)`.

* `assume_fn_gt!(f, a, b)`: assume `f(a) > f(b)`.

* `assume_fn_ge!(f, a, b)`: assume `f(a) >= f(b)`.

`assume_fn_ok_…` macros:

* `assume_fn_ok_eq!(f, a, b)`: assume `f(a).unwrap() == f(b).unwrap()`.

* `assume_fn_ok_ne!(f, a, b)`: assume `f(a).unwrap() != f(b).unwrap()`.

* `assume_fn_ok_lt!(f, a, b)`: assume `f(a).unwrap() < f(b).unwrap()`.

* `assume_fn_ok_le!(f, a, b)`: assume `f(a).unwrap() <= f(b).unwrap()`.

* `assume_fn_ok_gt!(f, a, b)`: assume `f(a).unwrap() > f(b).unwrap()`.

* `assume_fn_ok_ge!(f, a, b)`: assume `f(a).unwrap() >= f(b).unwrap()`.

`assume_fn_err_string_…` macros:

* `assume_fn_err_string_eq!(f, a, b)`: assume `f(a).unwrap_err().to_string() == f(b).unwrap_err().to_string()`.

* `assume_fn_err_string_ne!(f, a, b)`: assume `f(a).unwrap_err().to_string() != f(b).unwrap_err().to_string()`.

* `assume_fn_err_string_lt!(f, a, b)`: assume `f(a).unwrap_err().to_string() < f(b).unwrap_err().to_string()`.

* `assume_fn_err_string_le!(f, a, b)`: assume `f(a).unwrap_err().to_string() <= f(b).unwrap_err().to_string()`.

* `assume_fn_err_string_gt!(f, a, b)`: assume `f(a).unwrap_err().to_string() > f(b).unwrap_err().to_string()`.

* `assume_fn_err_string_ge!(f, a, b)`: assume `f(a).unwrap_err().to_string() >= f(b).unwrap_err().to_string()`.

`assure_fn_…` macros:

* `assure_fn_eq!(f, a, b)`: assure `f(a) == f(b)`.

* `assure_fn_ne!(f, a, b)`: assure `f(a) != f(b)`.

* `assure_fn_lt!(f, a, b)`: assure `f(a) < f(b)`.

* `assure_fn_le!(f, a, b)`: assure `f(a) <= f(b)`.

* `assure_fn_gt!(f, a, b)`: assure `f(a) > f(b)`.

* `assure_fn_ge!(f, a, b)`: assure `f(a) >= f(b)`.

`assure_fn_ok_…` macros:

* `assure_fn_ok_eq!(f, a, b)`: assure `f(a).unwrap() == f(b).unwrap()`.

* `assure_fn_ok_ne!(f, a, b)`: assure `f(a).unwrap() != f(b).unwrap()`.

* `assure_fn_ok_lt!(f, a, b)`: assure `f(a).unwrap() < f(b).unwrap()`.

* `assure_fn_ok_le!(f, a, b)`: assure `f(a).unwrap() <= f(b).unwrap()`.

* `assure_fn_ok_gt!(f, a, b)`: assure `f(a).unwrap() > f(b).unwrap()`.

* `assure_fn_ok_ge!(f, a, b)`: assure `f(a).unwrap() >= f(b).unwrap()`.

`assure_fn_err_string_…` macros:

* `assure_fn_err_string_eq!(f, a, b)`: assure `f(a).unwrap_err().to_string() == f(b).unwrap_err().to_string()`.

* `assure_fn_err_string_ne!(f, a, b)`: assure `f(a).unwrap_err().to_string() != f(b).unwrap_err().to_string()`.

* `assure_fn_err_string_lt!(f, a, b)`: assure `f(a).unwrap_err().to_string() < f(b).unwrap_err().to_string()`.

* `assure_fn_err_string_le!(f, a, b)`: assure `f(a).unwrap_err().to_string() <= f(b).unwrap_err().to_string()`.

* `assure_fn_err_string_gt!(f, a, b)`: assure `f(a).unwrap_err().to_string() > f(b).unwrap_err().to_string()`.

* `assure_fn_err_string_ge!(f, a, b)`: assure `f(a).unwrap_err().to_string() >= f(b).unwrap_err().to_string()`.


### Macros for set checking

These macros help with comparison of set parameters,
such as two arrays or two vectors, where the item order 
does not matter, and the item count does not matter.

Examples:

```rust
let x = assume_set_eq!([1, 2], [2, 1]);
//-> Ok(true)
```

```rust
let x = assume_set_eq!([1, 2], [3, 4]);
//-> Err("assumption failed: `assume_set_eq(left, right)`\n  left: `[1, 2]`\n right: `[3, 4]`")
```

`assert_set…` macros:

* `assert_set_eq!(a, b)`: assert the set `a` is equal to the set `b`.

* `assert_set_ne!(a, b)`: assert the set `a` is not equal to the set `b`.

`assume_set…` macros:

* `assume_set_eq!(a, b)`: assume the set `a` is equal to the set `b`.

* `assume_set_ne!(a, b)`: assume the set `a` is not equal to the set `b`.

`assure_set…` macros:

* `assure_set_eq!(a, b)`: assure the set `a` is equal to the set `b`.

* `assure_set_ne!(a, b)`: assure the set `a` is not equal to the set `b`.


### Macros for bag checking

Thes macros help with comparison of bag parameters, such as comparison of two arrays or two vectors, where the item order does not matter, and the item count does matter.

Examples:

```rust
let x = assume_bag_eq!([1, 1], [1, 1]);
//-> Ok(true)
```

```rust
let x = assume_bag_eq!([1, 1], [1, 1, 1]);
//-> Err("assumption failed: `assume_bag_eq(left, right)`\n  left: `[1, 1]`\n right: `[1, 1, 1]`")]
```

`assert_bag…` macros:

* `assert_bag_eq(a, b)`: assert the bag `a` is equal to the bag `b`.

* `assert_bag_ne(a, b)`: assert the bag `a` is not equal to the bag `b`.

`assume_bag…` macros:

* `assume_bag_eq!(a, b)`: assume the bag `a` is equal to the bag `b`.

* `assume_bag_ne!(a, b)`: assume the bag `a` is not equal to the bag `b`.

`assure_bag…` macros:

* `assure_bag_eq!(a, b)`: assure the bag `a` is equal to the bag `b`.

* `assure_bag_ne!(a, b)`: assure the bag `a` is not equal to the bag `b`.


### Macros for IO-related checking

These macros help with IO-related checking, such as comparison of files, streams, etc. These macros return a `Result` with `Ok(true)` or `Err(std::io::Error::new(std::io::ErrorKind::InvalidInput, message))`.

Examples:

```rust
let x = assume_io_lt!(1, 2);
//-> Ok(true)
```

```rust
let x = assume_io_lt!(2, 1);
//-> Err(
//       std::io::Error::new(
//           std::io::ErrorKind::InvalidInput, 
//           "assumption failed: `assume_io_lt(left, right)`\n  left: `2`\n right: `1`")]
//       )
//   )
```

`assert_io…` macros:

* `assert_io!(a)`: assert `a` is true.

* `assert_io_eq!(a, b)`: assert `a == b`.

* `assert_io_ne!(a, b)`: assert `a != b`.

* `assert_io_lt!(a, b)`: assert `a < b`.

* `assert_io_le!(a, b)`: assert `a <= b`.

* `assert_io_gt!(a, b)`: assert `a > b`.

* `assert_io_ge!(a, b)`: assert `a >= b`.

`assume_io…` macros:

* `assume_io!(a)`: assume `a` is true.

* `assume_io_eq!(a, b)`: assume `a == b`.

* `assume_io_ne!(a, b)`: assume `a != b`.

* `assume_io_lt!(a, b)`: assume `a < b`.

* `assume_io_le!(a, b)`: assume `a <= b`.

* `assume_io_gt!(a, b)`: assume `a > b`.

* `assume_io_ge!(a, b)`: assume `a >= b`.

`assure_io…` macros:

* `assure_io!(a)`: assure `a` is true.

* `assure_io_eq!(a, b)`: assure `a == b`.

* `assure_io_ne!(a, b)`: assure `a != b`.

* `assure_io_lt!(a, b)`: assure `a < b`.

* `assure_io_le!(a, b)`: assure `a <= b`.

* `assure_io_gt!(a, b)`: assure `a > b`.

* `assure_io_ge!(a, b)`: assure `a >= b`.
