//! Assert for comparing commands and their stdout & stderr.
//!
//! These macros help with calling external commands, then capturing the
//! standard output stream and standard error stream.
//! See tutorial below.
//!
//! These macros have corresponding the macros in the module [`assert_program_args`](module@crate::assert_program_args).
//!
//! ## Macros for command standard output
//!
//! Compare command standard output to another command standard output:
//!
//! * [`assert_command_stdout_eq!(a_command, b_command)`](macro@crate::assert_command_stdout_eq) ≈ a_command stdout = b_command stdout
//! * [`assert_command_stdout_ne!(a_command, b_command)`](macro@crate::assert_command_stdout_ne) ≈ a_command stdout ≠ b_command stdout
//! * [`assert_command_stdout_lt!(a_command, b_command)`](macro@crate::assert_command_stdout_lt) ≈ a_command stdout < b_command stdout
//! * [`assert_command_stdout_le!(a_command, b_command)`](macro@crate::assert_command_stdout_le) ≈ a_command stdout ≤ b_command stdout
//! * [`assert_command_stdout_gt!(a_command, b_command)`](macro@crate::assert_command_stdout_gt) ≈ a_command stdout > b_command stdout
//! * [`assert_command_stdout_ge!(a_command, b_command)`](macro@crate::assert_command_stdout_ge) ≈ a_command stdout ≥ b_command stdout
//!
//! Compare command standard output to an expression:
//!
//! * [`assert_command_stdout_eq_x!(command, expr)`](macro@crate::assert_command_stdout_eq_x) ≈ command stdout = expr
//! * [`assert_command_stdout_ne_x!(command, expr)`](macro@crate::assert_command_stdout_ne_x) ≈ command stdout ≠ expr
//! * [`assert_command_stdout_lt_x!(command, expr)`](macro@crate::assert_command_stdout_lt_x) ≈ command stdout < expr
//! * [`assert_command_stdout_le_x!(command, expr)`](macro@crate::assert_command_stdout_le_x) ≈ command stdout ≤ expr
//! * [`assert_command_stdout_gt_x!(command, expr)`](macro@crate::assert_command_stdout_gt_x) ≈ command stdout > expr
//! * [`assert_command_stdout_ge_x!(command, expr)`](macro@crate::assert_command_stdout_ge_x) ≈ command stdout ≥ expr
//!
//! Assert command standard output as a string:
//!
//! * [`assert_command_stdout_string_contains!(command, containee)`](macro@crate::assert_command_stdout_string_contains) ≈ command stdout string contains containee
//! * [`assert_command_stdout_string_is_match!(command, matcher)`](macro@crate::assert_command_stdout_string_is_match) ≈ command stdout string is a matcher match
//!
//! ## Macros for command standard error
//!
//! Compare command standard error to another command standard error:
//!
//! * [`assert_command_stderr_eq!(a_command, b_command)`](macro@crate::assert_command_stderr_eq) ≈ a_command stderr = b_command stderr
//! * [`assert_command_stderr_ne!(a_command, b_command)`](macro@crate::assert_command_stderr_ne) ≈ a_command stderr ≠ b_command stderr
//! * [`assert_command_stderr_lt!(a_command, b_command)`](macro@crate::assert_command_stderr_lt) ≈ a_command stderr < b_command stderr
//! * [`assert_command_stderr_le!(a_command, b_command)`](macro@crate::assert_command_stderr_le) ≈ a_command stderr ≤ b_command stderr
//! * [`assert_command_stderr_gt!(a_command, b_command)`](macro@crate::assert_command_stderr_gt) ≈ a_command stderr > b_command stderr
//! * [`assert_command_stderr_ge!(a_command, b_command)`](macro@crate::assert_command_stderr_ge) ≈ a_command stderr ≥ b_command stderr
//!
//! Compare command standard error to an expression:
//!
//! * [`assert_command_stderr_eq_x!(command, expr)`](macro@crate::assert_command_stderr_eq_x) ≈ command stderr = expr
//! * [`assert_command_stderr_ne_x!(command, expr)`](macro@crate::assert_command_stderr_ne_x) ≈ command stderr ≠ expr
//! * [`assert_command_stderr_lt_x!(command, expr)`](macro@crate::assert_command_stderr_lt_x) ≈ command stderr < expr
//! * [`assert_command_stderr_le_x!(command, expr)`](macro@crate::assert_command_stderr_le_x) ≈ command stderr ≤ expr
//! * [`assert_command_stderr_gt_x!(command, expr)`](macro@crate::assert_command_stderr_gt_x) ≈ command stderr > expr
//! * [`assert_command_stderr_ge_x!(command, expr)`](macro@crate::assert_command_stderr_ge_x) ≈ command stderr ≥ expr
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
//! let mut a = Command::new("bin/printf-stdout");
//! a.args(["%s", "alfa"]);
//! let mut b = Command::new("bin/printf-stdout");
//! b.args(["%s%s%s%s", "a", "l", "f", "a"]);
//! assert_command_stdout_eq!(a, b);
//! ```
//!
//! ## Tutorial
//!
//! Rust programs can call system commands.
//!
//! For example this system command will print the word hello:
//!
//! ```sh
//! printf %s hello
//! ```
//!
//! Rust can create a system command then capture its standard output, by using:
//!
//! ```rust
//! # use std::process::Command;
//! let mut command = Command::new("printf"); command.args(["%s", "hello"]);
//! let stdout = command.output().unwrap().stdout;
//! ```
//!
//! Rust can compare a command's standard output to another command's standard output, by using:
//!
//! ```rust
//! # use std::process::Command;
//! let mut a_command = Command::new("printf"); a_command.args(["%s", "hello"]);
//! let mut b_command = Command::new("printf"); a_command.args(["%s", "world"]);
//! let a_stdout = a_command.output().unwrap().stdout;
//! let b_stdout = b_command.output().unwrap().stdout;
//! assert_ne!(a_stdout, b_stdout);
//! ```
//!
//! The `assertables` crate provides macros that do the same kind of processing, by automatically converting each command into standard output:
//!
//! ```rust
//! # use std::process::Command;
//! # use assertables::*;
//! let mut a_command = Command::new("printf"); a_command.args(["%s", "hello"]);
//! let mut b_command = Command::new("printf"); a_command.args(["%s", "world"]);
//! assert_command_stdout_ne!(a_command, b_command);
//! ```

