# Assertables: Rust crate of assert macros for testing

The `assertables` Rust crate provides many assert macros to improve your
compile-time tests and run-time reliability.

* Crate: [https://crates.io/crates/assertables](https://crates.io/crates/assertables)
* Docs: [https://docs.rs/assertables/](https://docs.rs/assertables/)
* Repo: [https://github.com/sixarm/assertables-rust-crate/](https://github.com/sixarm/assertables-rust-crate/)
* Contact: [joel@joelparkerhenderson.com](mailto:joel@joelparkerhenderson.com)


## Introduction

The Rust programming language provides assert macros such as `assert!(x)` to test code.

* [`assert!(a)`](https://doc.rust-lang.org/std/macro.assert.html) `// a is true`
* [`assert_eq!(a, b)`](https://doc.rust-lang.org/std/macro.assert_eq.html) `// a is equal to b`
* [`assert_ne!(a, b)`](https://doc.rust-lang.org/std/macro.assert_ne.html) `// a is not equal to b`

The assertables crate provides many more, to help you work with numbers,
strings, results, options, polls, iterators, files, streams, commands, and more. 

Examples:

```rust
use assertables::*;
let s = "hello world";
assert_starts_with!(s, "hello");
assert_contains!(s, "o");
assert_len_eq!(s, "***********");
assert_all!(s.chars(), |c: char| c < 'x');
assert_any!(s.chars(), |c: char| c.is_whitespace());
```

Top benefits:

1. You can write better tests to improve reliability and maintainability.
2. You can handle more corner cases without needing to write custom code.
3. You can troubleshoot faster because error messages show specifics.

Top features:

1. Easy to use: everything is well-documented with runnable examples.
2. Runtime savvy: all the assertables macros have runtime versions.
3. Zero overhead: if you don't use a macro, then it's never compiled.

To use the macros, add this to your `Cargo.toml` file:

```toml
[dev-dependencies]
assertables = "*"
```

## Highlights

For values:

* [`assert_lt!(a, b)`](https://docs.rs/assertables/8.16.0/assertables/macro.assert_lt.html) `// less than`
* [`assert_le!(a, b)`](https://docs.rs/assertables/8.16.0/assertables/macro.assert_le.html) `// less than or equal to`
* [`assert_gt!(a, b)`](https://docs.rs/assertables/8.16.0/assertables/macro.assert_gt.html) `// greater than`
* [`assert_ge!(a, b)`](https://docs.rs/assertables/8.16.0/assertables/macro.assert_ge.html) `// greater than or equal to`

For approximations:

* [`assert_approx_eq!(a, b)`](https://docs.rs/assertables/8.16.0/assertables/macro.assert_approx_eq.html)
* [`assert_approx_ne!(a, b)`](https://docs.rs/assertables/8.16.0/assertables/macro.assert_approx_ne.html)

For nearness:

* [`assert_in_delta!(a, b, delta)`](https://docs.rs/assertables/8.16.0/assertables/macro.assert_in_delta.html)
* [`assert_in_epsilon!(a, b, epsilon)`](https://docs.rs/assertables/8.16.0/assertables/macro.assert_in_epsilon.html)

For groups:

* [`assert_all!(group, predicate)`](https://docs.rs/assertables/8.16.0/assertables/macro.assert_all.html)
* [`assert_any!(group, predicate)`](https://docs.rs/assertables/8.16.0/assertables/macro.assert_any.html)


## Modules

There are many more macros that are organized in modules.

For values:

* [`assert_infix`](https://docs.rs/assertables/8.16.0/assertables/assert_infix)
* [`assert_approx`](https://docs.rs/assertables/8.16.0/assertables/assert_approx)

For parts:

* [`assert_starts_with`](https://docs.rs/assertables/8.16.0/assertables/assert_starts_with)
* [`assert_ends_with`](https://docs.rs/assertables/8.16.0/assertables/assert_ends_with)

For lengths:

* [`assert_len`](https://docs.rs/assertables/8.16.0/assertables/assert_len)
* [`assert_is_empty`](https://docs.rs/assertables/8.16.0/assertables/assert_is_empty)

For matching:

* [`assert_contains`](https://docs.rs/assertables/8.16.0/assertables/assert_contains)
* [`assert_is_match`](https://docs.rs/assertables/8.16.0/assertables/assert_is_match)

For collections such as arrays, vectors, iterators, sets, bags:

* [`assert_iter`](https://docs.rs/assertables/8.16.0/assertables/assert_iter) for iterator collections
* [`assert_set`](https://docs.rs/assertables/8.16.0/assertables/assert_set) for set collections
* [`assert_bag`](https://docs.rs/assertables/8.16.0/assertables/assert_bag) for bag collections

For Result Ok/Err:

* [`assert_ok`](module@crate::assert_ok)
* [`assert_err`](module@crate::assert_err)
  
For Option Some/None:

* [`assert_some`](module@crate::assert_some)
* [`assert_none`](module@crate::assert_none)

For Poll Ready/Pending:

* [`assert_ready`](module@crate::assert_ready)
* [`assert_pending`](module@crate::assert_pending)

For functions:

* [`assert_fn`](https://docs.rs/assertables/8.16.0/assertables/assert_fn) for functions in general.
* [`assert_fn_ok`](https://docs.rs/assertables/8.16.0/assertables/assert_fn_ok) for functions that return `Result::Ok`.
* [`assert_fn_err`](https://docs.rs/assertables/8.16.0/assertables/assert_fn_err) for functions that return `Result::Err`.

For reading file systems and input/output streams:

* [`assert_fs_read_to_string`](https://docs.rs/assertables/8.16.0/assertables/assert_fs_read_to_string) for file system path contents.
* [`assert_io_read_to_string`](https://docs.rs/assertables/8.16.0/assertables/assert_io_read_to_string) for input/output reader streams.

For commands to capture stdout and stderr:

* [`assert_command`](https://docs.rs/assertables/8.16.0/assertables/assert_command) for commands with stdout/stderr.
* [`assert_program_args`](https://docs.rs/assertables/8.16.0/assertables/assert_program_args) for programs with args with stdout/stderr.


## Forms

All assertables macros have forms for different outcomes:

* [`assert_gt!(a, b)`](https://docs.rs/assertables/8.16.0/assertables/macro.assert_gt.html) `// panic during typical test`
* [`assert_gt_as_result!(a, b)`](https://docs.rs/assertables/8.16.0/assertables/macro.assert_gt_as_result.html) `// return Ok or Err`
* [`debug_assert_gt!(a, b)`](https://docs.rs/assertables/8.16.0/assertables/macro.debug_assert_gt.html) `// panic when in debug mode`

All assertables macros have forms for an optional message:

* [`assert_gt!(a, b)`](https://docs.rs/assertables/8.16.0/assertables/macro.assert_gt.html) `// automatic error message`
* [`assert_gt!(a, b, "your text")`](https://docs.rs/assertables/8.16.0/assertables/macro.assert_gt.html) `// custom error message`

Many assertables macros have forms for comparing left hand side (LHS) and right hand side (RHS) as the same type or as an arbitrary expression:

* [`assert_ok_eq!(a, b)`](https://docs.rs/assertables/8.16.0/assertables/macro.assert_ok_eq.html) `// Ok(…) = Ok(…)`
* [`assert_ok_eq_expr!(a, b)`](https://docs.rs/assertables/8.16.0/assertables/macro.assert_ok_eq_expr.html) `// Ok(…) = expression`


## Tracking

* Package: assertables-rust-crate
* Version: 8.16.0
* Created: 2021-03-30T15:47:49Z
* Updated: 2024-10-08T15:07:34Z
* License: MIT or Apache-2.0 or GPL-2.0 or GPL-3.0 or contact us for more
* Contact: Joel Parker Henderson (joel@sixarm.com)
