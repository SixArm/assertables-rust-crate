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
* [`assert_option_some!(a)`](macro@crate::assert_option_some)

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

* [`assert_option`](https://docs.rs/assertables/latest/assertables/macro.assert_option.html)for `Option` {`Some`, `None`}

* [`assert_result`](https://docs.rs/assertables/latest/assertables/macro.assert_result.html) for `Result` {`Ok`, `Err`}

Modules for collections, such as arrays, vectors, lists, maps:

* [`assert_set`](https://docs.rs/assertables/latest/assertables/macro.assert_set.html) for set collections

* [`assert_bag`](https://docs.rs/assertables/latest/assertables/macro.assert_bag.html) for bag collections

Modules for functions:

* [`assert_fn`](https://docs.rs/assertables/latest/assertables/macro.assert_fn.html) for functions in general.

* [`assert_fn_ok`](https://docs.rs/assertables/latest/assertables/macro.assert_ok.html) for functions that return `Result::Ok`.

* [`assert_fn_err`](https://docs.rs/assertables/latest/assertables/macro.assert_fn_err.html) for functions that return `Result::Err`.

Modules for readers:

* [`assert_fs_read_to_string`](https://docs.rs/assertables/latest/assertables/macro.assert_fs_read_to_string.html) for file system path contents.

* [`assert_io_read_to_string`](https://docs.rs/assertables/latest/assertables/macro.assert_io_read_to_string.html) for input/output reader streams.

Modules for external calls:

* [`assert_command`](https://docs.rs/assertables/latest/assertables/macro.assert_command.html) for commands and their stdout & stderr.

* [`assert_program_args`](https://docs.rs/assertables/latest/assertables/macro.assert_program_args.html) for programs with arguments and their stdout & stderr.


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

Types:

* `path` ≈ implements `AsRef<Path>` such as `std::path::PathBuf`.

* `reader` ≈ implements method `.read_to_string()` such as `std::io::Read`.

* `matcher` ≈ implements `.is_match(…)` such as `regex::Regex`.

* `containee` ≈ usable inside `.contains(…)` such as a
  `std::string::String` substring.

* `set` ≈ a collection such as `::std::collections::BTreeSet`.

* `bag` ≈ a collection such as `::std::collections::BTreeMap` which has
  key counts.


## Forms


### Forms for panic! versus Err()

All the assert macros have 3 forms that you can use depending on your goals.

Panic form for typical tests:

* [`assert_starts_with!(a, b)`](https://docs.rs/assertables/latest/assertables/macro.assert_starts_with.html)

Debug form for runtime:

* [`debug_assert_starts_with!(a, b)`](https://docs.rs/assertables/latest/assertables/macro.debug_assert_starts_with.html)

Result form for runtime, validation, verification, sanitization, and more:

* [`assert_starts_with_as_result!(a, b)`](https://docs.rs/assertables/latest/assertables/macro.assert_starts_with_as_result.html); // return Result Ok(()) or Err(…), for any runtime


### Forms for messages

All the assert macros have 2 forms that are for default messages versus custom messages.

Default message form:

* [`assert_starts_with!(a, b)`](https://docs.rs/assertables/latest/assertables/macro.assert_starts_with.html)

Custom message form:

* [`assert_starts_with!(a, b, "Your custom message here")`](https://docs.rs/assertables/latest/assertables/macro.assert_starts_with.html)


### Forms for comparing an other versus an expression

Many of the assert macros have 2 forms that are for comparing an item to an other of the same type versus to an expression.

Compare an item to an other i.e. of the same type:
 
* [`assert_io_read_to_string_eq!(reader1, reader2)`](https://docs.rs/assertables/latest/assertables/macro.assert_io_read_to_string_eq.html)

Compare an item to an expression:

* [`assert_io_read_to_string_eq_expr!(reader, expr)`](https://docs.rs/assertables/latest/assertables/macro.assert_io_read_to_string_eq_expr.html)


## Change highlights

8.2:

* Add `assert_infix`

* Add submodules with smoke tests for better documentability.

8.1:

* Add Result macros `assert_result_ok` and `assert_result_err`

* Add Option macros `assert_option_some` and `assert_option_none`

8.0:

* Add `assert_fs_read_to_string_*` macros for comparing files.

* Breaking change: rename `assert_read_to_string_*` macros to `assert_io_read_to_string_*`. If you use these macros, then please update your code to use the new naming convention.

7.x:

* Add `assert_in_delta`, `assert_in_epsilon`.

* Add `assert_fn_*` macros with multiple arities.

* Add `cargo release` for optimized tagged releases.

6.x:

* Add `assert_starts_with`, `assert_ends_with`, `assert_contains`, `assert_is_match`.

* Add `debug_assert_*` macros everywhere.

* Add `GPL-3.0` license.


## Tracking

* Package: assertables-rust-crate
* Version: 8.2.1
* Created: 2021-03-30T15:47:49Z
* Updated: 2024-09-07T12:31:17Z
* License: MIT or Apache-2.0 or GPL-2.0 or GPL-3.0 or contact us for more
* Contact: Joel Parker Henderson (joel@sixarm.com)
