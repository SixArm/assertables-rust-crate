# Assure: macros for Rust runtime checking

This Rust crate provides the macro `assure!` and its friends. 

These macros are similar to the macro `assert!` and its friends.

The `assure!` macros work like this:

* `assure!(condition)` will return `Result` with `Ok(true)` or `Err("assure")`.

* `assure!(condition, message)` will return `Result` with `Ok(true)` or `Err(message)`.

Whereas typical `assert!` macros work like this:

* `assert!(condition)` will return successfully or will call `panic!`.

* `assert!(condition, message)` will return successfully or will call `panic!` with the message.


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

Examples:

```rust
assure_eq!(1, 1) 
-> Ok(true)

assure_eq!(1, 2) 
-> Err("assure_eq left:1 right:2")

assure_eq!(1, 2, "message") 
-> Err("message")
```

## Macros for iterator set checking

These macros help with order-independent comparison of iterator set parameters, such as comparison of two arrays, or two vectors.

* `assure_set_eq(a, b)`: assure the set `a` is equal to the set `b`.

* `assure_set_ne(a, b)`: assure the set `a` is not equal to the set `b`.

Examples:

```rust
assure_set_eq!([1, 2], [2, 1]) 
-> Ok(true)

assure_set_eq!([1, 2], [3, 4]) 
-> Err("assure_set_eq left:{1, 2} right:{3, 4}")

assure_eq!([1, 2], [3, 4], "message") 
-> Err("message")
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

```rust
assure_io_eq!(1, 1) 
-> Ok(true)

assure_io_eq!(1, 2) 
-> Err(std::io::Error::new(std::io::ErrorKind::InvalidInput, "assure_io_eq left:1 right:2"))

assure_io_eq!(1, 2, "message")
-> Err(std::io::Error:new(std::io::ErrorKind::InvalidInput, "message"))
```
