//! # Assertables: Rust crate of assert macros for testing
//!
//! Assertables is a Rust crate that provides many assert macros
//! to improve your compile-time tests and run-time reliability.
//!
//! * Crate: [https://crates.io/crates/assertables](https://crates.io/crates/assertables)
//! * Docs: [https://docs.rs/assertables/](https://docs.rs/assertables/)
//! * Repo: [https://github.com/sixarm/assertables-rust-crate/](https://github.com/sixarm/assertables-rust-crate/)
//! * Contact: [joel@joelparkerhenderson.com](mailto:joel@joelparkerhenderson.com)
//!
//!
//! ## Introduction
//!
//! The Assertables Rust crate provides many assert macros
//! that can help you develop, test, and debug.
//!
//! * Test values with
//!   [assert_lt](module@crate::assert_lt),
//!   [assert_gt](module@crate::assert_gt),
//!   [etc.](https://docs.rs/assertables/)
//! * Test strings with
//!   [assert_starts_with](module@crate::assert_starts_with),
//!   [assert_ends_with](module@crate::assert_ends_with),
//!   [etc.](https://docs.rs/assertables/)
//! * Test groups with
//!   [assert_all](module@crate::assert_all),
//!   [assert_any](module@crate::assert_any),
//!   [etc.](https://docs.rs/assertables/)
//!
//! There are many more for
//!   [results](https://docs.rs/assertables/9.1.0/assertables/assert_result),
//!   [options](https://docs.rs/assertables/9.1.0/assertables/assert_option),
//!   [polls](https://docs.rs/assertables/9.1.0/assertables/assert_poll),
//!   [matches](https://docs.rs/assertables/9.1.0/assertables/assert_matches),
//!   [iterators](https://docs.rs/assertables/9.1.0/assertables/assert_iter),
//!   [sets](https://docs.rs/assertables/9.1.0/assertables/assert_set),
//!   [files](https://docs.rs/assertables/9.1.0/assertables/assert_fs_read_to_string),
//!   [bytes](https://docs.rs/assertables/9.1.0/assertables/assert_io_read_to_string),
//!   [commands](https://docs.rs/assertables/9.1.0/assertables/assert_command),
//!   [etc.](https://docs.rs/assertables/)
//!
//! To use this crate, add it to your file `Cargo.toml`:
//!
//! ```toml
//! assertables = "9.1.0"
//! ``````
//!
//! Top benefits:
//!
//! 1. You can write better tests to improve reliability and maintainability.
//! 2. You can handle more corner cases without needing to write custom code.
//! 3. You can troubleshoot faster because error messages show specifics.
//!
//! Top features:
//!
//! 1. Easy to use: everything is well-documented with runnable examples.
//! 2. Zero overhead: if you don't use a macro, then it's not compiled.
//! 3. Multiple forms: for debugging, for results, and for success returns.
//!
//! Help:
//!
//! * [Documentation](https://docs.rs/assertables/)
//! * [Frequently asked questions](https://github.com/SixArm/assertables-rust-crate/tree/main/help/faq)
//! * [Examples](https://github.com/SixArm/assertables-rust-crate/blob/main/tests/examples/)
//! * [Upgrade from version 8 to 9](https://github.com/SixArm/assertables-rust-crate/tree/main/help/upgrades/upgrade-from-version-8-to-9)
//! * [Comparisons to more_asserts, cool_asserts, assert2, claims, etc.](https://github.com/SixArm/assertables-rust-crate/tree/main/help/comparisons)
//!
//! ## Highlights
//!
//! Values:
//!
//! * [`assert_eq!(a, b)`](module@crate::assert_eq) `// a == b`
//! * [`assert_ne!(a, b)`](module@crate::assert_ne) `// a != b`
//! * [`assert_lt!(a, b)`](module@crate::assert_lt) `// a < b`
//! * [`assert_le!(a, b)`](module@crate::assert_le) `// a <= b`
//! * [`assert_gt!(a, b)`](module@crate::assert_gt) `// a > b`
//! * [`assert_ge!(a, b)`](module@crate::assert_ge) `// a >= b`
//!
//! Differences:
//!
//! * [`assert_approx_eq!(a, b)`](module@crate::assert_approx::assert_approx_eq) `// |a-b| <= 1e-6`
//! * [`assert_abs_diff_eq!(a, b, delta)`](module@crate::assert_abs_diff::assert_abs_diff_eq) `// |a-b| == Δ`
//! * [`assert_in_delta!(a, b, delta)`](module@crate::assert_in::assert_in_delta) `// |a-b| <= Δ`
//! * [`assert_in_epsilon!(a, b, epsilon)`](module@crate::assert_in::assert_in_epsilon) `// |a-b| <= ε min(a,b)`
//!
//! Groups for iterators, chars, etc.:
//!
//! * [`assert_all!(group, predicate)`](module@crate::assert_all) `// group.all(predicate)`
//! * [`assert_any!(group, predicate)`](module@crate::assert_any) `// group.any(predicate)`
//!
//! Infix for order operators, logic operators, etc.:
//!
//! * [`assert_infix!(a == b)`](module@crate::assert_infix) `// order operators: == != < <= > >=`
//! * [`assert_infix!(a && b)`](module@crate::assert_infix) `// logic operators: && || ^ & |`
//!
//! Lengths and counts for strings, vectors, iterators, etc.:
//!
//! * [`assert_len_eq!(item, x)`](module@crate::assert_len::assert_len_eq) `// item.len() == x`
//! * [`assert_count_eq!(item, x)`](module@crate::assert_count::assert_count_eq) `// item.count() == x`
//! * [`assert_is_empty!(item)`](module@crate::assert_is_empty::assert_is_empty) `// item.is_empty()`
//!
//! Matching:
//!
//! * [`assert_starts_with!(whole, part)`](module@crate::assert_starts_with) `// whole.starts_with(part)`
//! * [`assert_ends_with!(whole, part)`](module@crate::assert_ends_with) `// whole.ends_with(part)`
//! * [`assert_contains!(container, x)`](module@crate::assert_contains) `// container.contains(x)`
//! * [`assert_matches!(expr, pattern)`](module@crate::assert_matches) `// matches!(expr, pattern)`
//! * [`assert_is_match!(matcher, x)`](module@crate::assert_is_match) `// matcher.is_match(x)`
//!
//! Results:
//!
//! * [`assert_ok!(a)`](module@crate::assert_ok) `// a is Ok`
//! * [`assert_ok_eq_x!(a, 1)`](module@crate::assert_ok::assert_ok_eq_x) `// a is Ok(1)`
//! * [`assert_err!(a)`](module@crate::assert_err) `// a is Err`
//!
//! Options:
//!
//! * [`assert_some!(a)`](module@crate::assert_some) `// a is Some`
//! * [`assert_some_eq_x!(a, 1)`](module@crate::assert_some::assert_some_eq_x) `// a is Some(1)`
//! * [`assert_none!(a)`](module@crate::assert_none) `// a is None`
//!
//! Polls:
//!
//! * [`assert_ready!(a)`](module@crate::assert_ready) `// a is Ready`
//! * [`assert_ready_eq_x!(a, 1)`](module@crate::assert_ready::assert_ready_eq_x) `// a is Ready(1)`
//! * [`assert_pending!(a)`](module@crate::assert_pending) `// a is Pending`
//!
//! Readers:
//!
//! * [`assert_fs_read_to_string_eq!(a_path, b_path)`](module@crate::assert_fs_read_to_string) `// read a_path == read b_path`
//! * [`assert_io_read_to_string_eq!(a_bytes, b_bytes)`](module@crate::assert_io_read_to_string) `// read a_bytes == read b_bytes`
//!
//! Commands:
//!
//! * [`assert_command_stdout_eq!(a_command, b_command)`](module@crate::assert_command) `// a_command stdout == b_command stdout`
//! * [`assert_program_args_stderr_eq!(a_program, a_args, b_program, b_args)`](module@crate::assert_program_args) `// a_program a_args stderr == b_program b_args stderr`
//!
//! Collections:
//!
//! * [`assert_iter_eq!(a, b)`](module@crate::assert_iter) `// a into iter == b into iter`
//! * [`assert_set_eq!(a, b)`](module@crate::assert_set) `// a into set == b into set`
//! * [`assert_bag_eq!(a, b)`](module@crate::assert_bag) `// a into bag == = b into bag`
//!
//! For a complete list of modules and macros, see the
//! [docs](https://docs.rs/assertables/)
//!
//! ## Forms
//!
//! The Assertables macros have a variety of forms to help you write the tests that matter most to you.
//!
//! All the macros have forms for an optional message:
//!
//! * [`assert_gt!(a, b)`](module@crate::assert_gt) `// default message`
//! * [`assert_gt!(a, b, "your text")`](module@crate::assert_gt) `// custom message`
//!
//! All the macros have forms for different outcomes:
//!
//! * [`assert_gt!(a, b)`](macro@crate::assert_gt) `// panic`
//! * [`assert_gt_as_result!(a, b)`](macro@crate::assert_gt_as_result) `// return Result, no panic`
//! * [`debug_assert_gt!(a, b)`](macro@crate::debug_assert_gt) `// special use in debug mode`
//!
//! Many of the macros have a "solo" form for comparing one item to an expression, and a "pair" form for comparing two items to each other:
//!
//! * [`assert_ok_eq!(a, x)`](module@crate::assert_ok::assert_ok_eq)) `// a.unwrap() == x`
//! * [`assert_ok_eq!(a, b)`](module@crate::assert_ok::assert_ok_eq) `// a.unwrap() == b.unwrap()`
//!
//! Many of the macros has a "success return", which means the macro returns data that you can optionally use for more testing.
//!
//! * [`let inner = assert_ok!(result)`](module@crate::assert_ok::assert_ok)
//! * [`let string = assert_fs_read_to_string_ne!("alfa.txt", "")`](module@crate::assert_fs_read_to_string::assert_fs_read_to_string_ne)
//! * [`let stdout = assert_command_stdout_gt!("ls", vec![b' '])`](module@crate::assert_command::assert_command_stdout_gt)
//!
//! ## Tracking
//!
//! * Package: assertables-rust-crate
//! * Version: 9.1.0
//! * Created: 2021-03-30T15:47:49Z
//! * Updated: 2024-10-29T20:21:37Z
//! * License: MIT or Apache-2.0 or GPL-2.0 or GPL-3.0 or contact us for more
//! * Contact: Joel Parker Henderson (joel@joelparkerhenderson.com)

