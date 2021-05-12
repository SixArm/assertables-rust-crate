# Assertables: Rust crate of macros assert, assume, assure

This `assertables` Rust crate provides macros 
`assert…!`, `assume…!`, `assure…!`, all for runtime 
reliability checking, and all described below. By SixArm.com.

Crate: 
[https://crates.io/crates/assertables](https://crates.io/crates/assure)

Docs: 
[https://docs.rs/assertables/](https://docs.rs/assure/)

Repo: 
[https://github.com/sixarm/assertables-rust-crate/](https://github.com/sixarm/assertables-rust-crate/)

Contents:

* [Introduction](#introduction)
  * [Assert](#assert)
  * [Assume](#assume)
  * [Assure](#assure)
* [Macros list](#macros-list)
  * [Macros for values](#macros-for-values)
  * [Macros for set checking](#macros-for-set-checking)
  * [Macros for bag checking](#macros-for-bag-checking)
  * [Macros for IO-related checking](#macros-for-io-related-checking)


## Introduction

This Rust crate provides macros for Rust runtime checking:

* The `assert` macros return `()` or call [`panic!(…)`]

* The `assume` macros return `Result` with `Ok(true)` or `Err(…)`

* The `assure` macros return `Result` with `Ok(true)` or `Ok(false)` or exceptional `Err(…)`.


### Assert

Example: assert that x is less than y; return `()` or call `panic!(message)`.

```rust
assert_lt!(1, 2);
//-> ()
```

```rust
assert_lt!(2, 1);
//-> panic!("assertion failed: `assert_lt(left, right)`\n  left: `2`,\n right: `1`")
```


### Assume

Example: assume that x is less than y; return `Result` with `Ok(true)` or `Err(message)`.

```rust
assume_lt!(1, 2);
//-> Ok(true)
```

```rust
assume_lt!(2, 1);
//-> Err("assumption failed: `assume_lt(left, right)`\n  left: `2`,\n right: `1`")
```


### Assure

Example: assure that x is less than y; return `Result` with `Ok(true)` or `Ok(false)` or exceptional `Err(message)`.

```rust
assure_lt!(1, 2);
//-> Ok(true)
```

```rust
assure_lt!(2, 1);
//-> Ok(false)
```


## Macros


### Macros for value checking

To compare values:

```rust
assert_lt!(1, 2); // check that 1 is less than 2
```


### Macros for function checking

To compare function return values:

```rust
assert_fn_eq!(abs, 1, -1); // abs(1) == abs(-1)
```

To compare function `Result` `Ok()` values:

```rust
assert_fn_ok_eq!(::i32::from_str, "1", "1"); // ::i32::from_str("1").unwrap() == ::i32::from_str("1").unwrap()
```

Test a function `Result` `Err()` strings:

```rust
assert_fn_err_string_eq!(::i32::from_str, "foo", "goo"); // ::i32::from_str("foo").unwrap_err().to_string() == ::i32::from_str("goo").unwrap_err().to_string()
```


### Macros for set checking

To compare sets, such as two arrays, where the item order does not matter, and
the item count does not matter.

Examples:

```rust
assert_set_eq!([1, 2], [2, 1]);
//-> ()
```


### Macros for bag checking

Compare bags, such as two arrays, where the item order does not matter, and the
item count does matter.

Examples:

```rust
assert_bag_eq!([1, 1, 2], [2, 1, 1]);
//-> ()
```



### Macros for IO-related checking

These macros help with IO-related checking, such as comparison of files, streams, etc. These macros return a `Result` with `Ok(true)` or `Err(std::io::Error::new(std::io::ErrorKind::InvalidInput, message))`.

Examples:

```rust
assert_io_lt!(1, 2);
//-> ()
```

```rust
assert_io_lt!(2, 1);
//-> Err(
//       std::io::Error::new(
//           std::io::ErrorKind::InvalidInput, 
//           "assumption failed: `assume_io_lt(left, right)`\n  left: `2`\n right: `1`")]
//       )
//   )
```


## Extras 


### Custom error messages

The macros have a second form where a custom error message can be provided.


### Comparison abbreviations

The comparison macros use abbreviations such as `eq` (equals), `ne` (not equals), `lt` (less than), `le` (less than or equal to), `gt` (greater than), `ge` (greater than or equals).


## Complete list of macros


### assert…

* `assert!(a)`: assert `a` is true, provided by Rust `std`.

* `assert_eq!(a, b)`: assert `a == b`, provided by Rust `std`.

* `assert_ne!(a, b)`: assert `a != b`, provided by Rust `std`.

* `assert_lt!(a, b)`: assert `a < b`.

* `assert_le!(a, b)`: assert `a <= b`.

* `assert_gt!(a, b)`: assert `a > b`.

* `assert_ge!(a, b)`: assert `a >= b`.


### assert_fn…

* `assert_fn_eq!(f, a, b)`: assert `f(a) == f(b)`.

* `assert_fn_ne!(f, a, b)`: assert `f(a) != f(b)`.

* `assert_fn_lt!(f, a, b)`: assert `f(a) < f(b)`.

* `assert_fn_le!(f, a, b)`: assert `f(a) <= f(b)`.

* `assert_fn_gt!(f, a, b)`: assert `f(a) > f(b)`.

* `assert_fn_ge!(f, a, b)`: assert `f(a) >= f(b)`.


### assert_fn_ok…

* `assert_fn_ok_eq!(f, a, b)`: assert `f(a).unwrap() == f(b).unwrap()`.

* `assert_fn_ok_ne!(f, a, b)`: assert `f(a).unwrap() != f(b).unwrap()`.

* `assert_fn_ok_lt!(f, a, b)`: assert `f(a).unwrap() < f(b).unwrap()`.

* `assert_fn_ok_le!(f, a, b)`: assert `f(a).unwrap() <= f(b).unwrap()`.

* `assert_fn_ok_gt!(f, a, b)`: assert `f(a).unwrap() > f(b).unwrap()`.

* `assert_fn_ok_ge!(f, a, b)`: assert `f(a).unwrap() >= f(b).unwrap()`.


### assert_fn_err_string…

* `assert_fn_err_string_eq!(f, a, b)`: assert `f(a).unwrap_err().to_string() == f(b).unwrap_err().to_string()`.

* `assert_fn_err_string_ne!(f, a, b)`: assert `f(a).unwrap_err().to_string() != f(b).unwrap_err().to_string()`.

* `assert_fn_err_string_lt!(f, a, b)`: assert `f(a).unwrap_err().to_string() < f(b).unwrap_err().to_string()`.

* `assert_fn_err_string_le!(f, a, b)`: assert `f(a).unwrap_err().to_string() <= f(b).unwrap_err().to_string()`.

* `assert_fn_err_string_gt!(f, a, b)`: assert `f(a).unwrap_err().to_string() > f(b).unwrap_err().to_string()`.

* `assert_fn_err_string_ge!(f, a, b)`: assert `f(a).unwrap_err().to_string() >= f(b).unwrap_err().to_string()`.


### assert_set…

* `assert_set_eq!(a, b)`: assert the set `a` is equal to the set `b`.

* `assert_set_ne!(a, b)`: assert the set `a` is not equal to the set `b`.


### assert_bag…

* `assert_bag_eq(a, b)`: assert the bag `a` is equal to the bag `b`.

* `assert_bag_ne(a, b)`: assert the bag `a` is not equal to the bag `b`.


### assert_io…

* `assert_io!(a)`: assert `a` is true.

* `assert_io_eq!(a, b)`: assert `a == b`.

* `assert_io_ne!(a, b)`: assert `a != b`.

* `assert_io_lt!(a, b)`: assert `a < b`.

* `assert_io_le!(a, b)`: assert `a <= b`.

* `assert_io_gt!(a, b)`: assert `a > b`.

* `assert_io_ge!(a, b)`: assert `a >= b`.


### assume…

* `assume!(a)`: assume `a` is true.

* `assume_eq!(a, b)`: assume `a == b`.

* `assume_ne!(a, b)`: assume `a != b`.

* `assume_lt!(a, b)`: assume `a < b`.

* `assume_le!(a, b)`: assume `a <= b`.

* `assume_gt!(a, b)`: assume `a > b`.

* `assume_ge!(a, b)`: assume `a >= b`.


### assume_fn…

* `assume_fn_eq!(f, a, b)`: assume `f(a) == f(b)`.

* `assume_fn_ne!(f, a, b)`: assume `f(a) != f(b)`.

* `assume_fn_lt!(f, a, b)`: assume `f(a) < f(b)`.

* `assume_fn_le!(f, a, b)`: assume `f(a) <= f(b)`.

* `assume_fn_gt!(f, a, b)`: assume `f(a) > f(b)`.

* `assume_fn_ge!(f, a, b)`: assume `f(a) >= f(b)`.


### assume_fn_ok…

* `assume_fn_ok_eq!(f, a, b)`: assume `f(a).unwrap() == f(b).unwrap()`.

* `assume_fn_ok_ne!(f, a, b)`: assume `f(a).unwrap() != f(b).unwrap()`.

* `assume_fn_ok_lt!(f, a, b)`: assume `f(a).unwrap() < f(b).unwrap()`.

* `assume_fn_ok_le!(f, a, b)`: assume `f(a).unwrap() <= f(b).unwrap()`.

* `assume_fn_ok_gt!(f, a, b)`: assume `f(a).unwrap() > f(b).unwrap()`.

* `assume_fn_ok_ge!(f, a, b)`: assume `f(a).unwrap() >= f(b).unwrap()`.


### assume_fn_err_string…

* `assume_fn_err_string_eq!(f, a, b)`: assume `f(a).unwrap_err().to_string() == f(b).unwrap_err().to_string()`.

* `assume_fn_err_string_ne!(f, a, b)`: assume `f(a).unwrap_err().to_string() != f(b).unwrap_err().to_string()`.

* `assume_fn_err_string_lt!(f, a, b)`: assume `f(a).unwrap_err().to_string() < f(b).unwrap_err().to_string()`.

* `assume_fn_err_string_le!(f, a, b)`: assume `f(a).unwrap_err().to_string() <= f(b).unwrap_err().to_string()`.

* `assume_fn_err_string_gt!(f, a, b)`: assume `f(a).unwrap_err().to_string() > f(b).unwrap_err().to_string()`.

* `assume_fn_err_string_ge!(f, a, b)`: assume `f(a).unwrap_err().to_string() >= f(b).unwrap_err().to_string()`.


### assume_set…

* `assume_set_eq!(a, b)`: assume the set `a` is equal to the set `b`.

* `assume_set_ne!(a, b)`: assume the set `a` is not equal to the set `b`.


### assume_bag…

* `assume_bag_eq!(a, b)`: assume the bag `a` is equal to the bag `b`.

* `assume_bag_ne!(a, b)`: assume the bag `a` is not equal to the bag `b`.


### assume_io…

* `assume_io!(a)`: assume `a` is true.

* `assume_io_eq!(a, b)`: assume `a == b`.

* `assume_io_ne!(a, b)`: assume `a != b`.

* `assume_io_lt!(a, b)`: assume `a < b`.

* `assume_io_le!(a, b)`: assume `a <= b`.

* `assume_io_gt!(a, b)`: assume `a > b`.

* `assume_io_ge!(a, b)`: assume `a >= b`.


### assure…

* `assure!(a)`: assure `a` is true.

* `assure_eq!(a, b)`: assure `a == b`.

* `assure_ne!(a, b)`: assure `a != b`.

* `assure_lt!(a, b)`: assure `a < b`.

* `assure_le!(a, b)`: assure `a <= b`.

* `assure_gt!(a, b)`: assure `a > b`.

* `assure_ge!(a, b)`: assure `a >= b`.


### assure_fn…

* `assure_fn_eq!(f, a, b)`: assure `f(a) == f(b)`.

* `assure_fn_ne!(f, a, b)`: assure `f(a) != f(b)`.

* `assure_fn_lt!(f, a, b)`: assure `f(a) < f(b)`.

* `assure_fn_le!(f, a, b)`: assure `f(a) <= f(b)`.

* `assure_fn_gt!(f, a, b)`: assure `f(a) > f(b)`.

* `assure_fn_ge!(f, a, b)`: assure `f(a) >= f(b)`.


### assure_fn_ok…

* `assure_fn_ok_eq!(f, a, b)`: assure `f(a).unwrap() == f(b).unwrap()`.

* `assure_fn_ok_ne!(f, a, b)`: assure `f(a).unwrap() != f(b).unwrap()`.

* `assure_fn_ok_lt!(f, a, b)`: assure `f(a).unwrap() < f(b).unwrap()`.

* `assure_fn_ok_le!(f, a, b)`: assure `f(a).unwrap() <= f(b).unwrap()`.

* `assure_fn_ok_gt!(f, a, b)`: assure `f(a).unwrap() > f(b).unwrap()`.

* `assure_fn_ok_ge!(f, a, b)`: assure `f(a).unwrap() >= f(b).unwrap()`.


### assure_fn_err_string…

* `assure_fn_err_string_eq!(f, a, b)`: assure `f(a).unwrap_err().to_string() == f(b).unwrap_err().to_string()`.

* `assure_fn_err_string_ne!(f, a, b)`: assure `f(a).unwrap_err().to_string() != f(b).unwrap_err().to_string()`.

* `assure_fn_err_string_lt!(f, a, b)`: assure `f(a).unwrap_err().to_string() < f(b).unwrap_err().to_string()`.

* `assure_fn_err_string_le!(f, a, b)`: assure `f(a).unwrap_err().to_string() <= f(b).unwrap_err().to_string()`.

* `assure_fn_err_string_gt!(f, a, b)`: assure `f(a).unwrap_err().to_string() > f(b).unwrap_err().to_string()`.

* `assure_fn_err_string_ge!(f, a, b)`: assure `f(a).unwrap_err().to_string() >= f(b).unwrap_err().to_string()`.


### assure_set…

* `assure_set_eq!(a, b)`: assure the set `a` is equal to the set `b`.

* `assure_set_ne!(a, b)`: assure the set `a` is not equal to the set `b`.


### assure_bag…

* `assure_bag_eq!(a, b)`: assure the bag `a` is equal to the bag `b`.

* `assure_bag_ne!(a, b)`: assure the bag `a` is not equal to the bag `b`.


### assure_io…

* `assure_io!(a)`: assure `a` is true.

* `assure_io_eq!(a, b)`: assure `a == b`.

* `assure_io_ne!(a, b)`: assure `a != b`.

* `assure_io_lt!(a, b)`: assure `a < b`.

* `assure_io_le!(a, b)`: assure `a <= b`.

* `assure_io_gt!(a, b)`: assure `a > b`.

* `assure_io_ge!(a, b)`: assure `a >= b`.
