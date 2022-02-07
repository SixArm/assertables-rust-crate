# Assertables: Rust crate of macros assert and assertable

This `assertables` Rust crate provides macros for `assert…!` and `assume…!`,
which are useful for testing, quality assurance, and runtime reliability.
By SixArm.com.

Crate:
[https://crates.io/crates/assertables](https://crates.io/crates/assure)

Docs:
[https://docs.rs/assertables/](https://docs.rs/assure/)

Repo:
[https://github.com/sixarm/assertables-rust-crate/](https://github.com/sixarm/assertables-rust-crate/)


### assert and assertable

These macros have two styles:

* `assert` macros return `()` or `panic!(…)`.

* `assertable` macros return `Ok(())` or `Err(…)`

Example:

```rust
assert_eq!(1, 1); //-> ()
assert_eq!(1, 2); //-> panic!("assertion failed: `(left == right)`\n  left: `1`,\n right: `2`)

assertable_eq!(1, 1); //-> Ok(())
assertable_eq!(1, 2); //-> Err("assertion failed: `(left == right)`\n  left: `1`,\n right: `2`)
```

These styles are useful because the `assert` macros favor compile-time tests and diagnostics, whereas the `assertable` macros favor run-time reliability and tracing.


### assert

Compare values.

* `assert_eq!(a, b)`: a == b

* `assert_ne!(a, b)`: a !=b

* `assert_lt!(a, b)`: a < b

* `assert_le!(a, b)`: a <= b

* `assert_gt!(a, b)`: a > b

* `assert_ge!(a, b)`: a >= b

Examples:

```rust
assert_eq!(1, 1);
// ()
```

```rust
assert_eq!(1, 2);
// panic!
//   assertion failed: `(left == right)`
//     left: `1`
//    right: `2`
```


### assert_fn

Compare function return values.

* `assert_f_eq!(f, a, b)`: f(a) == f(b)

* `assert_f_ne!(f, a, b)`: f(a) != f(b)

* `assert_f_lt!(f, a, b)`: f(a) < f(b)

* `assert_f_le!(f, a, b)`: f(a) <= f(b)

* `assert_f_gt!(f, a, b)`: f(a) > f(b)

* `assert_f_ge!(f, a, b)`: f(a) != f(b)

Examples:

```rust
assert_f_eq!(i32::abs, 1, -1);
// ()
```

```rust
assert_f_eq!(i32::abs, 1, -2);
// panic!
//   assertion failed: `assert_f_eq!(fn, left, right)`
//     left input: `1`,
//    right input: `-2`,
//     left output: `1`,
//    right output: `2`
```


### assert_f_ok

Compare function `Ok` values.

* `assert_f_ok_eq!(f, a, b)`: f(a).unwrap() == f(b).unwrap()

* `assert_f_ok_ne!(f, a, b)`: f(a).unwrap() != f(b).unwrap()

* `assert_f_ok_lt!(f, a, b)`: f(a).unwrap() < f(b).unwrap()

* `assert_f_ok_le!(f, a, b)`: f(a).unwrap() <= f(b).unwrap()

* `assert_f_ok_gt!(f, a, b)`: f(a).unwrap() > f(b).unwrap()

* `assert_f_ok_ge!(f, a, b)`: f(a).unwrap() >- f(b).unwrap()

Examples:

```rust
// Assert from_str("1").unwrap() == from_str("2").unwrap()
assert_f_ok_eq!(i32::from_str, "1", "2");
// panic!
//   assertion failed: `assert_f_ok_eq!(fn, left, right)`
//     left input: `\"1\"`,
//    right input: `\"2\"`,
//     left output: `1`,
//    right output: `2`
```

```rust
// Assert from_str("1").unwrap() == from_str("2").unwrap()
assert_f_ok_eq!(i32::from_str, "1", "2");
// panic!
//   assertion failed: `assert_f_ok_eq!(fn, left, right)`
//     left input: `\"1\"`,
//    right input: `\"2\"`,
//     left output: `1`,
//    right output: `2`
```


### assert_f_err_string

Compare function `Err` `String` values.

* `assert_f_err_string_eq!(f, a, b)`: f(a).unwrap_err().to_string() == f(b).unwrap_err().to_string()

* `assert_f_err_string_ne!(f, a, b)`: f(a).unwrap_err().to_string() != f(b).unwrap_err().to_string()

* `assert_f_err_string_lt!(f, a, b)`: f(a).unwrap_err().to_string() < f(b).unwrap_err().to_string()

* `assert_f_err_string_le!(f, a, b)`: f(a).unwrap_err().to_string() <= f(b).unwrap_err().to_string()

* `assert_f_err_string_gt!(f, a, b)`: f(a).unwrap_err().to_string() > f(b).unwrap_err().to_string()

* `assert_f_err_string_ge!(f, a, b)`: f(a).unwrap_err().to_string() >= f(b).unwrap_err().to_string()

Example:

```rust
// Assert from_str("foo").unwrap_err().to_string() == from_str("goo").unwrap_err().to_string()
assert_f_err_string_eq!(i32::from_str, "10", "20");
// panic!
//   assertion failed: `assert_f_err_string_eq!(convert_number_to_one_digit_string, left, right)`
//     left input: `10`,
//    right input: `20`,
//     left is err: `true`,
//    right is err: `true`,
//     left output: `\"10 is out of range\"`,
//    right output: `\"20 is out of range\"`
```


### Macros for set comparison

Compare two sets, such as implemented by arrays or vectors.
Item order does not matter. Duplicate items do not matter.

* `assert_set_eq!(a, b)`: set a == set b

* `assert_set_ne!(a, b)`: set a != set b

* `assert_set_subset!(a, b)`: set a ⊆ set b

* `assert_set_superset!(a, b)`: set a ⊇ set b

* `assert_set_joint!(a, b)`: set a is joint with set b

* `assert_set_disjoint!(a, b)`: set a is disjoint with set b


Example:

```rust
// Assert set {1, 2} == set {2, 1}
assert_set_eq!([1, 2], [2, 1]);
```


### Macros for bag comparison

Compare two bags, such as implemented by arrays or vectors.
Item order does not matter. Duplicate items do matter.

* `assert_bag_eq(a, b)`: bag a == bag b

* `assert_bag_ne(a, b)`: bag a != bag b

Example:

```rust
/// Assert bag {1, 1, 2} == bag {2, 1, 1}
assert_bag_eq!([1, 1, 2], [2, 1, 1]);
```


### Macros for IO-related checking

Compare two IO-related values, such as comparison of files, streams, etc.

* `assert_io!(a)`: a is true

* `assert_io_eq!(a, b)`: a == b

* `assert_io_ne!(a, b)`: a != b

* `assert_io_lt!(a, b)`: a < b

* `assert_io_le!(a, b)`: a <= b

* `assert_io_gt!(a, b)`: a > b

* `assert_io_ge!(a, b)`: a >= b


Examples:

```rust
// TODO
assert_io_lt!(1, 2);
```


### Macros for read_to_string()

Compare read_to_string() results. This can be useful for file handles, byte
arrays, input streams, anything that implements the trait std::io::Read, etc.

* `assert_read_to_string_eq!(a, b)`: a.read_to_string() == b.read_to_string()

* `assert_read_to_string_ne!(a, b)`: a.read_to_string() != b.read_to_string()

* `assert_read_to_string_lt!(a, b)`: a.read_to_string() < b.read_to_string()

* `assert_read_to_string_le!(a, b)`: a.read_to_string() <= b.read_to_string()

* `assert_read_to_string_gt!(a, b)`: a.read_to_string() > b.read_to_string()

* `assert_read_to_string_ge!(a, b)`: a.read_to_string() >= b.read_to_string()


Example:

```rust
use std::io::Read;
let mut a = "a".as_bytes();
let mut b = "b".as_bytes();
assert_read_to_string_eq!(a, b);
//-> panic!("assertion failed: `assert_read_to_string_eq!(left, right)`\n  left: `\"a\"`,\n right: `\"b`");
```
