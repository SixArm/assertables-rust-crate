//! # Assertables: Rust crate of assert macros for testing
//!
//! The `assertables` Rust crate provides many assert macros to improve your
//! compile-time tests and run-time reliability.
//!
//! Crate:
//! [https://crates.io/crates/assertables](https://crates.io/crates/assertables)
//!
//! Docs: [https://docs.rs/assertables/](https://docs.rs/assertables/)
//!
//! Repo:
//! [https://github.com/sixarm/assertables-rust-crate/](https://github.com/sixarm/assertables-rust-crate/)
//!
//!
//! ## Why use this?
//!
//! When you write Rust tests, then you can use Rust assert macros, such as:
//!
//! ```ignore
//! assert_eq!(value1, value2)
//! ```
//!
//! The assertables Rust crate provides many more assert macros for values,
//! strings, vectors, paths, readers, commands, and more, such as:
//!
//! ```ignore
//! assert_gt!(value1, value2); // value1 greater than value2
//! 
//! assert_starts_with!(string1, string2); // string1 starts with string2
//! 
//! assert_is_match!(regex, string); // regex is match of string
//! 
//! assert_set_subset!(vector1, vector2); // vector1 as set ⊆ vector2 as set
//! 
//! assert_fn_ok_eq!(function1, function2); // function1 ok = function2 ok
//! 
//! assert_fs_read_to_string_eq!(path1, path2); // path1 to string = path2 to string
//! 
//! assert_io_read_to_string_eq!(reader1, reader2); // reader1 to string = reader2 to string
//! 
//! assert_command_stdout_eq!(command1, command2); // command1 standard output = command2 standard output
//! ```
//!
//! See below for the complete list of all the assert macros.
//!
//!
//! ### Benefits
//!
//! * Your tests are more purposeful and powerful, which helps your code be more
//!   reliable.
//!
//! * Your assert failures provide more information, which helps you
//!   troubleshoot faster.
//!
//! * You gain runtime asserts, which helps you with validations and
//!   verifications.
//!
//! ### Features
//!
//! * Easy to use: each macro is well-documented with runnable examples and
//!   tests.
//!
//! * Zero overhead: if you don't use a macro, then it's never compiled into
//!   your code.
//!
//! * Three forms: `assert_*` for development, `debug_assert_*` for debugging,
//!   and `assert_*_as_result` for production.
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
//! Compare values by using nearness:
//!
//! * `assert_in_delta!(a, b, delta)` ≈ | a - b | ≤ delta
//!
//! * `assert_in_epsilon(a, b, epsilon)` ≈ | a - b | ≤ epsilon * min(a, b)
//!
//!
//! ### assert_* for strings and matchers
//!
//! These macros help with strings and also other structures that provide
//! matchers such as `starts_with`, `ends_width`, `contains`, and `is_match`.
//!
//! * `assert_starts_with(a, b)` ≈ a.starts_with(b)
//!
//! * `assert_not_starts_with(a, b)` ≈ !a.starts_with(b)
//!
//! * `assert_ends_with(a, b)` ≈ a.ends_with(b)
//!
//! * `assert_not_ends_with(a, b)` ≈ !a.ends_with(b)
//!
//! * `assert_contains(container, containee)` ≈ container.contains(containee)
//!
//! * `assert_not_contains(container, containee)` ≈
//!   !container.contains(containee)
//!
//! * `assert_is_match(matcher, matchee)` ≈ matcher.is_match(matchee)
//!
//! * `assert_not_match(matcher, matchee)` ≈ !matcher.is_match(matchee)
//!
//!
//! ### assert_set_* for set collection comparisons
//!
//! These macros help with comparison of set parameters, such as two arrays or
//! two vectors. where the item order does not matter, and the item count does
//! not matter. These macros convert their inputs into HashSet iterators.
//!
//! * `assert_set_eq!(a, b)` ≈ set a = set b
//!
//! * `assert_set_ne!(a, b)` ≈ set a ≠ set b
//!
//! * `assert_set_subset!(a, b)` ≈ set a ⊆ set b
//!
//! * `assert_set_superset!(a, b)` ≈ set a ⊇ set b
//!
//! * `assert_set_joint!(a, b)` ≈ set a ∩ set b ≠ ∅
//!
//! * `assert_set_disjoint!(a, b)` ≈ set a ∩ set b = ∅
//!
//!
//! ### assert_bag_* for bag collection comparisons
//!
//! These macros help with comparison of bag parameters, such as comparison of
//! two arrays or two vectors, where the item order does not matter, and the
//! item count does matter. These macros convert their inputs into HashMap
//! iterators.
//!
//! * `assert_bag_eq(a, b)` ≈ bag a = bag b
//!
//! * `assert_bag_ne(a, b)` ≈ bag a ≠ bag b
//!
//! * `assert_bag_subbag(a, b)` ≈ bag a ⊆ bag b
//!
//! * `assert_bag_superbag(a, b)` ≈ bag a ⊇ bag b
//!
//!
//! ### assert_fn_* for function return-value comparisons
//!
//! Compare a function with another function:
//!
//! * `assert_fn_eq!(function1, function2)` ≈ function1() = function2()
//!
//! * `assert_fn_ne!(function1, function2)` ≈ function1() ≠ function2()
//!
//! * `assert_fn_ge!(function1, function2)` ≈ function1() ≥ function2()
//!
//! * `assert_fn_gt!(function1, function2)` ≈ function1() > function2()
//!
//! * `assert_fn_le!(function1, function2)` ≈ function1() ≤ function2()
//!
//! * `assert_fn_lt!(function1, function2)` ≈ function1() < function2()
//!
//! Compare a function with an expression:
//!
//! * `assert_fn_eq_expr!(function, expr)` ≈ function() = expr
//!
//! * `assert_fn_ne_expr!(function, expr)` ≈ function() ≠ expr
//!
//! * `assert_fn_ge_expr!(function, expr)` ≈ function() ≥ expr
//!
//! * `assert_fn_gt_expr!(function, expr)` ≈ function() > expr
//!
//! * `assert_fn_le_expr!(function, expr)` ≈ function() ≤ expr
//!
//! * `assert_fn_lt_expr!(function, expr)` ≈ function() < expr
//!
//!
//! ### assert_fn_ok_* for Result Ok() comparisons
//!
//! Compare a function Ok() with another function Ok():
//!
//! * `assert_fn_ok_eq!(function1, function2)` ≈ function1().ok().unwrap() = function2().ok().unwrap()
//!
//! * `assert_fn_ok_ne!(function1, function2)` ≈ function1().ok().unwrap() ≠ function2().ok().unwrap()
//!
//! * `assert_fn_ok_ge!(function1, function2)` ≈ function1().ok().unwrap() ≥ function2().ok().unwrap()
//!
//! * `assert_fn_ok_gt!(function1, function2)` ≈ function1().ok().unwrap() > function2().ok().unwrap()
//!
//! * `assert_fn_ok_le!(function1, function2)` ≈ function1().ok().unwrap() ≤ function2().ok().unwrap()
//!
//! * `assert_fn_ok_lt!(function1, function2)` ≈ function1().ok().unwrap() < function2().ok().unwrap()
//!
//! Compare a function Ok() with an expression:
//!
//! * `assert_fn_ok_eq_expr!(function, expr)` ≈ function().ok().unwrap() = expr
//!
//! * `assert_fn_ok_ne_expr!(function, expr)` ≈ function().ok().unwrap() ≠ expr
//!
//! * `assert_fn_ok_ge_expr!(function, expr)` ≈ function().ok().unwrap() ≥ expr
//!
//! * `assert_fn_ok_gt_expr!(function, expr)` ≈ function().ok().unwrap() > expr
//!
//! * `assert_fn_ok_le_expr!(function, expr)` ≈ function().ok().unwrap() ≤ expr
//!
//! * `assert_fn_ok_lt_expr!(function, expr)` ≈ function().ok().unwrap() < expr
//!
//!
//! ### assert_fn_err_* for function Err() comparisons
//!
//! Compare a function Err() with another function Err():
//!
//! * `assert_fn_err_eq!(function1, function2)` ≈ function1().unwrap_err() = function2().unwrap_err()
//!
//! * `assert_fn_err_ne!(function1, function2)` ≈ function1().unwrap_err() ≠ function2().unwrap_err()
//!
//! * `assert_fn_err_ge!(function1, function2)` ≈ function1().unwrap_err() ≥ function2().unwrap_err()
//!
//! * `assert_fn_err_gt!(function1, function2)` ≈ function1().unwrap_err() > function2().unwrap_err()
//!
//! * `assert_fn_err_le!(function1, function2)` ≈ function1().unwrap_err() ≤ function2().unwrap_err()
//!
//! * `assert_fn_err_lt!(function1, function2)` ≈ function1().unwrap_err() < function2().unwrap_err()
//!
//! Compare a function Err() with an expression:
//!
//! * `assert_fn_err_eq!(function, expr)` ≈ function().unwrap_err() = expr
//!
//! * `assert_fn_err_ne!(function, expr)` ≈ function().unwrap_err() ≠ expr
//!
//! * `assert_fn_err_ge!(function, expr)` ≈ function().unwrap_err() ≥ expr
//!
//! * `assert_fn_err_gt!(function, expr)` ≈ function().unwrap_err() > expr
//!
//! * `assert_fn_err_le!(function, expr)` ≈ function().unwrap_err() ≤ expr
//!
//! * `assert_fn_err_lt!(function, expr)` ≈ function().unwrap_err() < expr
//!
//!
//! ### assert_fs_read_to_string_* for std::fs path comparisons
//!
//! These macros help with file system paths, such as disk files, `Path`, `PathBuf`,
//! the trait `AsRef<Path>`, and anything that is readable via
//! `std::fs::read_to_string(…)`.
//!
//! Compare a path with another path:
//!
//! * `assert_fs_read_to_string_eq!(path1, path2)` ≈ std::fs::read_to_string(path1) = std::fs::read_to_string(path2)
//!
//! * `assert_fs_read_to_string_ne!(path1, path2)` ≈ std::fs::read_to_string(path1) ≠ std::fs::read_to_string(path2)
//!
//! * `assert_fs_read_to_string_ge!(path1, path2)` ≈ std::fs::read_to_string(path1) ≥ std::fs::read_to_string(path2)
//!
//! * `assert_fs_read_to_string_gt!(path1, path2)` ≈ std::fs::read_to_string(path1) > std::fs::read_to_string(path2)
//!
//! * `assert_fs_read_to_string_le!(path1, path2)` ≈ std::fs::read_to_string(path1) ≤ std::fs::read_to_string(path2)
//!
//! * `assert_fs_read_to_string_lt!(path1, path2)` ≈ std::fs::read_to_string(path1) < std::fs::read_to_string(path2)
//!
//! Compare a path with an expression:
//!
//! * `assert_fs_read_to_string_eq_expr(path, expr)` ≈ std::fs::read_to_string(path) = expr
//!
//! * `assert_fs_read_to_string_ne_expr(path, expr)` ≈ std::fs::read_to_string(path) ≠ expr
//!
//! * `assert_fs_read_to_string_ge_expr(path, expr)` ≈ std::fs::read_to_string(path) ≥ expr
//!
//! * `assert_fs_read_to_string_gt_expr(path, expr)` ≈ std::fs::read_to_string(path) > expr
//!
//! * `assert_fs_read_to_string_le_expr(path, expr)` ≈ std::fs::read_to_string(path) ≤ expr
//!
//! * `assert_fs_read_to_string_lt_expr(path, expr)` ≈ std::fs::read_to_string(path) < expr
//!
//! Compare a path with its contents:
//!
//! * `assert_fs_read_to_string_contains(path, containee)` ≈ std::fs::read_to_string(path).contains(containee)
//!
//! * `assert_fs_read_to_string_matches(path, matcher)` ≈ matcher.is_match(std::fs::read_to_string(path))
//!
//!
//! ### assert_io_read_to_string_* for std::io::Read comparisons
//!
//! These macros help with readers, such as file handles, byte arrays, input
//! streams, and the trait std::io::Read.
//!
//! Compare a reader with another reader:
//!
//! * `assert_io_read_to_string_eq!(reader1, reader2)` ≈ reader1.read_to_string() = reader2.read_to_string()
//!
//! * `assert_io_read_to_string_ne!(reader1, reader2)` ≈ reader1.read_to_string() ≠ reader2.read_to_string()
//!
//! * `assert_io_read_to_string_ge!(reader1, reader2)` ≈ reader1.read_to_string() ≥ reader2.read_to_string()
//!
//! * `assert_io_read_to_string_gt!(reader1, reader2)` ≈ reader1.read_to_string() > reader2.read_to_string()
//!
//! * `assert_io_read_to_string_le!(reader1, reader2)` ≈ reader1.read_to_string() ≤ reader2.read_to_string()
//!
//! * `assert_io_read_to_string_lt!(reader1, reader2)` ≈ reader1.read_to_string() < reader2.read_to_string()
//!
//! Compare a reader with an expression:
//!
//! * `assert_io_read_to_string_eq_expr(reader, expr)` ≈ reader.read_to_string() = expr
//!
//! * `assert_io_read_to_string_ne_expr(reader, expr)` ≈ reader.read_to_string() ≠ expr
//!
//! * `assert_io_read_to_string_ge_expr(reader, expr)` ≈ reader.read_to_string() ≥ expr
//!
//! * `assert_io_read_to_string_gt_expr(reader, expr)` ≈ reader.read_to_string() > expr
//!
//! * `assert_io_read_to_string_le_expr(reader, expr)` ≈ reader.read_to_string() ≤ expr
//!
//! * `assert_io_read_to_string_lt_expr(reader, expr)` ≈ reader.read_to_string() < expr
//!
//! Compare a reader with its contents:
//!
//! * `assert_io_read_to_string_contains(reader, containee)` ≈ reader.read_to_string().contains(containee)
//!
//! * `assert_io_read_to_string_matches(reader, matcher)` ≈ matcher.is_match(reader.read_to_string())
//!
//!
//! ### assert_command_* for process command comparisons
//!
//! Compare command standard output string:
//!
//! * `assert_command_stdout_eq!(command1, command2)` ≈ command1 stdout = command2 stdout
//!
//! * `assert_command_stdout_eq_expr!(command, expr)` ≈ command stdout = expr
//!
//! * `assert_command_stdout_contains!(command, containee)` ≈ command stdout contains containee
//!
//! * `assert_command_stdout_is_match!(command, matcher)` ≈ command stdout is a matcher match
//!
//! Compare command standard error string:
//!
//! * `assert_command_stderr_eq!(command1, command2)` ≈ command1 stderr = command2 stderr
//!
//! * `assert_command_stderr_eq_expr!(command, expr)` ≈ command stderr = expr
//!
//! * `assert_command_stderr_contains!(command, containee)` ≈ command stderr contains containee
//!
//! * `assert_command_stderr_is_match!(command, matcher)` ≈ command stderr is a matcher match
//!
//!
//! ### assert_program_args_* for process command comparisons created via program name and args interator
//!
//! Compare command using program and arguments to standard output:
//!
//! * `assert_program_args_stdout_eq!(program1, args1, program2, args2)` ≈ command using program1 and args1 to stdout = command2 with program2 and args2 to stdout
//!
//! * `assert_program_args_stdout_eq_expr!(program, args, expr)` ≈ command using program and args to stdout = expr
//!
//! * `assert_program_args_stdout_contains!(program, args, containee)` ≈ command using program and args to stdout contains containee
//!
//! * `assert_program_args_stdout_is_match!(program, args, matcher)` ≈ matcher is match with command using program and args
//!
//! Compare command using program and arguments to standard output:
//!
//! * `assert_program_args_stderr_eq!(program1, args1, program2, args2)` ≈ command using program1 and args1 to stderr = command2 with program2 and args2 to stderr
//!
//! * `assert_program_args_stderr_eq_expr!(program, args, expr)` ≈ command using program and args to stderr = expr
//!
//! * `assert_program_args_stderr_contains!(program, args, containee)` ≈ command using program and args to stderr contains containee
//!
//! * `assert_program_args_stderr_is_match!(program, args, matcher)` ≈ matcher is match with command using program and args
//!
//!
//! ## Naming conventions
//!
//! Abbreviations:
//!
//! * `eq` ≈ equal
//!
//! * `ne` ≈ not equal.
//!
//! * `ge` ≈ greater than or equal.
//!
//! * `gt` ≈ greater than
//!
//! * `le` ≈ less than or equal.
//!
//! * `lt` ≈ less than
//!
//! Shorthands:
//!
//! * `path` ≈ implements `AsRef<Path>` such as `std::path::PathBuf`.
//!
//! * `reader` ≈ implements method `.read_to_string()` such as `std::io::Read`.
//!
//! * `matcher` ≈ implements `.is_match(…)` such as `regex::Regex`.
//!
//! * `containee` ≈ usable inside `.contains(…)` such as a `std::string::String`
//!   substring.
//!
//! * `set` ≈ a collection such as `::std::collections::BTreeSet`.
//!
//! * `bag` ≈ a collection such as `::std::collections::BTreeMap` which has key
//!   counts.
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
//! ## Changes summary
//!
//!
//! ### Version 8.x top changes
//!
//! * Add `assert_fs_read_to_string_*` macros for comparing files.
//!
//! * Rename `assert_read_to_string_*` macros to `assert_io_read_to_string_*`.
//!   If you use these macros, then please update your code to use the new
//!   naming convention.
//!
//!
//! ### Version 7.x top changes
//!
//! * Add `assert_in_delta`, `assert_in_epsilon`.
//!
//! * Add `assert_fn_*` macros with multiple arities.
//!
//! * Add `cargo release` for optimized tagged releases.
//!
//!
//! ### Version 6.x top changes
//!
//! * Add `assert_starts_with`, `assert_ends_with`, `assert_contains`,
//!   `assert_is_match`.
//!
//! * Add `debug_assert_*` macros everywhere.
//!
//! * Add `GPL-3.0` license.
//!
//!
//! ## Tracking
//!
//! * Package: assertables-rust-crate
//! * Version: 8.0.0
//! * Created: 2021-03-30T15:47:49Z
//! * Updated: 2024-09-02T15:45:13Z
//! * License: MIT or Apache-2.0 or GPL-2.0 or GPL-3.0 or contact us for more
//! * Contact: Joel Parker Henderson (joel@sixarm.com)

