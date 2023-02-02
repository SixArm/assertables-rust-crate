//! # Assertables: Rust crate of assert macros for testing
//!
//! The `assertables` Rust crate provides many assert macros
//! to help with compile-time testing and run-time reliability
//!
//! Crate:
//! [https://crates.io/crates/assertables](https://crates.io/crates/assertables)
//!
//! Docs:
//! [https://docs.rs/assertables/](https://docs.rs/assertables/)
//!
//! Repo:
//! [https://github.com/sixarm/assertables-rust-crate/](https://github.com/sixarm/assertables-rust-crate/)
//!
//!
//! ## Highlights
//!
//! Macros for values such as:
//!
//! ```ignore
//! let x = 1;
//! let y = 2;
//! assert_gt!(x, y); // x is greater than y
//! ```
//!
//! Macros for strings, matchers, patterns, etc. such as:
//!
//! ```ignore
//! let x = "example";
//! let y = "ex"
//! assert_starts_with!(x, y); // x starts with y
//! ```
//!
//! Macros for arrays, vectors, sets, bags, etc., such as:
//!
//! ```ignore
//! let x = [1, 2];
//! let y = [1, 2, 3];
//! assert_set_subset!(x, y); // x is a subset of y
//! ```
//!
//! Macros for functions such as:
//!
//! ```ignore
//! let x = -1;
//! let y = 1;
//! assert_fn_eq!(abs, x, y); // function(input) == output
//! ```
//!
//! Macros for readers, streams, etc. such as:
//!
//! ```ignore
//! let mut reader = "alpha".as_bytes();
//! let expect = String::from("alpha");
//! assert_read_to_string_eq!(reader, expect); // reader read to string == expect
//! ```
//!
//! Macros for commands, arguments, and CLIs, such as:
//!
//! ```ignore
//! let mut command = Command::new("printf");
//! command.args(["%s", "hello"]);
//! let expect = "hello";
//! assert_command_stdout_eq!(command, expect); // command standard output == expect
//! ```
//!
//!
//! ## Version 6.x notable improvements
//!
//! * Add `assert_starts_with`, `assert_ends_with`, `assert_contains`, `assert_match`.
//!
//! * Add `debug_assert_*` macros everywhere.
//!
//! * Add many documentation examples.
//!
//! * Add GPL3 license.
//!
//!
//! ## Naming conventions
//!
//! Abbreviations:
//!
//! * `eq` means equal
//!
//! * `ne` means not equal.
//!
//! * `lt` means less than
//!
//! * `le` means less than or equal.
//!
//! * `gt` means greater than
//!
//! * `ge` means greater than or equal.
//!
//! Shorthands:
//!
//! * `reader` means implements `.read_to_string(…)` such as `std::io::Read`.
//!
//! * `matcher` means implements `.is_match(…)` such as `regex::Regex`.
//!
//! * `containee` means usable inside `.contains(…)` such as a `std::string::String` substring.
//!
//! * `set` means a collection such as `::std::collections::BTreeSet`.
//!
//! * `bag` means a collection such as `::std::collections::BTreeMap` which has key counts.
//!
//!
//! ## Forms for panic! versus Err()
//!
//! The macros have forms for immediate interrupts versus returning results:
//!
//! ```ignore
//! assert_gt!(a, b); // return () or panic!(…)
//!
//! assert_gt_as_result!(a, b); // return Result Ok(()) or Err(…)
//! ```
//!
//!
//! ## Forms for default messages versus custom messages
//!
//! The macros have forms for default messages versus custom messages.
//!
//! Examples with panics:
//!
//! ```ignore
//! assert_gt!(1, 2); // panic!("assertion failed: assert_gt(1, 2)…")
//!
//! assert_gt!(1, 2, "message"); // panic!("message")
//! ```
//!
//! Examples with errors:
//!
//! ```ignore
//! assert_gt_as_result!(1, 2); // return Err("assertion failed: assert_gt(1, 2)…")
//!
//! assert_gt_as_result!(1, 2, "message"); // return Err("message")
//! ```
//!
//!
//! ## Forms for comparing an expression versus an equivalent
//!
//! Some macros have forms for comparing to an expression (`expr`) versus an equivalent (`other`):
//!
//! ```ignore
//! assert_read_to_string_eq!(reader, expr); // reader.read_to_string() == expr
//!
//! assert_read_to_string_eq_other!(reader1, reader2); // reader1.read_to_string() == reader2.read_to_string()
//! ```
//!
//!
//! ## assert_* for values
//!
//! Compare values.
//!
//! * `assert_eq!(a, b)` ~ a == b
//!
//! * `assert_ne!(a, b)` ~ a != b
//!
//! * `assert_lt!(a, b)` ~ a < b
//!
//! * `assert_le!(a, b)` ~ a <= b
//!
//! * `assert_gt!(a, b)` ~ a > b
//!
//! * `assert_ge!(a, b)` ~ a >= b
//!
//!
//! ## assert_* for strings and matchers
//!
//! These macros help with strings and also other structures that
//! provide matchers such as starts_with, ends_width, and is_match.
//!
//! * `assert_starts_with(a, b) => a.starts_with(b)
//!
//! * `assert_not_starts_with(a, b) => !a.starts_with(b)
//! 
//! * `assert_ends_with(a, b) => a.ende_with(b)
//!
//! * `assert_not_ends_with(a, b) => !a.ende_with(b)
//! 
//! * `assert_contains(a, b) => a.contains(b)
//!
//! * `assert_not_contains(a, b) => !a.not_contains(b)
//! 
//! * `assert_match(a, b) => a.is_match(b)
//!
//! * `assert_match(a, b) => !a.is_match(b)
//!
//! 
//! ## assert_set_* for set collection comparisons
//!
//! These macros help with comparison of set parameters, such as two arrays or
//! two vectors. where the item order does not matter, and the item count does
//! not matter. The macros convert inputs into HashSet iterators.
//!
//! * `assert_set_eq!(a, b)` ~ set a == set b
//!
//! * `assert_set_ne!(a, b)` ~ set a != set b
//!
//! * `assert_set_subset!(a, b)` ~ set a ⊆ set b
//!
//! * `assert_set_superset!(a, b)` ~ set a ⊇ set b
//!
//! * `assert_set_joint!(a, b)` ~ set a is joint with set b
//!
//! * `assert_set_disjoint!(a, b)` ~ set a is disjoint with set b
//!
//!
//! ## assert_bag_* for bag collection comparisons
//!
//! These macros help with comparison of bag parameters, such as comparison of
//! two arrays or two vectors, where the item order does not matter, and the
//! item count does matter. The macros convert inputs into HashMap iterators.
//!
//! * `assert_bag_eq(a, b)` ~ bag a == bag b
//!
//! * `assert_bag_ne(a, b)` ~ bag a != bag b
//!
//! * `assert_bag_subbag(a, b)` ~ bag a ⊆ bag b
//!
//! * `assert_bag_superbag(a, b)` ~ bag a ⊇ bag b
//!
//!
//! ## assert_fn_* for function return comparisons
//!
//! * `assert_fn_eq!(f, a, b)` ~ f(a) == b
//!
//! * `assert_fn_eq_other!(f, a, b)` ~ f(a) == f(b)
//!
//! * `assert_fn_ne!(f, a, b)` ~ f(a) != b
//!
//! * `assert_fn_ne_other!(f, a, b)` ~ f(a) != f(b)
//!
//! * `assert_fn_lt!(f, a, b)` ~ f(a) < v
//!
//! * `assert_fn_lt_other!(f, a, b)` ~ f(a) < f(b)
//!
//! * `assert_fn_le!(f, a, b)` ~ f(a) <= b
//!
//! * `assert_fn_le_other!(f, a, b)` ~ f(a) <= f(b)
//!
//! * `assert_fn_gt!(f, a, b)` ~ f(a) > b
//!
//! * `assert_fn_gt_other!(f, a, b)` ~ f(a) > f(b)
//!
//! * `assert_fn_ge!(f, a, b)` ~ f(a) >= b
//!
//! * `assert_fn_ge_other!(f, a, b)` ~ f(a) >= f(b)
//!
//!
//! ## assert_fn_ok_* for function Ok() comparisons
//!
//! ### Compare with expr
//!
//! * `assert_fn_ok_eq!(f, a, b)` ~ f(a).unwrap() == b
//!
//! * `assert_fn_ok_ne!(f, a, b)` ~ f(a).unwrap() != b
//!
//! * `assert_fn_ok_lt!(f, a, b)` ~ f(a).unwrap() < b
//!
//! * `assert_fn_ok_le!(f, a, b)` ~ f(a).unwrap() <= b
//!
//! * `assert_fn_ok_gt!(f, a, b)` ~ f(a).unwrap() > b
//!
//! * `assert_fn_ok_gt!(f, a, b)` ~ f(a).unwrap() > b
//!
//! ### Compare with other
//!
//! * `assert_fn_ok_eq_other!(f, a, b)` ~ f(a).unwrap() == f(b).unwrap()
//!
//! * `assert_fn_ok_ne_other!(f, a, b)` ~ f(a).unwrap() != f(b).unwrap()
//!
//! * `assert_fn_ok_lt_other!(f, a, b)` ~ f(a).unwrap() < f(b).unwrap()
//!
//! * `assert_fn_ok_le_other!(f, a, b)` ~ f(a).unwrap() <= f(b).unwrap()
//!
//! * `assert_fn_ok_gt_other!(f, a, b)` ~ f(a).unwrap() > f(b).unwrap()
//!
//! * `assert_fn_ok_gt_other!(f, a, b)` ~ f(a).unwrap() > f(b).unwrap()
//!
//!
//! ## assert_fn_err_* for function Err() comparisons
//!
//! ### with expr
//!
//! * `assert_fn_err_eq!(f, a, b)` ~ f(a).unwrap_err() == b
//!
//! * `assert_fn_err_ne!(f, a, b)` ~ f(a).unwrap_err() != b
//!
//! * `assert_fn_err_lt!(f, a, b)` ~ f(a).unwrap_err() < b
//!
//! * `assert_fn_err_le!(f, a, b)` ~ f(a).unwrap_err() <= b
//!
//! * `assert_fn_err_gt!(f, a, b)` ~ f(a).unwrap_err() > b
//!
//! * `assert_fn_err_ge!(f, a, b)`~ f(a).unwrap_err() >= b
//!
//!
//! ### with other
//!
//! * `assert_fn_err_eq_other!(f, a, b)` ~ f(a).unwrap_err() == f(b).unwrap_err()
//!
//! * `assert_fn_err_ne_other!(f, a, b)` ~ f(a).unwrap_err() != f(b).unwrap_err()
//!
//! * `assert_fn_err_lt_other!(f, a, b)` ~ f(a).unwrap_err() < f(b).unwrap_err()
//!
//! * `assert_fn_err_le_other!(f, a, b)` ~ f(a).unwrap_err() <= f(b).unwrap_err()
//!
//! * `assert_fn_err_gt_other!(f, a, b)` ~ f(a).unwrap_err() > f(b).unwrap_err()
//!
//! * `assert_fn_err_ge_other!(f, a, b)`~ f(a).unwrap_err() >= f(b).unwrap_err()
//!
//!
//! ## assert_read_to_string_* for std::io::Read comparisons
//!
//! These macros help with readers, such as file handles, byte
//! arrays, input streams, and the trait std::io::Read.
//!
//! * `assert_read_to_string_eq!(a, b)` ~ a.read_to_string() == b
//!
//! * `assert_read_to_string_ne!(a, b)` ~ a.read_to_string() != b
//!
//! * `assert_read_to_string_lt!(a, b)` ~ a.read_to_string() < b
//!
//! * `assert_read_to_string_le!(a, b)` ~ a.read_to_string() <= b
//!
//! * `assert_read_to_string_gt!(a, b)` ~ a.read_to_string() > b
//!
//! * `assert_read_to_string_ge!(a, b)` ~ a.read_to_string() >= b
//!
//! * `assert_read_to_string_eq_other!(a, b)` ~ a.read_to_string() == b.read_to_string()
//!
//! * `assert_read_to_string_ne_other!(a, b)` ~ a.read_to_string() != b.read_to_string()
//!
//! * `assert_read_to_string_lt_other!(a, b)` ~ a.read_to_string() < b.read_to_string()
//!
//! * `assert_read_to_string_le_other!(a, b)` ~ a.read_to_string() <= b.read_to_string()
//!
//! * `assert_read_to_string_gt_other!(a, b)` ~ a.read_to_string() > b.read_to_string()
//!
//! * `assert_read_to_string_ge_other!(a, b)` ~ a.read_to_string() >= b.read_to_string()
//!
//!
//! ## assert_command_ for process command comparisons
//!
//! Using standard output a.k.a. stdout:
//!
//! * `assert_command_stdout_eq!(command, value)` ~ String::from_utf8(command.output().unwrap().stdout).unwrap() == value
//!
//! * `assert_command_stdout_eq_other!(command, command)` ~ String::from_utf8(command.output().unwrap().stdout).unwrap() == String::from_utf8(command.output().unwrap().stdout).unwrap()
//!
//! * `assert_command_stdout_contains!(command, containee)` ~ String::from_utf8(command.output().unwrap().stdout).unwrap().contains(containee)
//!
//! * `assert_command_stdout_matches!(command, matcher)` ~ regex.is_match(String::from_utf8(command.output().unwrap().stdout).unwrap())
//!
//! Using standard error a.k.a. stderr:
//!
//! * `assert_command_stderr_eq!(command, value)` ~ String::from_utf8(command.output().unwrap().stderr).unwrap() == value
//!
//! * `assert_command_stderr_eq_other!(command, command)` ~ String::from_utf8(command.output().unwrap().stderr).unwrap() == String::from_utf8(command.output().unwrap().stdout).unwrap()
//!
//! * `assert_command_stderr_contains!(command, containee)` ~ String::from_utf8(command.output().unwrap().stderr).unwrap().contains(containee)
//!
//! * `assert_command_stderr_matches!(command, matcher)` ~ regex.is_match(String::from_utf8(command.output().unwrap().stderr).unwrap())
//!
//!
//! ## assert_program_args_ for process command comparisons created via program name and args interator
//!
//! Using standard output a.k.a. stdout:
//!
//! * `assert_program_args_stdout_eq!(program, args, value)` ~ String::from_utf8(Command::new(program).args(args)..output().unwrap().stdout).unwrap() == value
//!
//! * `assert_program_args_stdout_eq_other!(program, args, program, args)` ~ String::from_utf8(Command::new(program).args(args)..output().unwrap().stdout).unwrap() == String::from_utf8(Command::new(program).args(args)..output().unwrap().stdout).unwrap()
//!
//! * `assert_program_args_stdout_contains!(program, args, containee)` ~ String::from_utf8(Command::new(program).args(args)..output().unwrap().stdout).unwrap().contains(containee)
//!
//! * `assert_program_args_stdout_matches!(program, args, matcher)` ~ regex.is_match(String::from_utf8(Command::new(program).args(args)..output().unwrap().stdout).unwrap())
//!
//! Using standard error a.k.a. stderr:
//!
//! * `assert_program_args_stderr_eq!(program, args, value)` ~ String::from_utf8(Command::new(program).args(args)..output().unwrap().stderr).unwrap() == value
//!
//! * `assert_program_args_stderr_eq_other!(program, args, program, args)` ~ String::from_utf8(Command::new(program).args(args)..output().unwrap().stderr).unwrap() == String::from_utf8(Command::new(program).args(args)..output().unwrap().stdout).unwrap()
//!
//! * `assert_program_args_stderr_contains!(program, args, containee)` ~ String::from_utf8(Command::new(program).args(args)..output().unwrap().stderr).unwrap().contains(containee)
//!
//! * `assert_program_args_stderr_matches!(program, args, matcher)` ~ regex.is_match(String::from_utf8(Command::new(program).args(args)..output().unwrap().stderr).unwrap())
//!
//!
//! ## Tracking
//!
//! * Package: assertables-rust-crate
//! * Version: 6.1.0
//! * Created: 2021-03-30T15:47:49Z
//! * Updated: 2023-01-11T17:57:42Z
//! * License: MIT or Apache-2.0 or GPL-2.0-or-later or contact us for custom license
//! * Contact: Joel Parker Henderson (joel@sixarm.com)

