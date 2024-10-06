//! Assert macros for comparing commands and their stdout & stderr.
//!
//! These macros help with calling external commands, then capturing the
//! standard output stream and standard error stream.
//!
//! These macros have corresponding the macros in the module [`assert_program_args`](module@crate::assert_program_args).
//!
//! ## Command standard output
//! 
//! Compare command standard output to another command standard output:
//!
//! * [`assert_command_stdout_eq!(command1, command2)`](macro@crate::assert_command_stdout_eq) ≈ command1 stdout = command2 stdout
//! * [`assert_command_stdout_ne!(command1, command2)`](macro@crate::assert_command_stdout_ne) ≈ command1 stdout ≠ command2 stdout
//! * [`assert_command_stdout_lt!(command1, command2)`](macro@crate::assert_command_stdout_lt) ≈ command1 stdout < command2 stdout
//! * [`assert_command_stdout_le!(command1, command2)`](macro@crate::assert_command_stdout_le) ≈ command1 stdout ≤ command2 stdout
//! * [`assert_command_stdout_gt!(command1, command2)`](macro@crate::assert_command_stdout_gt) ≈ command1 stdout > command2 stdout
//! * [`assert_command_stdout_ge!(command1, command2)`](macro@crate::assert_command_stdout_ge) ≈ command1 stdout ≥ command2 stdout
//! 
//! Compare command standard output to an expression:
//! 
//! * [`assert_command_stdout_eq_expr!(command, expr)`](macro@crate::assert_command_stdout_eq_expr) ≈ command stdout = expr
//! * [`assert_command_stdout_ne_expr!(command, expr)`](macro@crate::assert_command_stdout_ne_expr) ≈ command stdout ≠ expr
//! * [`assert_command_stdout_lt_expr!(command, expr)`](macro@crate::assert_command_stdout_lt_expr) ≈ command stdout < expr
//! * [`assert_command_stdout_le_expr!(command, expr)`](macro@crate::assert_command_stdout_le_expr) ≈ command stdout ≤ expr
//! * [`assert_command_stdout_gt_expr!(command, expr)`](macro@crate::assert_command_stdout_gt_expr) ≈ command stdout > expr
//! * [`assert_command_stdout_ge_expr!(command, expr)`](macro@crate::assert_command_stdout_ge_expr) ≈ command stdout ≥ expr
//! 
//! Assert command standard output as a string:
//! 
//! * [`assert_command_stdout_string_contains!(command, containee)`](macro@crate::assert_command_stdout_string_contains) ≈ command stdout string contains containee
//! * [`assert_command_stdout_string_is_match!(command, matcher)`](macro@crate::assert_command_stdout_string_is_match) ≈ command stdout string is a matcher match
//!
//! ## Command standard error
//! 
//! Compare command standard error to another command standard error:
//!
//! * [`assert_command_stderr_eq!(command1, command2)`](macro@crate::assert_command_stderr_eq) ≈ command1 stderr = command2 stderr
//! * [`assert_command_stderr_ne!(command1, command2)`](macro@crate::assert_command_stderr_ne) ≈ command1 stderr ≠ command2 stderr
//! * [`assert_command_stderr_lt!(command1, command2)`](macro@crate::assert_command_stderr_lt) ≈ command1 stderr < command2 stderr
//! * [`assert_command_stderr_le!(command1, command2)`](macro@crate::assert_command_stderr_le) ≈ command1 stderr ≤ command2 stderr
//! * [`assert_command_stderr_gt!(command1, command2)`](macro@crate::assert_command_stderr_gt) ≈ command1 stderr > command2 stderr
//! * [`assert_command_stderr_ge!(command1, command2)`](macro@crate::assert_command_stderr_ge) ≈ command1 stderr ≥ command2 stderr
//! 
//! Compare command standard error to an expression:
//! 
//! * [`assert_command_stderr_eq_expr!(command, expr)`](macro@crate::assert_command_stderr_eq_expr) ≈ command stderr = expr
//! * [`assert_command_stderr_ne_expr!(command, expr)`](macro@crate::assert_command_stderr_ne_expr) ≈ command stderr ≠ expr
//! * [`assert_command_stderr_lt_expr!(command, expr)`](macro@crate::assert_command_stderr_lt_expr) ≈ command stderr < expr
//! * [`assert_command_stderr_le_expr!(command, expr)`](macro@crate::assert_command_stderr_le_expr) ≈ command stderr ≤ expr
//! * [`assert_command_stderr_gt_expr!(command, expr)`](macro@crate::assert_command_stderr_gt_expr) ≈ command stderr > expr
//! * [`assert_command_stderr_ge_expr!(command, expr)`](macro@crate::assert_command_stderr_ge_expr) ≈ command stderr ≥ expr
//! 
//! Assert standard error as a string:
//! 
//! * [`assert_command_stderr_string_contains!(command, containee)`](macro@crate::assert_command_stderr_string_contains) ≈ command stderr string contains containee
//! * [`assert_command_stderr_string_is_match!(command, matcher)`](macro@crate::assert_command_stderr_string_is_match) ≈ command stderr string is a matcher match
//!
//! # Example
//!
//! ```rust
//! use assertables::*;
//! use std::process::Command;
//!
//! # fn main() {
//! let mut a = Command::new("bin/printf-stdout");
//! a.args(["%s", "alfa"]);
//! let mut b = Command::new("bin/printf-stdout");
//! b.args(["%s%s%s%s", "a", "l", "f", "a"]);
//! assert_command_stdout_eq!(a, b);
//! # }
//! ```

// stdout
pub mod assert_command_stdout_eq;
pub mod assert_command_stdout_ne;
pub mod assert_command_stdout_lt;
pub mod assert_command_stdout_le;
pub mod assert_command_stdout_gt;
pub mod assert_command_stdout_ge;

// stdout expr
pub mod assert_command_stdout_eq_expr;
pub mod assert_command_stdout_ne_expr;
pub mod assert_command_stdout_lt_expr;
pub mod assert_command_stdout_le_expr;
pub mod assert_command_stdout_gt_expr;
pub mod assert_command_stdout_ge_expr;

// stdout string
pub mod assert_command_stdout_contains;
pub mod assert_command_stdout_string_contains;
pub mod assert_command_stdout_is_match;
pub mod assert_command_stdout_string_is_match;

// stderr
pub mod assert_command_stderr_eq;
pub mod assert_command_stderr_ne;
pub mod assert_command_stderr_lt;
pub mod assert_command_stderr_le;
pub mod assert_command_stderr_gt;
pub mod assert_command_stderr_ge;

// stderr expr
pub mod assert_command_stderr_eq_expr;
pub mod assert_command_stderr_ne_expr;
pub mod assert_command_stderr_lt_expr;
pub mod assert_command_stderr_le_expr;
pub mod assert_command_stderr_gt_expr;
pub mod assert_command_stderr_ge_expr;

// stderr string
pub mod assert_command_stderr_contains;
pub mod assert_command_stderr_string_contains;
pub mod assert_command_stderr_is_match;
pub mod assert_command_stderr_string_is_match;