// Assert truth
pub mod assert; // (provided by Rust `std`)

// Assert value comparison
pub mod assert_eq; // (provided by Rust `std`)
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

// Assertable iterator-related set-based comparison
pub mod assert_set_disjoint;
pub mod assert_set_eq;
pub mod assert_set_joint;
pub mod assert_set_ne;
pub mod assert_set_subset;
pub mod assert_set_superset;

// Assertable iterator-related bag-based comparison
pub mod assert_bag_eq;
pub mod assert_bag_ne;
pub mod assert_bag_subbag;
pub mod assert_bag_superbag;

// Assert function return
pub mod assert_fn_eq;
pub mod assert_fn_eq_expr;
pub mod assert_fn_ge;
pub mod assert_fn_ge_expr;
pub mod assert_fn_gt;
pub mod assert_fn_gt_expr;
pub mod assert_fn_le;
pub mod assert_fn_le_expr;
pub mod assert_fn_lt;
pub mod assert_fn_lt_expr;
pub mod assert_fn_ne;
pub mod assert_fn_ne_expr;

// Assert function Ok()
pub mod assert_fn_ok_eq;
pub mod assert_fn_ok_eq_expr;
pub mod assert_fn_ok_ge;
pub mod assert_fn_ok_ge_expr;
pub mod assert_fn_ok_gt;
pub mod assert_fn_ok_gt_expr;
pub mod assert_fn_ok_le;
pub mod assert_fn_ok_le_expr;
pub mod assert_fn_ok_lt;
pub mod assert_fn_ok_lt_expr;
pub mod assert_fn_ok_ne;
pub mod assert_fn_ok_ne_expr;