// Assert truth
pub mod assert; // (provided by Rust `std`)

// Assert value comparison
pub mod assert_eq; // (provided by Rust `std`)
pub mod assert_ne;
pub mod assert_lt;
pub mod assert_le;
pub mod assert_gt;
pub mod assert_ge;

// Assert value matching
pub mod assert_starts_with; pub mod assert_not_starts_with;
pub mod assert_ends_with; pub mod assert_not_ends_with;
pub mod assert_contains; pub mod assert_not_contains;
pub mod assert_match; pub mod assert_not_match;

// Assertable iterator-related set-based comparison
pub mod assert_set_eq;
pub mod assert_set_ne;
pub mod assert_set_subset;
pub mod assert_set_superset;
pub mod assert_set_joint;
pub mod assert_set_disjoint;

// Assertable iterator-related bag-based comparison
pub mod assert_bag_eq;
pub mod assert_bag_ne;
pub mod assert_bag_subbag;
pub mod assert_bag_superbag;

// Assert function return
pub mod assert_fn_eq; pub mod assert_fn_eq_expr;
pub mod assert_fn_ne; pub mod assert_fn_ne_expr;
pub mod assert_fn_lt; pub mod assert_fn_lt_expr;
pub mod assert_fn_le; pub mod assert_fn_le_expr;
pub mod assert_fn_gt; pub mod assert_fn_gt_expr;
pub mod assert_fn_ge; pub mod assert_fn_ge_expr;

