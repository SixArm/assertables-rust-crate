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

* `assert…!` returns `()` or calls [`panic!`]

* `assume…!` returns [`Result`] with `Ok(true)` or `Err(…)`

* `assure…!` returns [`Result`] with `Ok(true)` or `Ok(false)` or `Err(…)`

Examples of `assert_lt!`:

```rust
assert_lt!(1, 2);
//-> ()
```

```rust
// assert_lt!(2, 1);
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


### Assert

The `assert…` macros can be useful with Rust testing,
such as with macros that Rust `std` does not provide.

Example:

```rust
fn sum_positive_numbers(a: i32, b: i32) -> Result<i32, String> {
    assert_lt!(0, a);
    assert_lt!(0, b);
    Ok(a + b)
}

sum_positive_numbers(1, 2);
//-> 3

// sum_positive_numbers(-1, -2);
//-> panic!("assertion failed: `(left == right)`\n  left: `0`,\n right: `-1`")
```

### Assume

The `assume…` macros can be useful with the `?` operator, 
such as with early exits in functions.

Example:

```rust
fn sum_positive_numbers(a: i32, b: i32) -> Result<i32, String> {
    assume_lt!(0, a)?;
    assume_lt!(0, b)?;
    Ok(a + b)
}

sum_positive_numbers(1, 2);
//-> Ok(3)

sum_positive_numbers(-1, -2);
//-> Err("assumption failed: `assume_lt(left, right)`\n  left: `0`,\n right: `-1`")
```


### Assure

The `assure…` macros can be useful with chaining,
such as with gate conditions in functions.

Example:

```rust
fn sum_positive_numbers(a: i32, b: i32) -> Result<i32, String> {
    if assure_lt!(0, a).unwrap() && assure_lt!(0, b).unwrap() {
        Ok(a + b)
    } else {
        Err(format!("please use positive numbers"))
    }
}

sum_positive_numbers(1, 2);
//-> Ok(3)

sum_positive_numbers(-1, -2);
//-> Err("must use postive numbers")
```


### Messages

When a macro fails, it generates a failure message with the
values of expressions with their debug representations, such as:

```rust
// assert_lt!(2, 1)
//-> panic!("assertion failed: `(left == right)`\n  left: `2`,\n right: `1`")
```

These macros have a second form where a custom message can be provided,
such as:

```rust
// assert_lt!(2, 1, "my message here");
//-> panic!("my message here")
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

* `assert_eq!(a, b)`: assert `a` is equal to `b`, provided by Rust `std`.

* `assert_ne!(a, b)`: assert `a` is not equal to `b`, provided by Rust `std`.

* `assert_lt!(a, b)`: assert `a` is less than `b`.

* `assert_le!(a, b)`: assert `a` is less than or equal to `b`.

* `assert_gt!(a, b)`: assert `a` is greater than `b`.

* `assert_ge!(a, b)`: assert `a` is greater than or equal to `b`.

`assume…` macros:

* `assume!(a)`: assume `a` is true.

* `assume_eq!(a, b)`: assume `a` is equal to `b`.

* `assume_ne!(a, b)`: assume `a` is not equal to `b`.

* `assume_lt!(a, b)`: assume `a` is less than `b`.

* `assume_le!(a, b)`: assume `a` is less than or equal to `b`.

* `assume_gt!(a, b)`: assume `a` is greater than `b`.

* `assume_ge!(a, b)`: assume `a` is greater than or equal to `b`.

`assure…` macros:

* `assure!(a)`: assure `a` is true.

* `assure_eq!(a, b)`: assure `a` is equal to `b`.

* `assure_ne!(a, b)`: assure `a` is not equal to `b`.

* `assure_lt!(a, b)`: assure `a` is less than `b`.

* `assure_le!(a, b)`: assure `a` is less than or equal to `b`.

* `assure_gt!(a, b)`: assure `a` is greater than `b`.

* `assure_ge!(a, b)`: assure `a` is greater than or equal to `b`.


### Macros for set checking

These macros help with comparison of set parameters,
such as two arrays or two vectors. where the item order 
does not matter, and the item count does not matter.

Examples:

```rust
let x = assume_set_eq!([1, 2], [2, 1]);
//-> Ok(true)
```

```rust
let x = assume_set_eq!([1, 2], [3, 4]);
//-> Err("assertion failed: `assert_set_eq(left, right)`\n  left: `[1, 2]`\n right: `[3, 4]`")
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

* `assert_io_eq!(a, b)`: assert `a` is equal to `b`.

* `assert_io_ne!(a, b)`: assert `a` is not equal to `b`.

* `assert_io_lt!(a, b)`: assert `a` is less than `b`.

* `assert_io_le!(a, b)`: assert `a` is less than or equal to `b`.

* `assert_io_gt!(a, b)`: assert `a` is greater than `b`.

* `assert_io_ge!(a, b)`: assert `a` is greater than or equal to `b`.

`assume_io…` macros:

* `assume_io!(a)`: assume `a` is true.

* `assume_io_eq!(a, b)`: assume `a` is equal to `b`.

* `assume_io_ne!(a, b)`: assume `a` is not equal to `b`.

* `assume_io_lt!(a, b)`: assume `a` is less than `b`.

* `assume_io_le!(a, b)`: assume `a` is less than or equal to `b`.

* `assume_io_gt!(a, b)`: assume `a` is greater than `b`.

* `assume_io_ge!(a, b)`: assume `a` is greater than or equal to `b`.

`assure_io…` macros:

* `assure_io!(a)`: assure `a` is true.

* `assure_io_eq!(a, b)`: assure `a` is equal to `b`.

* `assure_io_ne!(a, b)`: assure `a` is not equal to `b`.

* `assure_io_lt!(a, b)`: assure `a` is less than `b`.

* `assure_io_le!(a, b)`: assure `a` is less than or equal to `b`.

* `assure_io_gt!(a, b)`: assure `a` is greater than `b`.

* `assure_io_ge!(a, b)`: assure `a` is greater than or equal to `b`.