// Compare another
pub mod assert_command_stdout_eq;
pub mod assert_command_stdout_ge;
pub mod assert_command_stdout_gt;
pub mod assert_command_stdout_le;
pub mod assert_command_stdout_lt;
pub mod assert_command_stdout_ne;

// Compare expression
pub mod assert_command_stdout_eq_x;
pub mod assert_command_stdout_ge_x;
pub mod assert_command_stdout_gt_x;
pub mod assert_command_stdout_le_x;
pub mod assert_command_stdout_lt_x;
pub mod assert_command_stdout_ne_x;

// stdout string
pub mod assert_command_stdout_contains;
pub mod assert_command_stdout_is_match;
pub mod assert_command_stdout_string_contains;
pub mod assert_command_stdout_string_is_match;

// stderr
pub mod assert_command_stderr_eq;
pub mod assert_command_stderr_ge;
pub mod assert_command_stderr_gt;
pub mod assert_command_stderr_le;
pub mod assert_command_stderr_lt;
pub mod assert_command_stderr_ne;

// stderr vs expr
pub mod assert_command_stderr_eq_x;
pub mod assert_command_stderr_ge_x;
pub mod assert_command_stderr_gt_x;
pub mod assert_command_stderr_le_x;
pub mod assert_command_stderr_lt_x;
pub mod assert_command_stderr_ne_x;

// stderr string
pub mod assert_command_stderr_contains;
pub mod assert_command_stderr_is_match;
pub mod assert_command_stderr_string_contains;
pub mod assert_command_stderr_string_is_match;