// Assert function Ok()
pub mod assert_fn_ok_eq; pub mod assert_fn_ok_eq_expr;
pub mod assert_fn_ok_ne; pub mod assert_fn_ok_ne_expr;
pub mod assert_fn_ok_lt; pub mod assert_fn_ok_lt_expr;
pub mod assert_fn_ok_le; pub mod assert_fn_ok_le_expr;
pub mod assert_fn_ok_gt; pub mod assert_fn_ok_ge_expr;
pub mod assert_fn_ok_ge; pub mod assert_fn_ok_gt_expr;

// Assert function Err() 
pub mod assert_fn_err_eq; pub mod assert_fn_err_eq_expr;
pub mod assert_fn_err_ne; pub mod assert_fn_err_ne_expr;
pub mod assert_fn_err_lt; pub mod assert_fn_err_lt_expr;
pub mod assert_fn_err_le; pub mod assert_fn_err_le_expr;
pub mod assert_fn_err_gt; pub mod assert_fn_err_gt_expr;
pub mod assert_fn_err_ge; pub mod assert_fn_err_ge_expr;

// Assert std::io::read comparisons
pub mod assert_read_to_string_eq; pub mod assert_read_to_string_eq_expr;
pub mod assert_read_to_string_ne; pub mod assert_read_to_string_ne_expr;
pub mod assert_read_to_string_lt; pub mod assert_read_to_string_lt_expr;
pub mod assert_read_to_string_le; pub mod assert_read_to_string_le_expr;
pub mod assert_read_to_string_gt; pub mod assert_read_to_string_gt_expr;
pub mod assert_read_to_string_ge; pub mod assert_read_to_string_ge_expr;


// Assert std::io::read specializations
pub mod assert_read_to_string_contains;
pub mod assert_read_to_string_matches;

// Assert command stdout
pub mod assert_command_stdout_eq; pub mod assert_command_stdout_eq_expr;

// Assert command stdout specializations
pub mod assert_command_stdout_contains;
pub mod assert_command_stdout_matches;

// Assert command stderr
pub mod assert_command_stderr_eq; pub mod assert_command_stderr_eq_expr;

// Assert command stderr specializations
pub mod assert_command_stderr_contains; 
pub mod assert_command_stderr_matches; 

// Assert program args stdout
pub mod assert_program_args_stdout_eq; pub mod assert_program_args_stdout_eq_expr;

// Assert program args stdout specializations
pub mod assert_program_args_stdout_contains;
pub mod assert_program_args_stdout_matches;

// Assert program args stderr
pub mod assert_program_args_stderr_eq; pub mod assert_program_args_stderr_eq_expr;

// Assert program args stderr specializations
pub mod assert_program_args_stderr_contains;
pub mod assert_program_args_stderr_matches;
