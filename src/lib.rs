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
//!   [assert_in](module@crate::assert_in),
//!   […](https://docs.rs/assertables)
//! * Test groups with
//!   [assert_all](module@crate::assert_all),
//!   [assert_any](module@crate::assert_any),
//!   [assert_iter](module@crate::assert_iter),
//!   […](https://docs.rs/assertables)
//! * Test wrappers with
//!   [assert_ok](module@crate::assert_ok),
//!   [assert_some](module@crate::assert_some),
//!   [assert_ready](module@crate::assert_ready),
//!   […](https://docs.rs/assertables)
//! * Test matching with
//!   [assert_matches](module@crate::assert_matches),
//!   [assert_is_match](module@crate::assert_is_match),
//!   […](https://docs.rs/assertables)
//! * Test nearness with
//!   [assert_approx](module@crate::assert_approx),
//!   [assert_abs_diff](module@crate::assert_abs_diff),
//!   […](https://docs.rs/assertables/)
//! * Test programs with
//!   [assert_command](module@crate::assert_command),
//!   [assert_status](module@crate::assert_status),
//!   […](https://docs.rs/assertables)
//! * Many more below.
//!
//! To use this crate, add it to your file `Cargo.toml`:
//!
//! ```toml
//! assertables = "9.5.0"
//! ``````
//!
//! Benefits:
//!
//! * You will write better tests to improve reliability and maintainability.
//! * You will handle more corner cases without needing to write custom code.
//! * You will troubleshoot faster because error messages show more detail.
//!
//! Learning:
//! [FAQ](https://github.com/SixArm/assertables-rust-crate/tree/main/help/faq),
//! [docs](https://docs.rs/assertables/),
//! [examples](https://github.com/SixArm/assertables-rust-crate/blob/main/tests/examples/),
//! [changes](https://github.com/SixArm/assertables-rust-crate/tree/main/CHANGES.md),
//! [upgrades](https://github.com/SixArm/assertables-rust-crate/tree/main/help/upgrades/upgrade-from-version-8-to-9),
//! [developing](https://github.com/SixArm/assertables-rust-crate/tree/main/help/developing/).
//!
//! Comparisons:
//! [more_asserts](https://github.com/SixArm/assertables-rust-crate/tree/main/help/comparisons/more_asserts),
//! [cool_asserts](https://github.com/SixArm/assertables-rust-crate/tree/main/help/comparisons/cool_asserts),
//! [assert2](https://github.com/SixArm/assertables-rust-crate/tree/main/help/comparisons/assert2),
//! [claims](https://github.com/SixArm/assertables-rust-crate/tree/main/help/comparisons/claims),
//! [etc.](https://github.com/SixArm/assertables-rust-crate/tree/main/help/comparisons)
//!
//! ## Examples
//!
//! Examples with numbers:
//!
//! ```rust
//! # use assertables::*;
//! let i = 1;
//! assert_lt!(i, 5);
//! assert_in_range!(i, 1..5);
//! assert_abs_diff_eq!(i, 5, 4);
//! ```
//!
//! Examples with strings:
//!
//! ```rust
//! # use assertables::*;
//! # use regex::Regex;
//! let s = "hello";
//! assert_starts_with!(s, "h");
//! assert_contains!(s, "e");
//! assert_is_match!(Regex::new(r"h.*o").unwrap(), s);
//! ```
//!
//! Examples with arrays:
//!
//! ```rust
//! # use assertables::*;
//! let a = [1, 2, 3];
//! assert_not_empty!(a);
//! assert_len_eq_x!(a, 3);
//! assert_all!(a.into_iter(), |i: i32| i < 4);
//! ```
//!
//! ## Highlights
//!
//! Values:
//!
//! * [`assert_eq!(a, b)`](module@crate::assert_eq) ≈ a = b ≈ equal to
//! * [`assert_ne!(a, b)`](module@crate::assert_ne) ≈ a ≠ b ≈ not equal to
//! * [`assert_lt!(a, b)`](module@crate::assert_lt) ≈ a < b ≈ less than
//! * [`assert_le!(a, b)`](module@crate::assert_le) ≈ a ≤ b ≈ less than or equal to
//! * [`assert_gt!(a, b)`](module@crate::assert_gt) ≈ a > b ≈ greater than
//! * [`assert_ge!(a, b)`](module@crate::assert_ge) ≈ a ≥ b ≈ greater than or equal to
//!
//! Nearness:
//!
//! * [`assert_approx_eq!(a, b)`](module@crate::assert_approx::assert_approx_eq) ≈ |a-b| ≤ 1e-6
//! * [`assert_diff_eq_x!(a, b, x)`](module@crate::assert_diff::assert_diff_eq_x) ≈ (b-a) = x
//! * [`assert_abs_diff_eq_x!(a, b, x)`](module@crate::assert_abs_diff::assert_abs_diff_eq_x) ≈ |b-a| = x
//! * [`assert_in_delta!(a, b, delta)`](module@crate::assert_in::assert_in_delta) ≈ |a-b| ≤ Δ
//! * [`assert_in_epsilon!(a, b, epsilon)`](module@crate::assert_in::assert_in_epsilon) ≈ |a-b| ≤ ε min(a,b)
//! * [`assert_in_range!(a, range)`](module@crate::assert_in::assert_in_range) ≈ range.contains(a)
//!
//! Groups:
//!
//! * [`assert_all!(group, predicate)`](module@crate::assert_all) ≈ group.all(predicate)
//! * [`assert_any!(group, predicate)`](module@crate::assert_any) ≈ group.any(predicate)
//! * [`assert_is_empty!(group)`](module@crate::assert_is_empty::assert_is_empty) ≈ a.is_empty()
//! * [`assert_len_eq!(a, b)`](module@crate::assert_len::assert_len_eq) ≈ a.len() = b.len()
//! * [`assert_count_eq!(a, b)`](module@crate::assert_count::assert_count_eq) ≈ a.count() = b.count()
//!
//! Matching:
//!
//! * [`assert_starts_with!(sequence, x)`](module@crate::assert_starts_with) ≈ sequence.starts_with(x)
//! * [`assert_ends_with!(sequence, x)`](module@crate::assert_ends_with) ≈ sequence.ends_with(x)
//! * [`assert_contains!(container, x)`](module@crate::assert_contains) ≈ container.contains(x)
//! * [`assert_is_match!(matcher, x)`](module@crate::assert_is_match) ≈ matcher.is_match(x)
//! * [`assert_matches!(expr, pattern)`](module@crate::assert_matches) ≈ matches!(expr, pattern)
//!
//! Results:
//!
//! * [`assert_ok!(a)`](module@crate::assert_ok) ≈ a is Ok
//! * [`assert_err!(a)`](module@crate::assert_err) ≈ a is Err
//! * [`assert_ok_eq_x!(a, x)`](module@crate::assert_ok::assert_ok_eq_x) ≈ a is Ok ⇒ unwrap = x
//!
//! Options:
//!
//! * [`assert_some!(a)`](module@crate::assert_some) ≈ a is Some
//! * [`assert_none!(a)`](module@crate::assert_none) ≈ a is None
//! * [`assert_some_eq_x!(a, x)`](module@crate::assert_some::assert_some_eq_x) ≈ a is Some ⇒ unwrap = x
//!
//! Polls:
//!
//! * [`assert_ready!(a)`](module@crate::assert_ready) ≈ a is Ready
//! * [`assert_pending!(a)`](module@crate::assert_pending) ≈ a is Pending
//! * [`assert_ready_eq_x!(a, x)`](module@crate::assert_ready::assert_ready_eq_x) ≈ a is Ready ⇒ unwrap = x
//!
//! Collections:
//!
//! * [`assert_iter_eq!(a, b)`](module@crate::assert_iter) ≈ a into iter = b into iter
//! * [`assert_set_eq!(a, b)`](module@crate::assert_set) ≈ a into set = b into set
//! * [`assert_bag_eq!(a, b)`](module@crate::assert_bag) ≈ a into bag = = b into bag
//!
//! Readers:
//!
//! * [`assert_fs_read_to_string_eq_x!(path, x)`](module@crate::assert_fs_read_to_string) ≈ path ⇒ file ⇒ string = x
//! * [`assert_io_read_to_string_eq_x!(reader, x)`](module@crate::assert_io_read_to_string) ≈ reader ⇒ bytes ⇒ string = x
//!
//! Commands:
//!
//! * [`assert_command_stdout_eq_x!(command, x)`](module@crate::assert_command) `// command ⇒ stdout == x`
//! * [`assert_program_args_stdout_eq_x!(program, args, x)`](module@crate::assert_program_args) `// program.args ⇒ stdout == x`
//!
//! Status:
//!
//! * [`assert_status_success!(a)`](module@crate::assert_status::assert_status_success) ≈ a.status().success()
//! * [`assert_status_code_value_eq_x!(a, x)`](module@crate::assert_status::assert_status_code_value_eq_x) ≈ a.status().code().unwrap() = x
//!
//! Infix:
//!
//! * [`assert_infix!(a == b)`](module@crate::assert_infix) ≈ order operators == != < <= > >=
//! * [`assert_infix!(a && b)`](module@crate::assert_infix) ≈ logic operators && || ^ & |
//!
//! For a complete list of modules and macros, see the
//! [docs](https://docs.rs/assertables/).
//!
//!
//! ## Forms
//!
//! The Assertables macros have a variety of forms to help you write the tests that matter most to you.
//!
//! All the macros have forms for an optional message:
//!
//! * [`assert_gt!(a, b)`](module@crate::assert_gt) ≈ default message
//! * [`assert_gt!(a, b, "your text")`](module@crate::assert_gt) ≈ custom message
//!
//! All the macros have forms for different outcomes:
//!
//! * [`assert_gt!(1, 2)`](macro@crate::assert_gt) ≈ panic
//! * [`assert_gt_as_result!(1, 2)`](macro@crate::assert_gt_as_result) ≈ Result Err
//! * [`debug_assert_gt!(1, 2)`](macro@crate::debug_assert_gt) ≈ panic in debug mode
//!
//! Many of the macros have a form "compare left item to right item" that compares
//! items of the same kind, and a form "compare left item to right expression" that
//! compares one item to any arbitrary expression:
//!
//! * [`assert_len_eq!(a, b)`](module@crate::assert_ok::assert_ok_eq) ≈ a.len() = b.len()
//! * [`assert_len_eq_x!(a, x)`](module@crate::assert_ok::assert_ok_eq_x)) ≈ a.len() = x
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
//! * Version: 9.5.1
//! * Created: 2021-03-30T15:47:49Z
//! * Updated: 2024-11-05T16:40:19Z
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
pub mod assert_diff;
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
pub mod assert_status;

// Misc
pub mod assert_success;
