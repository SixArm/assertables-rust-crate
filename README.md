# Assure: macros for Rust runtime checking

This Rust crate provides the macro `assure!` and related macros.

These are intentionally similar to the macro `assert!` and related macros.

Available via [https://crates.io/crates/assure](https://crates.io/crates/assure)


## Introduction

The `assure` macros work like this:

* `assure!(x)` will return `Result` with `Ok(x)` or `Err(message)`.

For comparison `assert` macros work like this:

* `assert!(x)` will return successfully or will call `panic!`.


## Error messages

The `assure` macros will generate a diagnostic error message such as:

* `assure_eq!(1, 2)` will return `Err("assure_eq left:1 right:2")`.

If you prefer, you can provide your own error message as the final argument:

* `assure_eq!(1, 2, "whatever")` will return `Err("whatever")`.

This behavior is intentionally similar to `assert` macros.


## Return Ok or Err

The `assure…!` macros will return `Result` with either:

* `Ok(…)` with the leftmost macro argument.

* `Err(…)` with a generated error message intended for diagnostics.

Example of `Ok`:

```rust
let a = 1;
let b = 1;
assure_eq!(a, b) 
-> Ok(a)
```

Example of `Err`:

```rust
let a = 1;
let b = 2;
assure_eq!(a, b)
-> Err("assure_eq left:1 right:2")
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



## Macros for bag checking

These macros help with comparison of bag parameters, such as comparison of two arrays or two vectors, where the item order does not matter, and the item count does matter.

* `assure_bag_eq(a, b)`: assure the bag `a` is equal to the bag `b`.

* `assure_bag_ne(a, b)`: assure the bag `a` is not equal to the bag `b`.

Example of `Ok`:

```rust
let a = [1, 1];
let b = [1, 1];
assure_set_eq!(&a, &b) 
-> Ok(&a)
```

Example of `Err`:

```rust
let a = [1, 1];
let b = [1, 1, 1];
assure_set_eq!(&a, &b) 
-> Err("assure_bag_eq left:{1: 2} right:{1: 3}")
```


## Macros for set checking

These macros help with comparison of set parameters, such as comparison of two arrays or two vectors. where the item order does not matter, and the item count does not matter.

* `assure_set_eq(a, b)`: assure the set `a` is equal to the set `b`.

* `assure_set_ne(a, b)`: assure the set `a` is not equal to the set `b`.

Example of `Ok`:

```rust
let a = [1, 2];
let b = [2, 1];
assure_set_eq!(&a, &b) 
-> Ok(&a)
```

Example of `Err`:

```rust
let a = [1, 2];
let b = [3, 4];
assure_set_eq!(&a, &b) 
-> Err("assure_set_eq left:{1, 2} right:{3, 4}")
```


## Macros for IO-related checking

These macros help with IO-related checking, such as comparison of files, streams, etc. These macros return a `Result` with `Ok(true)` or `Err(std::io::Error::new(std::io::ErrorKind::InvalidInput, message))`.

Macro for truth checking:

* `assure_io!(a)`: assure `a` is true.

Macros for value comparison:

* `assure_io_eq!(a, b)`: assure `a` is equal to `b`.

* `assure_io_ne!(a, b)`: assure `a` is not equal to `b`.

* `assure_io_lt!(a, b)`: assure `a` is less than `b`.

* `assure_io_le!(a, b)`: assure `a` is less than or equal to `b`.

* `assure_io_gt!(a, b)`: assure `a` is greater than `b`.

* `assure_io_ge!(a, b)`: assure `a` is greater than or equal to `b`.

Example of `Ok`:

```rust
let a = 1;
let b = 1;
assure_io_eq!(a, b) 
-> Ok(a)
```

Example of `Err`:

```rust
let a = 1;
let b = 2;
assure_io_eq!(a, b) 
-> Err(std::io::Error::new(std::io::ErrorKind::InvalidInput, "assure_io_eq left:1 right:2"))
```
