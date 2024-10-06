# Assertables: Rust crate of assert macros for testing

The `assertables` Rust crate provides many assert macros to improve your
compile-time tests and run-time reliability.

* Crate: [https://crates.io/crates/assertables](https://crates.io/crates/assertables)
* Docs: [https://docs.rs/assertables/](https://docs.rs/assertables/)
* Repo: [https://github.com/sixarm/assertables-rust-crate/](https://github.com/sixarm/assertables-rust-crate/)
* Contact: [joel@joelparkerhenderson.com](mailto:joel@joelparkerhenderson.com)


## Introduction

The Rust programming language provides assert macros to test code:

* [`assert!(…)`](https://doc.rust-lang.org/std/macro.assert.html)
* [`assert_eq!(a, b)`](https://doc.rust-lang.org/std/macro.assert_eq.html) `// equal`
* [`assert_ne!(a, b)`](https://doc.rust-lang.org/std/macro.assert_ne.html) `// not equal`

The assertables crate provides many more, so you can write smarter tests.

Examples:

* [`assert_lt!(1, 2)`](https://docs.rs/assertables/8.14.0/assertables/macro.assert_lt.html) `// compare values using less than`
* [`assert_approx_eq!(1.0000001, 1.0000002)`](https://docs.rs/assertables/8.14.0/assertables/macro.assert_approx_eq.html) `// compare floats`
* [`assert_len_eq!("hello", "world")`](https://docs.rs/assertables/8.14.0/assertables/macro.assert_len_eq.html) `// compare lengths`
* [`assert_starts_with!("hello world", "hello")`](https://docs.rs/assertables/8.14.0/assertables/macro.assert_starts_with.html) `// compare strings`
* [`assert_fs_read_to_string_eq!("a.txt", "b.txt")`](https://docs.rs/assertables/8.14.0/assertables/macro.assert_fs_read_to_string_eq.html) `// compare files`

Top 3 benefits:

1. You can write better tests to improve reliability and maintainability.
2. You can handle more corner cases without needing to write custom code.
3. You can troubleshoot faster because error messages show specifics.

Top 3 features:

1. Easy to use: everything is well-documented with runnable examples.
2. Runtime savvy: all the assertables macros have runtime versions.
3. Zero overhead: if you don't use a macro, then it's never compiled.

## Install

To use this crate, add it to your `Cargo.toml` file:

```toml
[dev-dependencies]
assertables = "8.14.0"
```

## Highlights

For values:

* [`assert_lt!(a, b)`](https://docs.rs/assertables/8.14.0/assertables/macro.assert_lt.html) `// less than`
* [`assert_le!(a, b)`](https://docs.rs/assertables/8.14.0/assertables/macro.assert_le.html) `// less than`or equal to
* [`assert_gt!(a, b)`](https://docs.rs/assertables/8.14.0/assertables/macro.assert_gt.html) `// greater than`
* [`assert_ge!(a, b)`](https://docs.rs/assertables/8.14.0/assertables/macro.assert_ge.html) `// greater than`or equal to

For approximation:

* [`assert_approx_eq!(a, b)`](https://docs.rs/assertables/8.14.0/assertables/macro.assert_approx_eq.html)
* [`assert_approx_ne!(a, b)`](https://docs.rs/assertables/8.14.0/assertables/macro.assert_approx_ne.html)

For strings:

* [`assert_starts_with!(a, b)`](https://docs.rs/assertables/8.14.0/assertables/macro.assert_starts_with.html)
* [`assert_ends_with!(a, b)`](https://docs.rs/assertables/8.14.0/assertables/macro.assert_ends_with.html)

For lengths:

* [`assert_len_eq!(a, b)`](https://docs.rs/assertables/8.14.0/assertables/macro.assert_len_eq.html)
* [`assert_is_empty!(a)`](https://docs.rs/assertables/8.14.0/assertables/macro.assert_is_empty.html)

For matching:

* [`assert_contains!(a, b)`](https://docs.rs/assertables/8.14.0/assertables/macro.assert_contains.html)
* [`assert_is_match!(a, b)`](https://docs.rs/assertables/8.14.0/assertables/macro.assert_is_match.html)

For infix operators:

* [`assert_infix!(a == b)`](https://docs.rs/assertables/8.14.0/assertables/macro.assert_infix.html)
* [`assert_infix!(a && b)`](https://docs.rs/assertables/8.14.0/assertables/macro.assert_infix.html)

For nearness:

* [`assert_in_delta!(a, b, delta)`](https://docs.rs/assertables/8.14.0/assertables/macro.assert_in_delta.html)
* [`assert_in_epsilon!(a, b, epsilon)`](https://docs.rs/assertables/8.14.0/assertables/macro.assert_in_epsilon.html)

For Result Ok/Err:

* [`assert_ok!(a)`](https://docs.rs/assertables/8.14.0/assertables/macro.assert_ok.html)
* [`assert_err!(a)`](https://docs.rs/assertables/8.14.0/assertables/macro.assert_err.html)

For Option Some/None:

* [`assert_some!(a)`](https://docs.rs/assertables/8.14.0/assertables/macro.assert_some.html)
* [`assert_none!(a)`](https://docs.rs/assertables/8.14.0/assertables/macro.assert_none.html)

For Poll Ready/Pending:

* [`assert_ready!(a)`](https://docs.rs/assertables/8.14.0/assertables/macro.assert_ready.html)
* [`assert_pending!(a)`](https://docs.rs/assertables/8.14.0/assertables/macro.assert_pending.html)

For collections such as arrays, vectors, iterators, sets:

* [`assert_iter_eq!(collection1, collection2)`](https://docs.rs/assertables/8.14.0/assertables/macro.assert_iter_eq.html)
* [`assert_set_eq!(collection1, collection2)`](https://docs.rs/assertables/8.14.0/assertables/macro.assert_set_eq.html)

For file system paths and input/output readers:

* [`assert_fs_read_to_string_eq!(path1, path2)`](https://docs.rs/assertables/8.14.0/assertables/macro.assert_fs_read_to_string_eq.html)
* [`assert_io_read_to_string_eq!(reader1, reader2)`](https://docs.rs/assertables/8.14.0/assertables/macro.assert_io_read_to_string_eq.html)

For command capture of standard output and standard error:

* [`assert_command_stdout_eq!(command1, command2)`](https://docs.rs/assertables/8.14.0/assertables/macro.assert_command_stdout_eq.html)
* [`assert_program_args_stdout_eq!(program1, args1, program2, args2)`](https://docs.rs/assertables/8.14.0/assertables/macro.assert_program_args_stdout_eq.html)


## Modules

There are many more macros that are organized in modules.

Modules for values:

* [`assert_infix`](https://docs.rs/assertables/8.14.0/assertables/assert_infix)
* [`assert_approx`](https://docs.rs/assertables/8.14.0/assertables/assert_approx)

Modules for strings:

* [`assert_starts_with`](https://docs.rs/assertables/8.14.0/assertables/assert_starts_with)
* [`assert_ends_with`](https://docs.rs/assertables/8.14.0/assertables/assert_ends_with)

Modules for lengths:

* [`assert_len`](https://docs.rs/assertables/8.14.0/assertables/assert_len)
* [`assert_is_empty`](https://docs.rs/assertables/8.14.0/assertables/assert_is_empty)

Modules for matching:

* [`assert_contains`](https://docs.rs/assertables/8.14.0/assertables/assert_contains)
* [`assert_is_match`](https://docs.rs/assertables/8.14.0/assertables/assert_is_match)

Modules for collections such as arrays, vectors, iterators, sets, bags:

* [`assert_iter`](https://docs.rs/assertables/8.14.0/assertables/assert_iter) for iterator collections
* [`assert_set`](https://docs.rs/assertables/8.14.0/assertables/assert_set) for set collections
* [`assert_bag`](https://docs.rs/assertables/8.14.0/assertables/assert_bag) for bag collections

Modules for Result Ok/Err:

* [`assert_ok`](module@crate::assert_ok)
* [`assert_err`](module@crate::assert_err)
  
Modules for Option Some/None:

* [`assert_some`](module@crate::assert_some)
* [`assert_none`](module@crate::assert_none)

Modules for Poll Ready/Pending:

* [`assert_ready`](module@crate::assert_ready)
* [`assert_pending`](module@crate::assert_pending)

Modules for functions:

* [`assert_fn`](https://docs.rs/assertables/8.14.0/assertables/assert_fn) for functions in general.
* [`assert_fn_ok`](https://docs.rs/assertables/8.14.0/assertables/assert_fn_ok) for functions that return `Result::Ok`.
* [`assert_fn_err`](https://docs.rs/assertables/8.14.0/assertables/assert_fn_err) for functions that return `Result::Err`.

Modules for readers:

* [`assert_fs_read_to_string`](https://docs.rs/assertables/8.14.0/assertables/assert_fs_read_to_string) for file system path contents.
* [`assert_io_read_to_string`](https://docs.rs/assertables/8.14.0/assertables/assert_io_read_to_string) for input/output reader streams.

Modules for external calls:

* [`assert_command`](https://docs.rs/assertables/8.14.0/assertables/assert_command) for commands with stdout/stderr.
* [`assert_program_args`](https://docs.rs/assertables/8.14.0/assertables/assert_program_args) for programs with args with stdout/stderr.


## Forms

All assertables macros have forms for different outcomes:

* [`assert_gt!(a, b)`](https://docs.rs/assertables/8.14.0/assertables/macro.assert_gt.html) `// panic during typical test`
* [`assert_gt_as_result!(a, b)`](https://docs.rs/assertables/8.14.0/assertables/macro.assert_gt_as_result.html) `// return Ok or Err`
* [`debug_assert_gt!(a, b)`](https://docs.rs/assertables/8.14.0/assertables/macro.debug_assert_gt.html) `// panic when in debug mode`

All assertables macros have forms for an optional message:

* [`assert_gt!(a, b)`](https://docs.rs/assertables/8.14.0/assertables/macro.assert_gt.html) `// automatic error message`
* [`assert_gt!(a, b, "your text")`](https://docs.rs/assertables/8.14.0/assertables/macro.assert_gt.html) `// custom error message`

Many assertables macros have forms for comparing left hand side (LHS) and right hand side (RHS) as the same type or as an expression:

* [`assert_ok_eq!(a, b)`](https://docs.rs/assertables/8.14.0/assertables/macro.assert_ok_eq.html) `// Ok(…) = Ok(…)`
* [`assert_ok_eq_expr!(a, b)`](https://docs.rs/assertables/8.14.0/assertables/macro.assert_ok_eq_expr.html) `// Ok(…) = expression`


## Tracking

* Package: assertables-rust-crate
* Version: 8.15.0
* Created: 2021-03-30T15:47:49Z
* Updated: 2024-10-06T23:11:30Z
* License: MIT or Apache-2.0 or GPL-2.0 or GPL-3.0 or contact us for more
* Contact: Joel Parker Henderson (joel@sixarm.com)
