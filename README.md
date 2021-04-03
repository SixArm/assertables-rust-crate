# Assure: macros for Rust runtime checking

This Rust crate provides the macro `assure!` and related macros.

These are intentionally similar to the macro `assert!` and related macros.

Crate: [https://crates.io/crates/assure](https://crates.io/crates/assure)

Docs: [https://docs.rs/assure/](https://docs.rs/assure/)

Repo: [https://github.com/joelparkerhenderson/assure-rust-crate/](https://github.com/joelparkerhenderson/assure-rust-crate/)


## Introduction

The `assure` macros work like this:

* `assure!(x)` returns `Result` with `Ok(x)` or `Err("assure")`.

* `assure_eq(x, y)` returns `Result` with `Ok(x)` or `Err("assure_eq left:1 right:2"))`.

For comparison `assert` macros work like this:

* `assert!(x)` returns successfully or calls `panic!`.

* `assert_eq!(x, y)` returns successfully or calls `panic!`.


## Return Ok or Err

The `assure` macros return `Result` with either:

* `Ok(…)` with the leftmost macro argument.

* `Err(…)` with a generated error message intended for diagnostics.

Example of `Ok`:

```rust
let a = 1;
let b = 1;
assure_eq!(a, b);
//-> Ok(a)
```

Example of `Err`:

```rust
let a = 1;
let b = 2;
assure_eq!(a, b);
//-> Err("assure_eq left:1 right:2")
```


## Usage

The `assure` macros can useful for checking with the `?` operator.

This example function uses the `assure_gt!` macro, which means assure greater than:

```rust
fn sum_positive_numbers(a: i32, b: i32) -> Result<i32, String> {
    assure_lt!(0, a)?;
    assure_lt!(0, b)?;
    Ok(a + b)
}
```

Example of `Ok`:

```rust
sum_positive_numbers(1, 2);
//-> Ok(3)
```

Example of `Err`:

```rust
sum_positive_numbers(1, -2);
//-> Err("assure_gt left:-2 right:0)
```


## Custom error messages

The `assure` macros generate a defult diagnostic error message such as:

* `assure_eq!(1, 2)` returns `Err("assure_eq left:1 right:2")`.

The macros have a second form, where a custom error message can be provided
 as the last argument:

* `assure_eq!(1, 2, "message")` returns `Err("message")`.

Example error message:

```rust
fn sum_positive_numbers(a: i32, b: i32) -> Result<i32, String> {
    assure_gt!(a, 0, "param 1 must be postive but is {}", a)?;
    assure_gt!(b, 0, "param 2 must be postive but is {}", b)?;
    Ok(a + b)
}
```


## Macros for simple values

Macro for truth checking:

* `assure!(a)`: assure `a` is true.

Macros for value comparison:

* `assure_eq!(a, b)`: assure `a` is equal to `b`.

* `assure_ne!(a, b)`: assure `a` is not equal to `b`.

* `assure_lt!(a, b)`: assure `a` is less than `b`.

* `assure_le!(a, b)`: assure `a` is less than or equal to `b`.

* `assure_gt!(a, b)`: assure `a` is greater than `b`.

* `assure_ge!(a, b)`: assure `a` is greater than or equal to `b`.


## Macros for set checking

The `assure_set…!` macros help with comparison of set parameters, such as two arrays or two vectors. where the item order does not matter, and the item count does not matter.

* `assure_set_eq(a, b)`: assure the set `a` is equal to the set `b`.

* `assure_set_ne(a, b)`: assure the set `a` is not equal to the set `b`.

Example of `Ok`:

```rust
let a = [1, 2];
let b = [2, 1];
assure_set_eq!(&a, &b);
//-> Ok(&a)
```

Example of `Err`:

```rust
let a = [1, 2];
let b = [3, 4];
assure_set_eq!(&a, &b);
//-> Err("assure_set_eq left:{1, 2} right:{3, 4}")
```


## Macros for bag checking

The `assure_bag…!` macros help with comparison of bag parameters, such as comparison of two arrays or two vectors, where the item order does not matter, and the item count does matter.

* `assure_bag_eq(a, b)`: assure the bag `a` is equal to the bag `b`.

* `assure_bag_ne(a, b)`: assure the bag `a` is not equal to the bag `b`.

Example of `Ok`:

```rust
let a = [1, 1];
let b = [1, 1];
assure_set_eq!(&a, &b);
//-> Ok(&a)
```

Example of `Err`:

```rust
let a = [1, 1];
let b = [1, 1, 1];
assure_set_eq!(&a, &b);
//-> Err("assure_bag_eq left:{1: 2} right:{1: 3}")
```


## Macros for IO-related checking

The `assure_io…!` macros help with IO-related checking, such as comparison of files, streams, etc. These macros return a `Result` with `Ok(true)` or `Err(std::io::Error::new(std::io::ErrorKind::InvalidInput, message))`.

Macro for truth checking:

* `assure_io!(a)`: assure `a` is true.

Macros for value comparison:

* `assure_io_eq!(a, b)`: assure `a` is equal to `b`.

* `assure_io_ne!(a, b)`: assure `a` is not equal to `b`.

* `assure_io_lt!(a, b)`: assure `a` is less than `b`.

* `assure_io_le!(a, b)`: assure `a` is less than or equal to `b`.

* `assure_io_gt!(a, b)`: assure `a` is greater than `b`.

* `assure_io_ge!(a, b)`: assure `a` is greater than or equal to `b`.

Example:

```rust
assume_io_lt!(1, 2);
//-> Ok(true)

assure_io_lt!(2, 1);
//-> Err(
//       std::io::Error::new(
//           std::io::ErrorKind::InvalidInput, 
//           "assure_io_lt left:2 right:1"
//       )
//   )
```
