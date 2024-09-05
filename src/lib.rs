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
//! ```ignore
//! assert!()
//! assert_eq!(a, b)
//! assert_ne!(a, b)
//! ```
//! 
//! The assertables crate provides many more, so you can write smarter tests.
//! 
//! For values:
//! 
//! ```ignore
//! assert_gt!(a, b)
//! assert_lt!(a, b)
//! ```
//! 
//! For numbers:
//! 
//! ```ignore
//! assert_in_delta!(a, b, delta)
//! assert_in_epsilon!(a, b, epsilon)
//! ```
//! 
//! For strings:
//! 
//! ```ignore
//! assert_starts_with!(a, b)
//! assert_ends_with!(a, b)
//! ```
//! 
//! For matching:
//! 
//! ```ignore
//! assert_contains!(a, b)
//! assert_is_match!(a, b)
//! ```
//! 
//! For collections such as arrays, vectors, maps, sets:
//! 
//! ```ignore
//! assert_set_subset!(a, b)
//! assert_set_disjoint!(a, b)
//! ```
//! 
//! For file system paths and input/output readers:
//! 
//! ```ignore
//! assert_fs_read_to_string_eq!(path1, path2)
//! assert_io_read_to_string_eq!(reader1, reader2)
//! ```
//! 
//! For command capture of standard output and standard error:
//! 
//! ```ignore
//! assert_command_stdout_eq!(command1 stdout = command2 stdout);
//! assert_command_stderr_eq!(command1, command2);
//! ```
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
//! ### Forms
//! 
//! Assertables macros come in three forms:
//! 
//! * Panic macro: `assert_*` is for typical test uses with `cargo test`.
//! 
//! * Debug macro: `debug_assert_*` is for runtime diagnostic configuration.
//! 
//! * Result macro:  `assert_*_as_result` is for runtime production configuration, such as for site reliability engineering, chaos engineering, validations, verifications, sanitizations, and more.
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
//! 
//! Shorthands:
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
//! ## Complete list of assert macros
//! 
//! 
//! ### assert_* for values
//! 
//! Compare values:
//! 
//! * `assert_eq!(a, b)` ≈ a = b
//! 
//! * `assert_ne!(a, b)` ≈ a ≠ b
//! 
//! * `assert_ge!(a, b)` ≈ a ≥ b
//! 
//! * `assert_gt!(a, b)` ≈ a > b
//! 
//! * `assert_le!(a, b)` ≈ a ≤ b
//! 
//! * `assert_lt!(a, b)` ≈ a < b
//! 
//! 
//! ## For infix operators
//! 
//! Compare values by using an infix value operator:
//! 
//! * `assert_infix!(a == b)` ≈ a == b
//! 
//! * `assert_infix!(a != b)` ≈ a ≠ b
//! 
//! * `assert_infix!(a < b)` ≈ a < b
//! 
//! * `assert_infix!(a <= b)` ≈ a ≤ b
//! 
//! * `assert_infix!(a > b)` ≈ a > b
//! 
//! * `assert_infix!(a >= b)` ≈ a ≥ b
//! 
//! Relate values by using an infix logical operator:
//! 
//! * `assert_infix!(a & b)` ≈ a ∧ b ≈ a AND b
//! 
//! * `assert_infix!(a | b)` ≈ a ∨ b ≈ a OR b
//! 
//! * `assert_infix!(a ^ b)` ≈ a ⊻ b ≈ a XOR b
//! 
//! * `assert_infix!(a && b)` ≈ a …∧ b ≈ a lazy AND b
//! 
//! * `assert_infix!(a || b)` ≈ a …∨ b ≈ a lazy OR b
//! 
//! 
//! ### For nearness
//! 
//! Compare values by using nearness math conventions:
//! 
//! * `assert_in_delta!(a, b, delta)` ≈ | a - b | ≤ delta
//! 
//! * `assert_in_epsilon(a, b, epsilon)` ≈ | a - b | ≤ epsilon * min(a, b)
//! 
//! 
//! ### For strings
//! 
//! These macros help with strings and also other structures that provide
//! matchers such as `starts_with`, `ends_width`, `contains`, and `is_match`.
//! Each macro also has a corresponding `not` version.
//! 
//! * `assert_starts_with(a, b)` ≈ a.starts_with(b)
//! 
//! * `assert_ends_with(a, b)` ≈ a.ends_with(b)
//! 
//! * `assert_contains(container, containee)` ≈ container.contains(containee)
//! 
//! * `assert_is_match(matcher, matchee)` ≈ matcher.is_match(matchee)
//! 
//! 
//! ### For much more
//! 
//! There are many more macros that are conveniently grouped into modules.
//! 
//! For enums:
//! 
//! * [`assert_option`] for `Option` (`Some`, `None`)
//! 
//! * [`assert_result`] for `Result` (`Ok`, `Err`)
//! 
//! For collections, such as arrays, vectors, lists, maps:
//! 
//! * [`assert_set`] for set collections
//! 
//! * [`assert_bag`] for bag collections
//! 
//! For functions:
//! 
//! * [`assert_fn`] for functions in general.
//! 
//! * [`assert_fn_ok`] for functions that return Result::Ok.
//! 
//! * [`assert_fn_err`] for functions that return Result::Err.
//! 
//! For readers:
//! 
//! * [`assert_fs_read_to_string`] for file system path contents.
//! 
//! * [`assert_io_read_to_string`] for input/output reader streams.
//! 
//! For external calls:
//! 
//! * [`assert_command`] for commands and their stdout & stderr.
//! 
//! * [`assert_program_args`] for programs with args and their stdout & stderr.
//! 
//! 
//! ## Forms
//! 
//! 
//! ### Forms for panic! versus Err()
//! 
//! The assert macros have three forms that you can use depending on your goals:
//! 
//! 
//! ```ignore
//! assert_gt!(a, b); // return () or panic!(…), for typical compile-time testing
//! 
//! debug_assert_gt!(a, b); // return () or panic!(…), for a non-optimized runtime
//! 
//! assert_gt_as_result!(a, b); // return Result Ok(()) or Err(…), for any runtime
//! ```
//! 
//! 
//! ### Forms for messages
//! 
//! The assert macros have forms for default messages versus custom messages.
//! 
//! ```ignore
//! assert_gt!(1, 2); // panic!("assertion failed: assert_gt(1, 2)…")
//! 
//! assert_gt!(1, 2, "message"); // panic!("message")
//! ```
//! 
//! 
//! ### Forms for comparing an other versus an expression
//! 
//! Some assert macros have forms for comparing an other versus an expression:
//! 
//! ```ignore
//! assert_io_read_to_string_eq!(reader1, reader2); // reader1.read_to_string() = reader2.read_to_string()
//! 
//! assert_io_read_to_string_eq_expr!(reader, expr); // reader1.read_to_string() = expr
//! ```
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
//! * Refactor into submodules for better discoverability and testability.
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
//! * Version: 8.2.0
//! * Created: 2021-03-30T15:47:49Z
//! * Updated: 2024-09-04T20:21:53Z
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