//! Assert macros for comparing commands and their stdout & stderr.
//!
//! These macros help with calling external commands, then capturing the
//! standard output stream and standard error stream.
//!
//! These macros have corresponding the macros in the module [`assert_program_args`](module@crate::assert_program_args).
//!
//! Compare command standard output string:
//!
//! * [`assert_command_stdout_eq!(command1, command2)`](macro@crate::assert_command_stdout_eq) ≈ command1 stdout = command2 stdout
//!
//! * [`assert_command_stdout_eq_expr!(command, expr)`](macro@crate::assert_command_stdout_eq_expr) ≈ command stdout = expr
//!
//! * [`assert_command_stdout_contains!(command, containee)`](macro@crate::assert_command_stdout_contains) ≈ command stdout contains containee
//!
//! * [`assert_command_stdout_is_match!(command, matcher)`](macro@crate::assert_command_stdout_is_match) ≈ command stdout is a matcher match
//!
//! Compare command standard error string:
//!
//! * [`assert_command_stderr_eq!(command1, command2)`](macro@crate::assert_command_stderr_eq) ≈ command1 stderr = command2 stderr
//!
//! * [`assert_command_stderr_eq_expr!(command, expr)`](macro@crate::assert_command_stderr_eq_expr) ≈ command stderr = expr
//!
//! * [`assert_command_stderr_contains!(command, containee)`](macro@crate::assert_command_stderr_contains) ≈ command stderr contains containee
//!
//! * [`assert_command_stderr_is_match!(command, matcher)`](macro@crate::assert_command_stderr_is_match) ≈ command stderr is a matcher match
//!
//!
//! # Example
//!
//! ```rust
//! # #[macro_use] extern crate assertables;
//! use std::process::Command;
//!
//! # fn main() {
//! let mut a = Command::new("bin/printf-stdout");
//! a.args(["%s", "hello"]);
//! let mut b = Command::new("bin/printf-stdout");
//! b.args(["%s%s%s%s%s", "h", "e", "l", "l", "o"]);
//! assert_command_stdout_eq!(a, b);
//! # }
//! ```

// stdout
pub mod assert_command_stdout_eq;
pub mod assert_command_stdout_eq_expr;

// stdout searches
pub mod assert_command_stdout_contains;
pub mod assert_command_stdout_is_match;

// stderr
pub mod assert_command_stderr_eq;
pub mod assert_command_stderr_eq_expr;

// stderr searchers
pub mod assert_command_stderr_contains;
pub mod assert_command_stderr_is_match;
