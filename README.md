# Assertables: Rust crate of assert macros for testing

The `assertables` Rust crate provides many assert macros to improve your
compile-time tests and run-time reliability.

* Crate: [https://crates.io/crates/assertables](https://crates.io/crates/assertables)
* Docs: [https://docs.rs/assertables/](https://docs.rs/assertables/)
* Repo: [https://github.com/sixarm/assertables-rust-crate/](https://github.com/sixarm/assertables-rust-crate/)
* Contact: [joel@joelparkerhenderson.com](mailto:joel@joelparkerhenderson.com)


## Introduction

The Rust programming language provides assert macros such as `assert!(x)` to
test code. The assertables crate provides many more for numbers, strings,
results, options, iterators, files, streams, and more. See below for examples.

Top benefits:

1. You can write better tests to improve reliability and maintainability.
2. You can handle more corner cases without needing to write custom code.
3. You can troubleshoot faster because error messages show specifics.

To use this crate, add it to your `Cargo.toml` file as a development dependency:

```toml
[dev-dependencies]
assertables = "*"
```

Comparable crates:
    [`assert_matches`](https://crates.io/crates/assert_matches),
    [`assert_approx_eq`](https://crates.io/crates/assert_approx_eq),
    [`more_asserts`](https://crates.io/crates/more_asserts),
    [`cool_asserts`](https://crates.io/crates/cool_asserts),
    [`assert2`](https://crates.io/crates/assert2),
    [`claims`](https://crates.io/crates/claims),
    [`static_assertions`](https://crates.io/crates/static_assertions).

## Highlights

Values:

* [`assert_eq!(a, b);`](https://docs.rs/assertables/9.0.0/assertables/assert_eq) `// a == b`
* [`assert_ne!(a, b);`](https://docs.rs/assertables/9.0.0/assertables/assert_ne) `// a != b`
* [`assert_lt!(a, b);`](https://docs.rs/assertables/9.0.0/assertables/assert_lt) `// a < b`
* [`assert_le!(a, b);`](https://docs.rs/assertables/9.0.0/assertables/assert_le) `// a <= b`
* [`assert_gt!(a, b);`](https://docs.rs/assertables/9.0.0/assertables/assert_gt) `// a > b`
* [`assert_ge!(a, b);`](https://docs.rs/assertables/9.0.0/assertables/assert_ge) `// a >= b`

Approximations:

* [`assert_approx_eq!(a, b);`](https://docs.rs/assertables/9.0.0/assertables/assert_approx/assert_approx_eq) `// |a-b| <= 1e-6`
* [`assert_in_delta!(a, b, delta);`](https://docs.rs/assertables/9.0.0/assertables/assert_in/assert_in_delta) `// |a-b| <= delta`
* [`assert_in_epsilon!(a, b, epsilon);`](https://docs.rs/assertables/9.0.0/assertables/assert_in/assert_in_epsilon) `// |a-b| <= epsilon * min(a,b)`

Groups for iterators, chars, etc.:

* [`assert_all!(group, predicate);`](https://docs.rs/assertables/9.0.0/assertables/assert_all) `// group.all(predicate)`
* [`assert_any!(group, predicate);`](https://docs.rs/assertables/9.0.0/assertables/assert_any) `// group.any(predicate)`

Infix for order operators, logic operators, etc.:

* [`assert_infix!(a == b);`](https://docs.rs/assertables/9.0.0/assertables/assert_infix) `// order: == != < <= > >=`
* [`assert_infix!(a && b);`](https://docs.rs/assertables/9.0.0/assertables/assert_infix) `// logic: && || ^ & |`

Parts for strings, vectors, etc.:

* [`assert_starts_with!(whole, part);`](https://docs.rs/assertables/9.0.0/assertables/assert_starts_with) `// whole.starts_with(part)`
* [`assert_ends_with!(whole, part);`](https://docs.rs/assertables/9.0.0/assertables/assert_ends_with) `// whole.ends_with(part)`

Lengths and counts for strings, vectors, iterators, etc.:

* [`assert_len_eq!(item, x);`](https://docs.rs/assertables/9.0.0/assertables/assert_len/assert_len_eq) `// item.len() == x`
* [`assert_count_eq!(item, x);`](https://docs.rs/assertables/9.0.0/assertables/assert_count/assert_count_eq) `// item.count() == x`
* [`assert_is_empty!(item);`](https://docs.rs/assertables/9.0.0/assertables/assert_is_empty/assert_is_empty) `// item.is_empty()`

Matching for strings, regex, etc.:

* [`assert_matches!(expression, pattern);`](https://docs.rs/assertables/9.0.0/assertables/assert_matches) `// matches!(expression, pattern)`
* [`assert_is_match!(matcher, matchee);`](https://docs.rs/assertables/9.0.0/assertables/assert_is_match) `// matcher.is_match(matchee)`
* [`assert_contains!(container, containee);`](https://docs.rs/assertables/9.0.0/assertables/assert_contains) `// container.contains(containee)`

Collections for arrays, vectors, iterators, sets, maps:

* [`assert_iter_eq2!(arr1, arr2);`](https://docs.rs/assertables/9.0.0/assertables/assert_iter) `// eq ne etc.`
* [`assert_set_eq2!(vec1, vec2);`](https://docs.rs/assertables/9.0.0/assertables/assert_set) `// eq ne etc.`
* [`assert_bag_eq2!(map1, map2);`](https://docs.rs/assertables/9.0.0/assertables/assert_bag) `// eq ne etc.`

Result Ok/Err:

* [`assert_ok_eq!(result, x);`](https://docs.rs/assertables/9.0.0/assertables/assert_ok/assert_ok_eq) `// result is Ok(x)`
* [`assert_err_eq!(result, x);`](https://docs.rs/assertables/9.0.0/assertables/assert_err/assert_err_eq) `// result is Err(x)`

Option Some/None:

* [`assert_some_eq!(option, x);`](https://docs.rs/assertables/9.0.0/assertables/assert_some/assert_some_eq) `// option is Some(x)`
* [`assert_none!(option);`](https://docs.rs/assertables/9.0.0/assertables/assert_none/assert_none) `// option is None`

Poll Ready/Pending:

* [`assert_ready_eq!(poll, x);`](https://docs.rs/assertables/9.0.0/assertables/assert_ready/assert_ready_eq) `// poll is Ready(x)`
* [`assert_pending!(poll);`](https://docs.rs/assertables/9.0.0/assertables/assert_pending/assert_pending) `// poll is Pending`

Read file system paths and input/output streams:

* [`assert_fs_read_to_string_eq!(path, x);`](https://docs.rs/assertables/9.0.0/assertables/assert_fs_read_to_string_eq) `// read path == x`
* [`assert_io_read_to_string_eq!(stream, x);`](https://docs.rs/assertables/9.0.0/assertables/assert_io_read_to_string) `// read stream == x`

Run commands and programs then assert on stdout or stderr:

* [`assert_command_stdout_eq!(command, x);`](https://docs.rs/assertables/9.0.0/assertables/assert_command) `// command stdout == x`
* [`assert_program_args_stdout_eq!(program, args, x);`](https://docs.rs/assertables/9.0.0/assertables/assert_program_args) `// program-args stdout == x`

Function comparisons, which can be especially helpful for refactoring:

* [`assert_fn_eq!(fn, x);`](https://docs.rs/assertables/9.0.0/assertables/assert_fn) `// fn() == x`
* [`assert_fn_ok_eq!(fn, x);`](https://docs.rs/assertables/9.0.0/assertables/assert_fn_ok) `// fn() == Ok(x)`
* [`assert_fn_err_eq!(fn, x);`](https://docs.rs/assertables/9.0.0/assertables/assert_fn_err) `// fn() == Err(x)`


## Forms

All assertables macros have forms for an optional message:

* [`assert_gt!(a, b);`](https://docs.rs/assertables/9.0.0/assertables/macro.assert_gt.html) `// automatic error message`
* [`assert_gt!(a, b, "your text");`](https://docs.rs/assertables/9.0.0/assertables/macro.assert_gt.html) `// custom error message`

All assertables macros have forms for different outcomes:

* [`assert_gt!(a, b);`](https://docs.rs/assertables/9.0.0/assertables/macro.assert_gt.html) `// panic during typical test`
* [`assert_gt_as_result!(a, b);`](https://docs.rs/assertables/9.0.0/assertables/macro.assert_gt_as_result.html) `// return Ok or Err`
* [`debug_assert_gt!(a, b);`](https://docs.rs/assertables/9.0.0/assertables/macro.debug_assert_gt.html) `// panic when in debug mode`

Many assertables macros have forms for comparing one item (to an expression) or two items (to each other):

* [`assert_ok_eq!(a, x);`](https://docs.rs/assertables/9.0.0/assertables/macro.assert_ok_eq.html) `// a.unwrap() == x`
* [`assert_ok_eq2!(a, b);`](https://docs.rs/assertables/9.0.0/assertables/macro.assert_ok_eq2.html) `// a.unwrap() == b.unwrap()`


## Migrating from version 8 to version 9

A macro naming convention is changing, to improve usability.

Version 8 naming convention:

```rust
assert_foo_eq_expr!(a, x) // compare one item with an expression
assert_foo_eq!(a, b) // compare two items of the same type
```

Version 9 naming conventions:

```rust
assert_foo_eq!(a, x) // compare one item with an expression
assert_foo_eq2!(a, b) // compare two items of the same type
```

To update your code, one way is to use regular expressions.

Run this first:

* Match: `\b(|debug_)(assert_\w*_)(eq|ne|lt|le|gt|ge)(|_as_result)\b`

* Replace: `$1$2$32$4`

Run this second:

* Match: `\b(|debug_)(assert_\w*_)(eq|ne|lt|le|gt|ge)_expr(|_as_result)\b`

* Replace: `$1$2$3$4`


## Tracking

* Package: assertables-rust-crate
* Version: 9.0.0
* Created: 2021-03-30T15:47:49Z
* Updated: 2024-10-19T21:00:54Z
* License: MIT or Apache-2.0 or GPL-2.0 or GPL-3.0 or contact us for more
* Contact: Joel Parker Henderson (joel@sixarm.com)
