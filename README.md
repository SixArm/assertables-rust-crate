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
* [Macros](#macros)
  * [Macros for value checking](#macros-for-value-checking)
  * [Macros for function checking](#macros-for-function-checking)
  * [Macros for set checking](#macros-for-set-checking)
  * [Macros for bag checking](#macros-for-bag-checking)
  * [Macros for IO-related checking](#macros-for-io-related-checking)
* [Extras](#extras)
  * [Custom error messages](#custom-error-messages)
  * [Comparison abbreviations](#comparison-abbreviations)
* [Complete list of macros](#complete-list-of-macros)
  * [assert macros](#assert-macros)
  * [assume macros](#assume-macros)
  * [assure macros](#assure-macros)


## Introduction

This Rust crate provides macros for Rust runtime checking.

Example:

```rust
assert_lt!(a, b); // assert a is less than b
```

The macros have a second form where a custom error message can be provided:

```rust
assert_lt!(a, b, "This is a custom error message");
```

The abbreviations are: `eq` equal, `ne` not equal, `lt` less than, `le` less than or equal, `gt` greater than, `ge` greater than or equal.

For advanced programming, the macros that have three forms, described in more detail below: `assert`, `assume`, `assure`.


### assert

Compare values.

* `assert_eq!(a, b)`: assert a == b

* `assert_ne!(a, b)`: assert a !=b

* `assert_lt!(a, b)`: assert a < b

* `assert_le!(a, b)`: assert a <= b

* `assert_gt!(a, b)`: assert a > b

* `assert_ge!(a, b)`: assert a >= b

Example:

```rust
assert_lt!(1, 2); // assert 1 < 2
```

### assert_fn

Compare function return values.

* `assert_fn_eq!(f, a, b)`: assert f(a) == f(b)

* `assert_fn_ne!(f, a, b)`: assert f(a) != f(b)

* `assert_fn_lt!(f, a, b)`: assert f(a) < f(b)

* `assert_fn_le!(f, a, b)`: assert f(a) <= f(b)

* `assert_fn_gt!(f, a, b)`: assert f(a) > f(b)

* `assert_fn_ge!(f, a, b)`: assert f(a) != f(b)

Example:

```rust
assert_fn_eq!(i32::abs, 1, -1); // assert i32::abs(1) == i32::abs(-1)
```

### assert_fn_ok

Compare function return `Result` `Ok` values.

* `assert_fn_ok_eq!(f, a, b)`: assert f(a).unwrap() == f(b).unwrap()

* `assert_fn_ok_ne!(f, a, b)`: assert f(a).unwrap() != f(b).unwrap()

* `assert_fn_ok_lt!(f, a, b)`: assert f(a).unwrap() < f(b).unwrap()

* `assert_fn_ok_le!(f, a, b)`: assert f(a).unwrap() <= f(b).unwrap()

* `assert_fn_ok_gt!(f, a, b)`: assert f(a).unwrap() > f(b).unwrap()

* `assert_fn_ok_ge!(f, a, b)`: assert f(a).unwrap() >- f(b).unwrap()

Example:

```rust
assert_fn_ok_eq!(i32::from_str, "1", "1"); // assert i32::from_str("1").unwrap() == i32::from_str("1").unwrap()
```


### assert_fn_err_string

Compare function return `Result` `Err` `String` values.

* `assert_fn_err_string_eq!(f, a, b)`: assert f(a).unwrap_err().to_string() == f(b).unwrap_err().to_string()

* `assert_fn_err_string_ne!(f, a, b)`: assert f(a).unwrap_err().to_string() != f(b).unwrap_err().to_string()

* `assert_fn_err_string_lt!(f, a, b)`: assert f(a).unwrap_err().to_string() < f(b).unwrap_err().to_string()

* `assert_fn_err_string_le!(f, a, b)`: assert f(a).unwrap_err().to_string() <= f(b).unwrap_err().to_string()

* `assert_fn_err_string_gt!(f, a, b)`: assert f(a).unwrap_err().to_string() > f(b).unwrap_err().to_string()

* `assert_fn_err_string_ge!(f, a, b)`: assert f(a).unwrap_err().to_string() >= f(b).unwrap_err().to_string()

Example:

```rust
assert_fn_err_string_eq!(i32::from_str, "foo", "goo"); // assert i32::from_str("foo").unwrap_err().to_string() == i32::from_str("goo").unwrap_err().to_string()
```


### Macros for set checking

Compare two sets, such as based on two arrays, when the item order does not
matter, and the item count does not matter.

* `assert_set_eq!(a, b)`: assert set a == set b

* `assert_set_ne!(a, b)`: assert set a != set b

Example:

```rust
assert_set_eq!([1, 2], [2, 1]); // assert set {1, 2} == set {2, 1}
```


### Macros for bag checking

Compare two bags, such as based on two arrays, when the item order does not
matter, and the item count does matter.

* `assert_bag_eq(a, b)`: assert bag a == bag b

* `assert_bag_ne(a, b)`: assert bag a != bag b

Example:

```rust
assert_bag_eq!([1, 1, 2], [2, 1, 1]); // assert bag {1, 1, 2} == bag {2, 1, 1}
```


### Macros for IO-related checking

Compare two IO-related values, such as comparison of files, streams, etc. 

* `assert_io!(a)`: assert a is true

* `assert_io_eq!(a, b)`: assert a == b

* `assert_io_ne!(a, b)`: assert a != b

* `assert_io_lt!(a, b)`: assert a < b

* `assert_io_le!(a, b)`: assert a <= b

* `assert_io_gt!(a, b)`: assert a > b

* `assert_io_ge!(a, b)`: assert a >= b


Examples:

```rust
assert_io_lt!(1, 2);
//-> ()
```


## Advanced programming: assert, assume, assure


For advanced programming, the macros have three forms that help with three kinds of checking:

* The `assert` macros return `()` or call `panic!(…)`.

* The `assume` macros return `Result` with `Ok(true)` or `Err(…)`.

* The `assure` macros return `Result` with `Ok(true)` or `Ok(false)`.


### Assert

Example: assert that x is less than y; return `()` or `panic!(message)`.

```rust
assert_lt!(1, 2); // assert 1 is less than 2
//-> ()
```

```rust
assert_lt!(2, 1); // assert 2 is less than 1
//-> panic!("assertion failed: `assert_lt!(left, right)`\n  left: `2`,\n right: `1`")
```


### Assume

Example: assume that x is less than y; return `Result` with `Ok(true)` or `Err(message)`.

```rust
assume_lt!(1, 2);
//-> Ok(true)
```

```rust
assume_lt!(2, 1);
//-> Err("assumption failed: `assume_lt!(left, right)`\n  left: `2`,\n right: `1`")
```


### Assure

Example: assure that x is less than y; return `Result` with `Ok(true)` or `Ok(false)`.

```rust
assure_lt!(1, 2);
//-> Ok(true)
```

```rust
assure_lt!(2, 1);
//-> Ok(false)
```
