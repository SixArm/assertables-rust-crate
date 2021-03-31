# Assure: macros for Rust runtime checks and error handling

This Rust crate provides the macro `assure!` and its friends. These are similar to the macro `assert!` and its friends.

* `assure!(condition)` will return `Result` with `Ok(true)` or `Err(message)`.

* `assert!(condition)` will return successfully or will call `panic!`.


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


## Macros for iterator set comparison

These macros help with order-independent comparison of iterator set parameters, such as comparison of two arrays, or two vectors.

* `assure_set_eq(a, b)`: assure the set `a` is equal to the set `b`.

* `assure_set_ne(a, b)`: assure the set `a` is not equal to the set `b`.


## Macros for IO-related checking

These macros help with IO-related checking, such as comparison of files, streams, etc. These macros return a `Result` with `Ok(true)` or `Err(std::io::Error(…))`.

Macro for truth checking:

* `assure_io!(a)`: assure `a` is true.

Macros for value comparison:

* `assure_io_eq!(a, b)`: assure `a` is equal to `b`.

* `assure_io_ne!(a, b)`: assure `a` is not equal to `b`.
