# Assertables: Rust crate of assert macros for testing

Assertables is a Rust crate that provides many assert macros 
to improve your compile-time tests and run-time reliability.

* Crate: [https://crates.io/crates/assertables](https://crates.io/crates/assertables)
* Docs: [https://docs.rs/assertables/](https://docs.rs/assertables/)
* Repo: [https://github.com/sixarm/assertables-rust-crate/](https://github.com/sixarm/assertables-rust-crate/)
* Contact: [joel@joelparkerhenderson.com](mailto:joel@joelparkerhenderson.com)

## Introduction

The Assertables Rust crate provides many assert macros 
that can help you develop, test, and debug.

* Test values with 
  [assert_lt](https://docs.rs/assertables/9.1.0/assertables/assert_lt), 
  [assert_gt](https://docs.rs/assertables/9.1.0/assertables/assert_gt), 
  [etc.](https://docs.rs/assertables/)
* Test strings with 
  [assert_starts_with](https://docs.rs/assertables/9.1.0/assertables/assert_starts_with),
  [assert_ends_with](https://docs.rs/assertables/9.1.0/assertables/assert_ends_with),
  [etc.](https://docs.rs/assertables/)
* Test groups with 
  [assert_all](https://docs.rs/assertables/9.1.0/assertables/assert_all),
  [assert_any](https://docs.rs/assertables/9.1.0/assertables/assert_any),
  [etc.](https://docs.rs/assertables/)

There are many more for
  [results](https://docs.rs/assertables/9.1.0/assertables/assert_result), 
  [options](https://docs.rs/assertables/9.1.0/assertables/assert_option), 
  [polls](https://docs.rs/assertables/9.1.0/assertables/assert_poll),
  [matches](https://docs.rs/assertables/9.1.0/assertables/assert_matches),
  [iterators](https://docs.rs/assertables/9.1.0/assertables/assert_iter),
  [sets](https://docs.rs/assertables/9.1.0/assertables/assert_set), 
  [files](https://docs.rs/assertables/9.1.0/assertables/assert_fs_read_to_string),
  [bytes](https://docs.rs/assertables/9.1.0/assertables/assert_io_read_to_string),
  [commands](https://docs.rs/assertables/9.1.0/assertables/assert_command), 
  [etc.](https://docs.rs/assertables/)

To use this crate, add it to your file `Cargo.toml`:

```toml
assertables = "9.1.0"
```

Top benefits:

1. You can write better tests to improve reliability and maintainability.
2. You can handle more corner cases without needing to write custom code.
3. You can troubleshoot faster because error messages show specifics.

Top features:

1. Easy to use: everything is well-documented with runnable examples.
2. Zero overhead: if you don't use a macro, then it's not compiled.
3. Multiple forms: for debugging, for results, and for success returns.

Help:

* [Documentation](https://docs.rs/assertables/)
* [Frequently asked questions](https://github.com/SixArm/assertables-rust-crate/tree/main/help/faq)
* [Examples](https://github.com/SixArm/assertables-rust-crate/blob/main/tests/examples/)
* [Upgrade from version 8 to 9](https://github.com/SixArm/assertables-rust-crate/tree/main/help/upgrades/upgrade-from-version-8-to-9)
* [Comparisons to more_asserts, cool_asserts, assert2, claims, etc.](https://github.com/SixArm/assertables-rust-crate/tree/main/help/comparisons)

## Highlights

Values:

* [`assert_eq!(a, b)`](https://docs.rs/assertables/9.1.0/assertables/assert_eq) `// a == b`
* [`assert_ne!(a, b)`](https://docs.rs/assertables/9.1.0/assertables/assert_ne) `// a != b`
* [`assert_lt!(a, b)`](https://docs.rs/assertables/9.1.0/assertables/assert_lt) `// a < b`
* [`assert_le!(a, b)`](https://docs.rs/assertables/9.1.0/assertables/assert_le) `// a <= b`
* [`assert_gt!(a, b)`](https://docs.rs/assertables/9.1.0/assertables/assert_gt) `// a > b`
* [`assert_ge!(a, b)`](https://docs.rs/assertables/9.1.0/assertables/assert_ge) `// a >= b`

Differences:

* [`assert_approx_eq!(a, b)`](https://docs.rs/assertables/9.1.0/assertables/assert_approx/assert_approx_eq) `// |a-b| <= 1e-6`
* [`assert_abs_diff_eq!(a, b, delta)`](https://docs.rs/assertables/9.1.0/assertables/assert_abs_diff/assert_abs_diff_eq) `// |a-b| == Δ`
* [`assert_in_delta!(a, b, delta)`](https://docs.rs/assertables/9.1.0/assertables/assert_in/assert_in_delta) `// |a-b| <= Δ`
* [`assert_in_epsilon!(a, b, epsilon)`](https://docs.rs/assertables/9.1.0/assertables/assert_in/assert_in_epsilon) `// |a-b| <= ε min(a,b)`

Groups for iterators, chars, etc.:

* [`assert_all!(group, predicate)`](https://docs.rs/assertables/9.1.0/assertables/assert_all) `// group.all(predicate)`
* [`assert_any!(group, predicate)`](https://docs.rs/assertables/9.1.0/assertables/assert_any) `// group.any(predicate)`

Infix for order operators, logic operators, etc.:

* [`assert_infix!(a == b)`](https://docs.rs/assertables/9.1.0/assertables/assert_infix) `// order: == != < <= > >=`
* [`assert_infix!(a && b)`](https://docs.rs/assertables/9.1.0/assertables/assert_infix) `// logic: && || ^ & |`

Lengths and counts for strings, vectors, iterators, etc.:

* [`assert_len_eq!(a, b)`](https://docs.rs/assertables/9.1.0/assertables/assert_len/assert_len_eq) `// a.len() == b.len()`
* [`assert_count_eq!(a, b)`](https://docs.rs/assertables/9.1.0/assertables/assert_count/assert_count_eq) `// a.count() == b.count()`
* [`assert_is_empty!(a)`](https://docs.rs/assertables/9.1.0/assertables/assert_is_empty/assert_is_empty) `// a.is_empty()`

Matching:

* [`assert_starts_with!(whole, part)`](https://docs.rs/assertables/9.1.0/assertables/assert_starts_with) `// whole.starts_with(part)`
* [`assert_ends_with!(whole, part)`](https://docs.rs/assertables/9.1.0/assertables/assert_ends_with) `// whole.ends_with(part)`
* [`assert_contains!(container, x)`](https://docs.rs/assertables/9.1.0/assertables/assert_contains) `// container.contains(x)`
* [`assert_matches!(expr, pattern)`](https://docs.rs/assertables/9.1.0/assertables/assert_matches) `// matches!(expr, pattern)`
* [`assert_is_match!(matcher, x)`](https://docs.rs/assertables/9.1.0/assertables/assert_is_match) `// matcher.is_match(x)`

Results:

* [`assert_ok!(a)`](https://docs.rs/assertables/9.1.0/assertables/assert_ok) `// a is Ok`
* [`assert_ok_eq_x!(a, x)`](https://docs.rs/assertables/9.1.0/assertables/assert_ok/assert_ok_eq_x) `// a is Ok(x)`
* [`assert_err!(a)`](https://docs.rs/assertables/9.1.0/assertables/assert_err) `// a is Err`

Options:

* [`assert_some!(a)`](https://docs.rs/assertables/9.1.0/assertables/assert_some) `// a is Some`
* [`assert_some_eq_x!(a, x)`](https://docs.rs/assertables/9.1.0/assertables/assert_some/assert_some_eq_x) `// a is Some(x)`
* [`assert_none!(a)`](https://docs.rs/assertables/9.1.0/assertables/assert_none) `// a is None`

Polls:

* [`assert_ready!(a)`](https://docs.rs/assertables/9.1.0/assertables/assert_ready) `// a is Ready`
* [`assert_ready_eq_x!(a, x)`](https://docs.rs/assertables/9.1.0/assertables/assert_ready/assert_ready__eq_x) `// a is Ready(x)`
* [`assert_pending!(a)`](https://docs.rs/assertables/9.1.0/assertables/assert_pending) `// a is Pending`

Readers:

* [`assert_fs_read_to_string_eq!(a_path, b_path)`](https://docs.rs/assertables/9.1.0/assertables/assert_fs_read_to_string_eq) `// read a_path == read b_path`
* [`assert_io_read_to_string_eq!(a_bytes, b_bytes)`](https://docs.rs/assertables/9.1.0/assertables/assert_io_read_to_string) `// read a_bytes == read b_bytes`

Commands:

* [`assert_command_stdout_eq!(a_command, b_command)`](https://docs.rs/assertables/9.1.0/assertables/assert_command) `// a_command stdout == b_command stdout`
* [`assert_program_args_stderr_eq!(a_program, a_args, b_program, b_args)`](https://docs.rs/assertables/9.1.0/assertables/assert_program_args) `// a_program a_args stderr == b_program b_args stderr`

Collections:

* [`assert_iter_eq!(arr1, arr2)`](https://docs.rs/assertables/9.1.0/assertables/assert_iter) `// a into iter == b into iter`
* [`assert_set_eq!(vec1, vec2)`](https://docs.rs/assertables/9.1.0/assertables/assert_set) `// a into set == b into set`
* [`assert_bag_eq!(map1, map2)`](https://docs.rs/assertables/9.1.0/assertables/assert_bag) `// a into bag == b into bag`

For a complete list of modules and macros, see the [docs](https://docs.rs/assertables/)


## Forms

All the macros have forms for an optional message:

* [`assert_gt!(a, b)`](https://docs.rs/assertables/9.1.0/assertables/macro.assert_gt.html) `// default message`
* [`assert_gt!(a, b, "your text")`](https://docs.rs/assertables/9.1.0/assertables/macro.assert_gt.html) `// custom message`

All the macros have forms for different outcomes:

* [`assert_gt!(a, b)`](https://docs.rs/assertables/9.1.0/assertables/macro.assert_gt.html) `// panic`
* [`assert_gt_as_result!(a, b)`](https://docs.rs/assertables/9.1.0/assertables/macro.assert_gt_as_result.html) `// return Result, no panic`
* [`debug_assert_gt!(a, b)`](https://docs.rs/assertables/9.1.0/assertables/macro.debug_assert_gt.html) `// special use in debug mode`

Many of the macros have a "solo" form for comparing one item to an expression, and a "pair" form for comparing two items to each other:

* [`assert_ok_eq!(a, x)`](https://docs.rs/assertables/9.1.0/assertables/macro.assert_ok_eq.html) `// a.unwrap() == x`
* [`assert_ok_eq!(a, b)`](https://docs.rs/assertables/9.1.0/assertables/macro.assert_ok_eq.html) `// a.unwrap() == b.unwrap()`

Many of the macros has a "success return", which means the macro returns data that you can optionally use for more testing.

* [`let inner = assert_ok!(result)`](https://docs.rs/assertables/9.1.0/assertables/macro.assert_ok.html)
* [`let string = assert_fs_read_to_string_ne!("alfa.txt", "")`](https://docs.rs/assertables/9.1.0/assertables/macro.assert_fs_read_to_string_ne.html)
* [`let stdout = assert_command_stdout_gt!("ls", vec![b' '])`](https://docs.rs/assertables/9.1.0/assertables/macro.assert_command_stdout_gt.html)


## Tracking

* Package: assertables-rust-crate
* Version: 9.1.0
* Created: 2021-03-30T15:47:49Z
* Updated: 2024-10-29T20:21:37Z
* License: MIT or Apache-2.0 or GPL-2.0 or GPL-3.0 or contact us for more
* Contact: Joel Parker Henderson (joel@joelparkerhenderson.com)
