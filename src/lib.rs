//! # Assertables: Rust crate of assert macros for testing
//!
//! The `assertables` Rust crate provides many assert macros to improve your
//! compile-time tests and run-time reliability.
//!
//! * Crate:
//!   [https://crates.io/crates/assertables](https://crates.io/crates/assertables)
//! * Docs: [https://docs.rs/assertables/](https://docs.rs/assertables/)
//! * Repo:
//!   [https://github.com/sixarm/assertables-rust-crate/](https://github.com/sixarm/assertables-rust-crate/)
//! * Contact:
//!   [joel@joelparkerhenderson.com](mailto:joel@joelparkerhenderson.com)
//!
//!
//! ## Introduction
//!
//! The Rust programming language provides assert macros such as `assert!(x)` to
//! test code. The assertables crate provides many more for numbers, strings,
//! results, options, iterators, files, streams, and more. See below for details.
//!
//! Top benefits:
//!
//! 1. You can write better tests to improve reliability and maintainability.
//! 2. You can handle more corner cases without needing to write custom code.
//! 3. You can troubleshoot faster because error messages show specifics.
//!
//! To use this crate, add it to your file `Cargo.toml`:
//!
//! ```toml
//! assertables = "9.0.0"
//! ``````
//!
//! Help:
//!
//! * [Upgrade version 8 to 9](https://github.com/SixArm/assertables-rust-crate/tree/main/help/upgrades/upgrade-from-version-8-to-9)
//! * [Comparison examples](https://github.com/SixArm/assertables-rust-crate/blob/main/tests/comparison_examples.rs)
//! * [Validation examples](https://github.com/SixArm/assertables-rust-crate/blob/main/tests/validation_examples.rs)
//! * [Similar crates such as more_asserts, cool_asserts, assert2, claims](https://github.com/SixArm/assertables-rust-crate/tree/main/help/comparisons)
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
//! Approximations:
//!
//! * [`assert_approx_eq!(a, b)`](module@crate::assert_approx::assert_approx_eq) `// |a-b| <= 1e-6`
//! * [`assert_in_delta!(a, b, delta)`](module@crate::assert_in::assert_in_delta) `// |a-b| <= delta`
//! * [`assert_in_epsilon!(a, b, epsilon)`](module@crate::assert_in::assert_in_epsilon) `// |a-b| <= epsilon * min(a,b)`
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
//! Parts for strings, vectors, etc.:
//!
//! * [`assert_starts_with!(whole, part)`](module@crate::assert_starts_with) `// whole.starts_with(part)`
//! * [`assert_ends_with!(whole, part)`](module@crate::assert_ends_with) `// whole.ends_with(part)`
//!
//! Lengths and counts for strings, vectors, iterators, etc.:
//!
//! * [`assert_len_eq!(item, x)`](module@crate::assert_len::assert_len_eq) `// item.len() == x`
//! * [`assert_count_eq!(item, x)`](module@crate::assert_count::assert_count_eq) `// item.count() == x`
//! * [`assert_is_empty!(item)`](module@crate::assert_is_empty::assert_is_empty) `// item.is_empty()`
//!
//! Matching for strings, regex, etc.:
//!
//! * [`assert_matches!(expr, pattern)`](module@crate::assert_matches) `// matches!(expr, pattern)`
//! * [`assert_is_match!(matcher, x)`](module@crate::assert_is_match) `// matcher.is_match(x)`
//! * [`assert_contains!(container, x)`](module@crate::assert_contains) `// container.contains(x)`
//!
//! Collections for arrays, vectors, iterators, sets, maps:
//!
//! * [`assert_iter_eq2!(arr1, arr2)`](module@crate::assert_iter) `// eq ne etc.`
//! * [`assert_set_eq2!(vec1, vec2)`](module@crate::assert_set) `// eq ne etc.`
//! * [`assert_bag_eq2!(map1, map2)`](module@crate::assert_bag) `// eq ne etc.`
//!
//! Result Ok & Err:
//!
//! * [`assert_ok_eq!(result, x)`](module@crate::assert_ok) `// result is Ok(x)`
//! * [`assert_err_eq!(result, x)`](module@crate::assert_err) `// result is Err(x)`
//!
//! Option Some & None:
//!
//! * [`assert_some_eq!(option, x)`](module@crate::assert_some) `// option is Some(x)`
//! * [`assert_none!(option)`](module@crate::assert_none) `// option is None`
//!
//! Poll Ready & Pending:
//!
//! * [`assert_ready_eq!(poll, x)`](module@crate::assert_ready) `// poll is Ready(x)`
//! * [`assert_pending!(poll)`](module@crate::assert_pending) `// poll is Pending`
//!
//! Read file system paths and input/output streams:
//!
//! * [`assert_fs_read_to_string_eq!(path, x)`](module@crate::assert_fs_read_to_string) `// read path == x`
//! * [`assert_io_read_to_string_eq!(bytes, x)`](module@crate::assert_io_read_to_string) `// read bytes == x`
//!
//! Run processes and use stdout or stderr:
//!
//! * [`assert_command_stdout_eq!(command, x)`](module@crate::assert_command) `// command stdout == x`
//! * [`assert_program_args_stderr_eq!(program, args, x)`](module@crate::assert_program_args) `// program-args stderr == x`
//!
//! For a complete list of modules and macros, see the [docs](https://docs.rs/assertables/)
//!
//! ## Forms
//!
//! All the macros have forms for an optional message:
//!
//! * [`assert_gt!(a, b)`](macro@crate::assert_gt) `// automatic message`
//! * [`assert_gt!(a, b, "Your text")`](macro@crate::assert_gt) `// custom message`
//!
//! All the macros have forms for different outcomes:
//!
//! * [`assert_gt!(a, b)`](macro@crate::assert_gt) `// panic during typical test`
//! * [`assert_gt_as_result!(a, b)`](macro@crate::assert_gt_as_result) `// return Ok or Err`
//! * [`debug_assert_gt!(a, b)`](macro@crate::debug_assert_gt) `// panic when in debug mode`
//!
//! Many of the macros have forms for comparing one item (to an expression)
//! or two items (to each other):
//!
//! * [`assert_ok_eq!(a, b)`](macro@crate::assert_ok_eq) `// Ok(…) = expression`
//! * [`assert_ok_eq2!(a, b)`](macro@crate::assert_ok_eq2) `// Ok(…) = Ok(…)`
//!
//! Many of the macros return more information upon success,
//! in case you want to do further testing or tracing, such as:
//!
//! * `let inner = assert_ok!(result); //=> result.unwrap()`
//! * `let string = assert_fs_read_to_string_ne!("alfa.txt", ""); //=> file contents`
//! * `let stdout = assert_command_stdout_gt!("ls", vec![b' ']); //=> process handle`
//!
//! ## Tracking
//!
//! * Package: assertables-rust-crate
//! * Version: 9.0.0
//! * Created: 2021-03-30T15:47:49Z
//! * Updated: 2024-10-23T20:21:04Z
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
pub mod assert_ne; // (in addition to what's provided by Rust `std`)

// Assert value nearness
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

// For Result Ok/Err
pub mod assert_err;
pub mod assert_ok;
pub mod assert_result; // Deprecated

// For Option Some/None
pub mod assert_none;
pub mod assert_option;
pub mod assert_some; // Deprecated

// For Poll Ready/Pending
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