// Assert function Err()
pub mod assert_fn_err_eq;
pub mod assert_fn_err_eq_expr;
pub mod assert_fn_err_ge;
pub mod assert_fn_err_ge_expr;
pub mod assert_fn_err_gt;
pub mod assert_fn_err_gt_expr;
pub mod assert_fn_err_le;
pub mod assert_fn_err_le_expr;
pub mod assert_fn_err_lt;
pub mod assert_fn_err_lt_expr;
pub mod assert_fn_err_ne;
pub mod assert_fn_err_ne_expr;

// Assert std::io read_to_string comparisons
pub mod assert_io_read_to_string_contains;
pub mod assert_io_read_to_string_eq;
pub mod assert_io_read_to_string_eq_expr;
pub mod assert_io_read_to_string_ge;
pub mod assert_io_read_to_string_ge_expr;
pub mod assert_io_read_to_string_gt;
pub mod assert_io_read_to_string_gt_expr;
pub mod assert_io_read_to_string_le;
pub mod assert_io_read_to_string_le_expr;
pub mod assert_io_read_to_string_lt;
pub mod assert_io_read_to_string_lt_expr;
pub mod assert_io_read_to_string_matches;
pub mod assert_io_read_to_string_ne;
pub mod assert_io_read_to_string_ne_expr;