// Assert truth
pub mod assert; // (in addition to what's provided by Rust `std`)

// Assert value comparison
pub mod assert_eq; // (in addition to what's provided by Rust `std`)
pub mod assert_ge;
pub mod assert_gt;
pub mod assert_le;
pub mod assert_lt;
pub mod assert_ne; // (in addition to what's provided by Rust `std`)

// Assert difference
pub mod assert_abs_diff;
pub mod assert_approx;
pub mod assert_in;

// Assert all/any
pub mod assert_all;
pub mod assert_any;

// Infix
pub mod assert_infix;

// Matching
pub mod assert_contains;
pub mod assert_count;
pub mod assert_ends_with;
pub mod assert_is_empty;
pub mod assert_is_match;
pub mod assert_len;
pub mod assert_matches;
pub mod assert_starts_with;

// For Result Ok & Err
pub mod assert_err;
pub mod assert_ok;
pub mod assert_result; // Deprecated

// For Option Some & None
pub mod assert_none;
pub mod assert_option;
pub mod assert_some; // Deprecated

// For Poll Ready & Pending
pub mod assert_pending;
pub mod assert_poll;
pub mod assert_ready; // Deprecated

// For collections
pub mod assert_bag;
pub mod assert_iter;
pub mod assert_set;

// For functions
pub mod assert_fn;
pub mod assert_fn_err;
pub mod assert_fn_ok;

// For reading
pub mod assert_fs_read_to_string;
pub mod assert_io_read_to_string;

// For externals
pub mod assert_command;
pub mod assert_program_args;
