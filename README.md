# Assertables: Rust crate of assert macros for testing

The `assertables` Rust crate provides many assert macros to improve your
compile-time tests and run-time reliability.

* Crate: [https://crates.io/crates/assertables](https://crates.io/crates/assertables)
* Docs: [https://docs.rs/assertables/](https://docs.rs/assertables/)
* Repo: [https://github.com/sixarm/assertables-rust-crate/](https://github.com/sixarm/assertables-rust-crate/)
* Contact: [joel@joelparkerhenderson.com](mailto:joel@joelparkerhenderson.com)


## Introduction

The Rust programming language provides a few built-in assert macros to test code:

* `assert!()`
* `assert_eq!(a, b)`
* `assert_ne!(a, b)`

The assertables crate provides many more, so you can write smarter tests.

For values:

* [`assert_gt!(a, b)`](https://docs.rs/assertables/latest/assertables/macro.assert_gt.html)
* [`assert_lt!(a, b)`](https://docs.rs/assertables/latest/assertables/macro.assert_lt.html)

For numbers:

* [`assert_in_delta!(a, b, delta)`](https://docs.rs/assertables/latest/assertables/macro.assert_in_delta.html)
* [`assert_in_epsilon!(a, b, epsilon)`](https://docs.rs/assertables/latest/assertables/macro.assert_in_epsilon.html)

For strings:

* [`assert_starts_with!(a, b)`](https://docs.rs/assertables/latest/assertables/macro.assert_starts_with.html)
* [`assert_ends_with!(a, b)`](https://docs.rs/assertables/latest/assertables/macro.assert_ends_with.html)

For matching:

* [`assert_contains!(a, b)`](https://docs.rs/assertables/latest/assertables/macro.assert_contains.html)
* [`assert_is_match!(a, b)`](https://docs.rs/assertables/latest/assertables/macro.assert_is_match.html)

For infix numeric operators and infix logical operators:

* [`assert_infix!(a == b)`](https://docs.rs/assertables/latest/assertables/macro.assert_infix.html)
* [`assert_infix!(a && b)`](https://docs.rs/assertables/latest/assertables/macro.assert_infix.html)

For maybes:

* [`assert_result_ok!(a)`](https://docs.rs/assertables/latest/assertables/macro.assert_result_ok.html)
* [`assert_option_some!(a)`](https://docs.rs/assertables/latest/assertables/macro.assert_option_some.html)

For collections such as arrays, vectors, maps, sets:

* [`assert_set_subset_eq!(a, b)`](https://docs.rs/assertables/latest/assertables/macro.assert_set_subset_eq.html)
* [`assert_set_disjoint_eq!(a, b)`](https://docs.rs/assertables/latest/assertables/macro.assert_set_disjoint_eq.html)

For file system paths and input/output readers:

* [`assert_fs_read_to_string_eq!(path1, path2)`](https://docs.rs/assertables/latest/assertables/macro.assert_fs_read_to_string_eq.html)
* [`assert_io_read_to_string_eq!(reader1, reader2)`](https://docs.rs/assertables/latest/assertables/macro.assert_io_read_to_string_eq.html)

For command capture of standard output and standard error:

* [`assert_command_stdout_eq!(command1, command2)`](https://docs.rs/assertables/latest/assertables/macro.assert_command_stdout_eq.html);
* [`assert_program_args_stdout_eq!(program1, args1, program2, args2`](https://docs.rs/assertables/latest/assertables/macro.assert_program_args_stdout_eq.html)

There are many more macros that are grouped into modules.

Modules for enums:

* [`assert_option`](https://docs.rs/assertables/latest/assertables/assert_option)for `Option` {`Some`, `None`}
* [`assert_result`](https://docs.rs/assertables/latest/assertables/assert_result) for `Result` {`Ok`, `Err`}
* [`assert_poll]`](https://docs.rs/assertables/latest/assertables/assert_poll) for `Poll` {`Ready`, `Pending`}

Modules for collections, such as arrays, vectors, lists, maps:

* [`assert_set`](https://docs.rs/assertables/latest/assertables/assert_set) for set collections
* [`assert_bag`](https://docs.rs/assertables/latest/assertables/assert_bag) for bag collections

Modules for functions:

* [`assert_fn`](https://docs.rs/assertables/latest/assertables/assert_fn) for functions in general.
* [`assert_fn_ok`](https://docs.rs/assertables/latest/assertables/assert_fn_ok) for functions that return `Result::Ok`.
* [`assert_fn_err`](https://docs.rs/assertables/latest/assertables/assert_fn_err) for functions that return `Result::Err`.

Modules for readers:

* [`assert_fs_read_to_string`](https://docs.rs/assertables/latest/assertables/assert_fs_read_to_string) for file system path contents.
* [`assert_io_read_to_string`](https://docs.rs/assertables/latest/assertables/assert_io_read_to_string) for input/output reader streams.

Modules for external calls:

* [`assert_command`](https://docs.rs/assertables/latest/assertables/assert_command) for commands with stdout/stderr.
* [`assert_program_args`](https://docs.rs/assertables/latest/assertables/assert_program_args) for programs with args with stdout/stderr.


### Benefits

* Your tests are more purposeful and powerful. This helps your code be more
reliable.

* Your assert failures provide more information. This helps you
troubleshoot faster.

* You gain runtime asserts. This helps you with validations and
verifications.


### Features

* Easy to use: each macro is well-documented with runnable examples and
tests.

* Zero overhead: if you don't use a macro, then it's never compiled into
your code.

* Zero dependencies: the crate has no release dependencies, and just a short list of development dependencies.


### Naming conventions

Abbreviations:

* `eq` ≈ equal
* `ne` ≈ not equal.
* `lt` ≈ less than
* `le` ≈ less than or equal.
* `gt` ≈ greater than
* `ge` ≈ greater than or equal.

Shorthands:

* `path` ≈ implements `AsRef<Path>`
* `reader` ≈ method `reader.read*()`
* `readee` ≈ function `read*(readee)`
* `matcher` ≈ such as `matcher.is_match(matchee)`
* `container` ≈ such as `container.contains(containee)`
* `set` ≈ a collection such as `::std::collections::BTreeSet`
* `bag` ≈ a collection such as `::std::collections::BTreeMap`

## Forms


### Forms for panic versus error

All the assert macros have 3 forms for different purposes:

* Panic form for typical tests.
* Debug form for debugging runtimes.
* Result form for runtime checks, verifications, validations, etc.

Examples:

* [`assert_starts_with!(a, b)`](https://docs.rs/assertables/latest/assertables/macro.assert_starts_with.html) // panic!
* [`debug_assert_starts_with!(a, b)`](https://docs.rs/assertables/latest/assertables/macro.debug_assert_starts_with.html) // panic! in debug mode
* [`assert_starts_with_as_result!(a, b)`](https://docs.rs/assertables/latest/assertables/macro.assert_starts_with_as_result.html) // return Ok or Err


### Forms for messages

All the assert macros have 2 forms for messages.

* Default message form.
* Custom message form.

Examples:

* [`assert_starts_with!(a, b)`](https://docs.rs/assertables/latest/assertables/macro.assert_starts_with.html)
* [`assert_starts_with!(a, b, "Your custom message here")`](https://docs.rs/assertables/latest/assertables/macro.assert_starts_with.html)


### Forms for other versus expression

Many of the assert macros have 2 forms for comparing left hand side and right hand side.

* Comparing a LHS item to a RHS other of the same type.
* Comparing a LHS item to a RHS expression.

Examples:

* [`assert_io_read_to_string_eq!(reader1, reader2)`](https://docs.rs/assertables/latest/assertables/macro.assert_io_read_to_string_eq.html)
* [`assert_io_read_to_string_eq_expr!(reader, expr)`](https://docs.rs/assertables/latest/assertables/macro.assert_io_read_to_string_eq_expr.html)


## Change highlights

8.3: Add `assert_poll_ready`, `assert_poll_pending`.

8.2: Add `assert_infix`.

8.1: Add `assert_result_ok`, `assert_result_err`, `assert_option_some`, `assert_option_none`.

8.0: Add `assert_fs_read_to_string_*`. Breaking change: rename `assert_read_to_string_*` macros to `assert_io_read_to_string_*`.

7.x: Add `assert_in_delta`, `assert_in_epsilon`. Add `assert_fn_*` macros with multiple arities.

6.x: Add `assert_starts_with`, `assert_ends_with`, `assert_contains`, `assert_is_match`. Add `debug_assert_*` macros everywhere.


## Tracking

* Package: assertables-rust-crate
* Version: 8.3.0
* Created: 2021-03-30T15:47:49Z
* Updated: 2024-09-07T22:22:42Z
* License: MIT or Apache-2.0 or GPL-2.0 or GPL-3.0 or contact us for more
* Contact: Joel Parker Henderson (joel@sixarm.com)