// Assert std::fs read_to_string comparisons
pub mod assert_fs_read_to_string_contains;
pub mod assert_fs_read_to_string_eq;
pub mod assert_fs_read_to_string_eq_expr;
pub mod assert_fs_read_to_string_ge;
pub mod assert_fs_read_to_string_ge_expr;
pub mod assert_fs_read_to_string_gt;
pub mod assert_fs_read_to_string_gt_expr;
pub mod assert_fs_read_to_string_le;
pub mod assert_fs_read_to_string_le_expr;
pub mod assert_fs_read_to_string_lt;
pub mod assert_fs_read_to_string_lt_expr;
pub mod assert_fs_read_to_string_matches;
pub mod assert_fs_read_to_string_ne;
pub mod assert_fs_read_to_string_ne_expr;

// Assert command stdout
pub mod assert_command_stdout_eq;
pub mod assert_command_stdout_eq_expr;

// Assert command stdout specializations
pub mod assert_command_stdout_contains;
pub mod assert_command_stdout_is_match;

// Assert command stderr
pub mod assert_command_stderr_eq;
pub mod assert_command_stderr_eq_expr;

// Assert command stderr specializations
pub mod assert_command_stderr_contains;
pub mod assert_command_stderr_is_match;

// Assert program args stdout
pub mod assert_program_args_stdout_eq;
pub mod assert_program_args_stdout_eq_expr;
pub mod assert_program_args_stdout_ge;
pub mod assert_program_args_stdout_ge_expr;
pub mod assert_program_args_stdout_gt;
pub mod assert_program_args_stdout_gt_expr;
pub mod assert_program_args_stdout_le;
pub mod assert_program_args_stdout_le_expr;
pub mod assert_program_args_stdout_lt;
pub mod assert_program_args_stdout_lt_expr;
pub mod assert_program_args_stdout_ne;
pub mod assert_program_args_stdout_ne_expr;

// Assert program args stdout specializations
pub mod assert_program_args_stdout_contains;
pub mod assert_program_args_stdout_is_match;

// Assert program args stderr
pub mod assert_program_args_stderr_eq;
pub mod assert_program_args_stderr_eq_expr;
pub mod assert_program_args_stderr_ge;
pub mod assert_program_args_stderr_ge_expr;
pub mod assert_program_args_stderr_gt;
pub mod assert_program_args_stderr_gt_expr;
pub mod assert_program_args_stderr_le;
pub mod assert_program_args_stderr_le_expr;
pub mod assert_program_args_stderr_lt;
pub mod assert_program_args_stderr_lt_expr;
pub mod assert_program_args_stderr_ne;
pub mod assert_program_args_stderr_ne_expr;

// Assert program args stderr specializations
pub mod assert_program_args_stderr_contains;
pub mod assert_program_args_stderr_is_match;
