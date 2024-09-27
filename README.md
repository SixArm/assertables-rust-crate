# Assertables: Rust crate of assert macros for testing

The `assertables` Rust crate provides many assert macros to improve your
compile-time tests and run-time reliability.

* Crate: [https://crates.io/crates/assertables](https://crates.io/crates/assertables)
* Docs: [https://docs.rs/assertables/](https://docs.rs/assertables/)
* Repo: [https://github.com/sixarm/assertables-rust-crate/](https://github.com/sixarm/assertables-rust-crate/)
* Contact: [joel@joelparkerhenderson.com](mailto:joel@joelparkerhenderson.com)


## Introduction

The Rust programming language provides assert macros to test code:

* [`assert!()`](https://doc.rust-lang.org/std/macro.assert.html)
* [`assert_eq!(a, b)`](https://doc.rust-lang.org/std/macro.assert_eq.html) `// equal`
* [`assert_ne!(a, b)`](https://doc.rust-lang.org/std/macro.assert_ne.html) `// not equal`

The assertables crate provides many more, so you can write smarter tests.

For values:

* [`assert_lt!(a, b)`](https://docs.rs/assertables/8.9.0/assertables/macro.assert_lt.html) `// less than`
* [`assert_le!(a, b)`](https://docs.rs/assertables/8.9.0/assertables/macro.assert_le.html) `// less than or equal`
* [`assert_gt!(a, b)`](https://docs.rs/assertables/8.9.0/assertables/macro.assert_gt.html) `// greater than`
* [`assert_ge!(a, b)`](https://docs.rs/assertables/8.9.0/assertables/macro.assert_ge.html) `// greater than or equal`

For approximation:

* [`assert_approx_eq!(a, b)`](https://docs.rs/assertables/8.9.0/assertables/macro.assert_approx_eq.html)
* [`assert_approx_ne!(a, b)`](https://docs.rs/assertables/8.9.0/assertables/macro.assert_approx_ne.html)

For strings:

* [`assert_starts_with!(a, b)`](https://docs.rs/assertables/8.9.0/assertables/macro.assert_starts_with.html)
* [`assert_ends_with!(a, b)`](https://docs.rs/assertables/8.9.0/assertables/macro.assert_ends_with.html)

For matching:

* [`assert_contains!(a, b)`](https://docs.rs/assertables/8.9.0/assertables/macro.assert_contains.html)
* [`assert_is_match!(a, b)`](https://docs.rs/assertables/8.9.0/assertables/macro.assert_is_match.html)

For infix operators:

* [`assert_infix!(a == b)`](https://docs.rs/assertables/8.9.0/assertables/macro.assert_infix.html)
* [`assert_infix!(a && b)`](https://docs.rs/assertables/8.9.0/assertables/macro.assert_infix.html)

For nearness:

* [`assert_in_delta!(a, b, delta)`](https://docs.rs/assertables/8.9.0/assertables/macro.assert_in_delta.html)
* [`assert_in_epsilon!(a, b, epsilon)`](https://docs.rs/assertables/8.9.0/assertables/macro.assert_in_epsilon.html)

For Result Ok/Err:

* [`assert_ok!(a)`](https://docs.rs/assertables/8.9.0/assertables/macro.assert_ok.html)
* [`assert_ok_eq!(a, b)`](https://docs.rs/assertables/8.9.0/assertables/macro.assert_ok_eq.html)
* [`assert_ok_eq_expr!(a, expr)`](https://docs.rs/assertables/8.9.0/assertables/macro.assert_ok_eq_expr.html)
* [`assert_ok_ne!(a, b)`](https://docs.rs/assertables/8.9.0/assertables/macro.assert_ok_ne.html)
* [`assert_ok_ne_expr!(a, expr)`](https://docs.rs/assertables/8.9.0/assertables/macro.assert_ok_ne_expr.html)
* [`assert_err!(a)`](https://docs.rs/assertables/8.9.0/assertables/macro.assert_err.html)

For Option Some/None:

* [`assert_some!(a)`](https://docs.rs/assertables/8.9.0/assertables/macro.assert_some.html)
* [`assert_some_eq!(a, b)`](https://docs.rs/assertables/8.9.0/assertables/macro.assert_some_eq.html)
* [`assert_some_eq_expr!(a, expr)`](https://docs.rs/assertables/8.9.0/assertables/macro.assert_some_eq_expr.html)
* [`assert_some_ne!(a, b)`](https://docs.rs/assertables/8.9.0/assertables/macro.assert_some_ne.html)
* [`assert_some_ne_expr!(a, expr)`](https://docs.rs/assertables/8.9.0/assertables/macro.assert_some_ne_expr.html)
* [`assert_none!(a)`](https://docs.rs/assertables/8.9.0/assertables/macro.assert_none.html)

For Poll Ready/Pending:

* [`assert_ready!(a)`](https://docs.rs/assertables/8.9.0/assertables/macro.assert_ready.html)
* [`assert_ready_eq!(a, b)`](https://docs.rs/assertables/8.9.0/assertables/macro.assert_ready_eq.html)
* [`assert_ready_eq_expr!(a, expr)`](https://docs.rs/assertables/8.9.0/assertables/macro.assert_ready_eq_expr.html)
* [`assert_ready_ne!(a, b)`](https://docs.rs/assertables/8.9.0/assertables/macro.assert_ready_ne.html)
* [`assert_ready_ne_expr!(a, expr)`](https://docs.rs/assertables/8.9.0/assertables/macro.assert_ready_ne_expr.html)
* [`assert_pending!(a)`](https://docs.rs/assertables/8.9.0/assertables/macro.assert_pending.html)

For collections such as arrays, vectors, maps, sets:

* [`assert_set_subset!(collection1, collection2)`](https://docs.rs/assertables/8.9.0/assertables/macro.assert_set_subset.html)
* [`assert_set_disjoint!(collection1, collection2)`](https://docs.rs/assertables/8.9.0/assertables/macro.assert_set_disjoint.html)

For file system paths and input/output readers:

* [`assert_fs_read_to_string_eq!(path1, path2)`](https://docs.rs/assertables/8.9.0/assertables/macro.assert_fs_read_to_string_eq.html)
* [`assert_io_read_to_string_eq!(reader1, reader2)`](https://docs.rs/assertables/8.9.0/assertables/macro.assert_io_read_to_string_eq.html)

For command capture of standard output and standard error:

* [`assert_command_stdout_eq!(command1, command2)`](https://docs.rs/assertables/8.9.0/assertables/macro.assert_command_stdout_eq.html)
* [`assert_program_args_stdout_eq!(program1, args1, program2, args2)`](https://docs.rs/assertables/8.9.0/assertables/macro.assert_program_args_stdout_eq.html)

There are many more macros that are grouped into modules.

Modules for collections, such as arrays, vectors, lists, maps:

* [`assert_set`](https://docs.rs/assertables/8.9.0/assertables/assert_set) for set collections
* [`assert_bag`](https://docs.rs/assertables/8.9.0/assertables/assert_bag) for bag collections

Modules for functions:

* [`assert_fn`](https://docs.rs/assertables/8.9.0/assertables/assert_fn) for functions in general.
* [`assert_fn_ok`](https://docs.rs/assertables/8.9.0/assertables/assert_fn_ok) for functions that return `Result::Ok`.
* [`assert_fn_err`](https://docs.rs/assertables/8.9.0/assertables/assert_fn_err) for functions that return `Result::Err`.

Modules for readers:

* [`assert_fs_read_to_string`](https://docs.rs/assertables/8.9.0/assertables/assert_fs_read_to_string) for file system path contents.
* [`assert_io_read_to_string`](https://docs.rs/assertables/8.9.0/assertables/assert_io_read_to_string) for input/output reader streams.

Modules for external calls:

* [`assert_command`](https://docs.rs/assertables/8.9.0/assertables/assert_command) for commands with stdout/stderr.
* [`assert_program_args`](https://docs.rs/assertables/8.9.0/assertables/assert_program_args) for programs with args with stdout/stderr.


### Top 3 benefits

* You can write better tests that improve reliability and maintainability.

* You can troubleshoot faster because error messages show more specifics.

* You can leverage runtime asserts for validations and verifications.


### Top 3 features

* Easy to learn: everything is well-documented with runnable examples.

* Zero overhead: if you don't use a macro, then it's never compiled.

* Zero dependencies in production: no chain and no delegated risks.


## Forms


### Forms for panic versus error

All the assert macros have 3 forms for different purposes:

* Panic form for typical tests.
* Debug form for debugging runtimes.
* Result form for runtime checks, verifications, validations, etc.

Examples:

* [`assert_starts_with!(a, b)`](https://docs.rs/assertables/8.9.0/assertables/macro.assert_starts_with.html) // panic!
* [`debug_assert_starts_with!(a, b)`](https://docs.rs/assertables/8.9.0/assertables/macro.debug_assert_starts_with.html) // panic! in debug mode
* [`assert_starts_with_as_result!(a, b)`](https://docs.rs/assertables/8.9.0/assertables/macro.assert_starts_with_as_result.html) // return Ok or Err


### Forms for messages

All the assert macros have 2 forms for messages.

* Default message form.
* Custom message form.

Examples:

* [`assert_starts_with!(a, b)`](https://docs.rs/assertables/8.9.0/assertables/macro.assert_starts_with.html)
* [`assert_starts_with!(a, b, "Your custom message here")`](https://docs.rs/assertables/8.9.0/assertables/macro.assert_starts_with.html)


### Forms for other versus expression

Many of the assert macros have 2 forms for comparing left hand side and right hand side.

* Comparing a LHS item to a RHS other of the same type.
* Comparing a LHS item to a RHS expression.

Examples:

* [`assert_io_read_to_string_eq!(reader1, reader2)`](https://docs.rs/assertables/8.9.0/assertables/macro.assert_io_read_to_string_eq.html)
* [`assert_io_read_to_string_eq_expr!(reader, expr)`](https://docs.rs/assertables/8.9.0/assertables/macro.assert_io_read_to_string_eq_expr.html)


## Tracking

* Package: assertables-rust-crate
* Version: 8.10.0
* Created: 2021-03-30T15:47:49Z
* Updated: 2024-09-27T18:25:43Z
* License: MIT or Apache-2.0 or GPL-2.0 or GPL-3.0 or contact us for more
* Contact: Joel Parker Henderson (joel@sixarm.com)
