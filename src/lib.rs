//! # Assertables: Rust crate of assert macros for testing
//! 
//! The `assertables` Rust crate provides many assert macros to improve your
//! compile-time tests and run-time reliability.
//! 
//! * Crate: [https://crates.io/crates/assertables](https://crates.io/crates/assertables)
//! * Docs: [https://docs.rs/assertables/](https://docs.rs/assertables/)
//! * Repo: [https://github.com/sixarm/assertables-rust-crate/](https://github.com/sixarm/assertables-rust-crate/)
//! * Contact: [joel@joelparkerhenderson.com](mailto:joel@joelparkerhenderson.com)
//! 
//! 
//! ## Introduction
//! 
//! The Rust programming language provides a few built-in assert macros to test code:
//! 
//! The Rust programming language provides a few built-in assert macros to test code:
//! 
//! * `assert!()`
//! * `assert_eq!(a, b)`
//! * `assert_ne!(a, b)`
//! 
//! The assertables crate provides many more, so you can write smarter tests.
//! 
//! For values:
//! 
//! * [`assert_gt!(a, b)`](macro@crate::assert_gt)
//! * [`assert_lt!(a, b)`](macro@crate::assert_lt)
//! 
//! For numbers:
//! 
//! * [`assert_in_delta!(a, b, delta)`](macro@crate::assert_in_delta)
//! * [`assert_in_epsilon!(a, b, epsilon)`](macro@crate::assert_in_epsilon)
//! 
//! For strings:
//! 
//! * [`assert_starts_with!(a, b)`](macro@crate::assert_starts_with)
//! * [`assert_ends_with!(a, b)`](macro@crate::assert_ends_with)
//! 
//! For matching:
//! 
//! * [`assert_contains!(a, b)`](macro@crate::assert_contains)
//! * [`assert_is_match!(a, b)`](macro@crate::assert_is_match)
//! 
//! For infix numeric operators and infix logical operators:
//!
//! * [`assert_infix!(a == b)`](macro@crate::assert_infix)
//! * [`assert_infix!(a && b)`](macro@crate::assert_infix)
//! 
//! For maybes:
//! 
//! * [`assert_result_ok!(a)`](macro@crate::assert_result_ok)
//! * [`assert_option_some!(a)`](macro@crate::assert_option_some)
//!    
//! For collections such as arrays, vectors, maps, sets:
//! 
//! * [`assert_set_subset!(a, b)`](macro@crate::assert_set_subset)
//! * [`assert_set_disjoint!(a, b)`](macro@crate::assert_set_disjoint)
//! 
//! For file system paths and input/output readers:
//! 
//! * [`assert_fs_read_to_string_eq!(path1, path2)`](macro@crate::assert_fs_read_to_string_eq)
//! * [`assert_io_read_to_string_eq!(reader1, reader2)`](macro@crate::assert_io_read_to_string_eq)
//! 
//! For command capture of standard output and standard error:
//! 
//! * [`assert_command_stdout_eq!(command1, command2)`](macro@crate::assert_command_stdout_eq);
//! * [`assert_program_args_stdout_eq!(program1, args1, program2, args2`](macro@crate::assert_program_args_stdout_eq);
//! 
//! There are many more macros that are grouped into modules.
//! 
//! Modules for enums:
//! 
//! * [`assert_option`](module@crate::assert_option) for `Option` {`Some`, `None`}
//! 
//! * [`assert_result`](module@crate::assert_result) for `Result` {`Ok`, `Err`}
//! 
//! Modules for collections, such as arrays, vectors, lists, maps:
//! 
//! * [`assert_set`](module@crate::assert_set) for set collections
//! 
//! * [`assert_bag`](module@crate::assert_bag) for bag collections
//! 
//! Modules for functions:
//! 
//! * [`assert_fn`](module@crate::assert_fn) for functions in general.
//! 
//! * [`assert_fn_ok`](module@crate::assert_fn_ok) for functions that return Result::Ok.
//! 
//! * [`assert_fn_err`](module@crate::assert_fn_err) for functions that return Result::Err.
//! 
//! Modules for readers:
//! 
//! * [`assert_fs_read_to_string`](module@crate::assert_fs_read_to_string) for file system path contents.
//! 
//! * [`assert_io_read_to_string`](module@crate::assert_io_read_to_string) for input/output reader streams.
//! 
//! Modules for external calls:
//! 
//! * [`assert_command`](module@crate::assert_command) for commands and their stdout & stderr.
//! 
//! * [`assert_program_args`](module@crate::assert_program_args) for programs with arguments and their stdout & stderr.
//! 
//! 
//! ### Benefits
//! 
//! * Your tests are more purposeful and powerful. This helps your code be more
//! reliable.
//! 
//! * Your assert failures provide more information. This helps you
//! troubleshoot faster.
//! 
//! * You gain runtime asserts. This helps you with validations and
//! verifications.
//! 
//! 
//! ### Features
//! 
//! * Easy to use: each macro is well-documented with runnable examples and
//! tests.
//! 
//! * Zero overhead: if you don't use a macro, then it's never compiled into
//! your code.
//! 
//! * Zero dependencies: the crate has no release dependencies, and just a short list of development dependencies.
//! 
//! 
//! ### Naming conventions
//! 
//! Abbreviations:
//! 
//! * `eq` ≈ equal
//! 
//! * `ne` ≈ not equal.
//! 
//! * `lt` ≈ less than
//! 
//! * `le` ≈ less than or equal.
//! 
//! * `gt` ≈ greater than
//! 
//! * `ge` ≈ greater than or equal.
//! 
//! Types:
//! 
//! * `path` ≈ implements `AsRef<Path>` such as `std::path::PathBuf`.
//! 
//! * `reader` ≈ implements method `.read_to_string()` such as `std::io::Read`.
//! 
//! * `matcher` ≈ implements `.is_match(…)` such as `regex::Regex`.
//! 
//! * `containee` ≈ usable inside `.contains(…)` such as a
//!   `std::string::String` substring.
//! 
//! * `set` ≈ a collection such as `::std::collections::BTreeSet`.
//! 
//! * `bag` ≈ a collection such as `::std::collections::BTreeMap` which has
//!   key counts.
//! 
//! 
//! ## Forms
//! 
//! 
//! ### Forms for panic! versus Err()
//! 
//! All the assert macros have three forms that you can use depending on your goals.
//! 
//! Panic form for typical tests:
//! 
//! * [`assert_starts_with!(a, b)`](macro@crate::assert_starts_with)
//! 
//! Debug form for runtime:
//! 
//! * [`debug_assert_starts_with!(a, b)`](macro@crate::debug_assert_starts_with)
//! 
//! Result form for runtime, validation, verification, sanitization, and more:
//! 
//! * [`assert_starts_with_as_result!(a, b)`](macro@crate::assert_starts_with_as_result)
//! 
//! 
//! ### Forms for messages
//! 
//! All the assert macros have 2 forms that are for default messages versus custom messages.
//! 
//! Default message form:
//! 
//! * [`assert_starts_with!(a, b)`](macro@crate::assert_starts_with)
//! 
//! Custom message form:
//!
//! * [`assert_starts_with!(a, b, "Your custom message here")`](macro@crate::assert_starts_with)
//! 
//! 
//! ### Forms for comparing an other versus an expression
//! 
//! Many of the assert macros have 2 forms that are for comparing a value to an other versus to an expression.
//! 
//! Compare the item to an other i.e. of the same type:
//! 
//! * [`assert_io_read_to_string_eq!(reader1, reader2)`](macro@crate::assert_io_read_to_string_eq)
//!
//! Compare an item to an expression:
//! 
//! * [`assert_io_read_to_string_eq_expr!(reader, expr)`](macro@crate::assert_io_read_to_string_eq_expr)
//! 
//! 
//! ## Change highlights
//! 
//! 
//! ### Version 8
//! 
//! 8.2:
//! 
//! * Add `assert_infix`
//! 
//! * Add modules for better discoverability and testability.
//! 
//! 8.1:
//! 
//! * Add Result macros `assert_result_ok` and `assert_result_err`
//! 
//! * Add Option macros `assert_option_some` and `assert_option_none`
//! 
//! 8.0:
//! 
//! * Add `assert_fs_read_to_string_*` macros for comparing files.
//! 
//! * Breaking change: rename `assert_read_to_string_*` macros to `assert_io_read_to_string_*`. If you use these macros, then please update your code to use the new naming convention.
//! 
//! 
//! ### Version 7
//! 
//! * Add `assert_in_delta`, `assert_in_epsilon`.
//! 
//! * Add `assert_fn_*` macros with multiple arities.
//! 
//! * Add `cargo release` for optimized tagged releases.
//! 
//! 
//! ### Version 6
//! 
//! * Add `assert_starts_with`, `assert_ends_with`, `assert_contains`, `assert_is_match`.
//! 
//! * Add `debug_assert_*` macros everywhere.
//! 
//! * Add `GPL-3.0` license.
//! 
//! 
//! ## Tracking
//! 
//! * Package: assertables-rust-crate
//! * Version: 8.2.1
//! * Created: 2021-03-30T15:47:49Z
//! * Updated: 2024-09-07T12:31:17Z
//! * License: MIT or Apache-2.0 or GPL-2.0 or GPL-3.0 or contact us for more
//! * Contact: Joel Parker Henderson (joel@sixarm.com)

// Assert truth
pub mod assert; // (in addition to what's provided by Rust `std`)

// Assert value comparison
pub mod assert_eq; // (in addition to what's provided by Rust `std`)
pub mod assert_ge;
pub mod assert_gt;
pub mod assert_le;
pub mod assert_lt;
pub mod assert_ne;

// Assert value nearness
pub mod assert_in_delta;
pub mod assert_in_epsilon;
// Assert value matching
pub mod assert_contains;
pub mod assert_ends_with;
pub mod assert_is_match;
pub mod assert_not_contains;
pub mod assert_not_ends_with;
pub mod assert_not_match;
pub mod assert_not_starts_with;
pub mod assert_starts_with;

// For maybes
pub mod assert_result;
pub mod assert_option;

// For collections
pub mod assert_set;
pub mod assert_bag;

// For functions
pub mod assert_fn;
pub mod assert_fn_ok;
pub mod assert_fn_err;

// For reading
pub mod assert_fs_read_to_string;
pub mod assert_io_read_to_string;

// For externals
pub mod assert_command;
pub mod assert_program_args;

// Experimental - work in progress - unsupported
pub mod assert_infix;