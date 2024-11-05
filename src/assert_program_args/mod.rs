//! Assert for comparing programs with arguments.
//!
//! These macros help with calling external programs with arguments, then
//! capturing the standard output stream and standard error stream.
//! See tutorial below.
//!
//! These macros have corresponding macros in the module [`assert_command`](module@crate::assert_command).
//!
//! ## Program args stdout
//!
//! Compare program and arguments standard output to another program and arguments standard output:
//!
//! * [`assert_program_args_stdout_eq!(a_program, a_args, b_program, b_args)`](macro@crate::assert_program_args_stdout_eq) ≈ command using a_program and a_args to stdout = b_command with b_program and b_args to stdout
//! * [`assert_program_args_stdout_ne!(a_program, a_args, b_program, b_args)`](macro@crate::assert_program_args_stdout_ne) ≈ command using a_program and a_args to stdout ≠ b_command with b_program and b_args to stdout
//! * [`assert_program_args_stdout_lt!(a_program, a_args, b_program, b_args)`](macro@crate::assert_program_args_stdout_lt) ≈ command using a_program and a_args to stdout < b_command with b_program and b_args to stdout
//! * [`assert_program_args_stdout_le!(a_program, a_args, b_program, b_args)`](macro@crate::assert_program_args_stdout_le) ≈ command using a_program and a_args to stdout ≤ b_command with b_program and b_args to stdout
//! * [`assert_program_args_stdout_gt!(a_program, a_args, b_program, b_args)`](macro@crate::assert_program_args_stdout_gt) ≈ command using a_program and a_args to stdout > b_command with b_program and b_args to stdout
//! * [`assert_program_args_stdout_ge!(a_program, a_args, b_program, b_args)`](macro@crate::assert_program_args_stdout_ge) ≈ command using a_program and a_args to stdout ≥ b_command with b_program and b_args to stdout
//!
//! Compare program and arguments standard output to an expression:
//!
//! * [`assert_program_args_stdout_eq_x!(program, args, expr)`](macro@crate::assert_program_args_stdout_eq_x) ≈ command using program and args to stdout = expr
//! * [`assert_program_args_stdout_ne_x!(program, args, expr)`](macro@crate::assert_program_args_stdout_ne_x) ≈ command using program and args to stdout ≠ expr
//! * [`assert_program_args_stdout_lt_x!(program, args, expr)`](macro@crate::assert_program_args_stdout_lt_x) ≈ command using program and args to stdout < expr
//! * [`assert_program_args_stdout_le_x!(program, args, expr)`](macro@crate::assert_program_args_stdout_le_x) ≈ command using program and args to stdout ≤ expr
//! * [`assert_program_args_stdout_gt_x!(program, args, expr)`](macro@crate::assert_program_args_stdout_gt_x) ≈ command using program and args to stdout > expr
//! * [`assert_program_args_stdout_ge_x!(program, args, expr)`](macro@crate::assert_program_args_stdout_ge_x) ≈ command using program and args to stdout ≥ expr
//!
//! Assert program and arguments standard output as a string:
//!
//! * [`assert_program_args_stdout_string_contains!(program, args, containee)`](macro@crate::assert_program_args_stdout_string_contains) ≈ command using program and args to stdout string contains containee
//! * [`assert_program_args_stdout_string_is_match!(program, args, matcher)`](macro@crate::assert_program_args_stdout_string_is_match) ≈ matcher is match with command using program and args
//!
//! ## Program args stderr
//!
//! Compare program and arguments standard error to another program and arguments standard error:
//!
//! * [`assert_program_args_stderr_eq!(a_program, a_args, b_program, b_args)`](macro@crate::assert_program_args_stderr_eq) ≈ command using a_program and a_args to stderr = b_command with b_program and b_args to stderr
//! * [`assert_program_args_stderr_ne!(a_program, a_args, b_program, b_args)`](macro@crate::assert_program_args_stderr_ne) ≈ command using a_program and a_args to stderr ≠ b_command with b_program and b_args to stderr
//! * [`assert_program_args_stderr_lt!(a_program, a_args, b_program, b_args)`](macro@crate::assert_program_args_stderr_lt) ≈ command using a_program and a_args to stderr < b_command with b_program and b_args to stderr
//! * [`assert_program_args_stderr_le!(a_program, a_args, b_program, b_args)`](macro@crate::assert_program_args_stderr_le) ≈ command using a_program and a_args to stderr ≤ b_command with b_program and b_args to stderr
//! * [`assert_program_args_stderr_gt!(a_program, a_args, b_program, b_args)`](macro@crate::assert_program_args_stderr_gt) ≈ command using a_program and a_args to stderr > b_command with b_program and b_args to stderr
//! * [`assert_program_args_stderr_ge!(a_program, a_args, b_program, b_args)`](macro@crate::assert_program_args_stderr_ge) ≈ command using a_program and a_args to stderr ≥ b_command with b_program and b_args to stderr
//!
//! Compare program and arguments standard error to an expression:
//!
//! * [`assert_program_args_stderr_eq_x!(program, args, expr)`](macro@crate::assert_program_args_stderr_eq_x) ≈ command using program and args to stderr = expr
//! * [`assert_program_args_stderr_ne_x!(program, args, expr)`](macro@crate::assert_program_args_stderr_ne_x) ≈ command using program and args to stderr ≠ expr
//! * [`assert_program_args_stderr_lt_x!(program, args, expr)`](macro@crate::assert_program_args_stderr_lt_x) ≈ command using program and args to stderr < expr
//! * [`assert_program_args_stderr_le_x!(program, args, expr)`](macro@crate::assert_program_args_stderr_le_x) ≈ command using program and args to stderr ≤ expr
//! * [`assert_program_args_stderr_gt_x!(program, args, expr)`](macro@crate::assert_program_args_stderr_gt_x) ≈ command using program and args to stderr > expr
//! * [`assert_program_args_stderr_ge_x!(program, args, expr)`](macro@crate::assert_program_args_stderr_ge_x) ≈ command using program and args to stderr ≥ expr
//!
//! Assert program and arguments standard error as a string:
//!
//! * [`assert_program_args_stderr_string_contains!(program, args, containee)`](macro@crate::assert_program_args_stderr_string_contains) ≈ command using program and args to stderr string contains containee
//! * [`assert_program_args_stderr_string_is_match!(program, args, matcher)`](macro@crate::assert_program_args_stderr_string_is_match) ≈ matcher is match with command using program and args
//!
//! # Example
//!
//! ```rust
//! use assertables::*;
//!
//! let a_program = "bin/printf-stdout";
//! let a_args = ["%s", "alfa"];
//! let b_program = "bin/printf-stdout";
//! let b_args = ["%s%s%s%s", "a", "l", "f", "a"];
//! assert_program_args_stdout_eq!(&a_program, &a_args, &b_program, &b_args);
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
//! let program = "printf";
//! let args = ["%s", "hello"];
//! let mut command = Command::new(program);
//! command.args(args);
//! let stdout = command.output().unwrap().stdout;
//! ```
//!
//! Rust can compare a command's standard output to another command's standard output, by using:
//!
//! ```rust
//! # use std::process::Command;
//! let a_program = "printf";
//! let a_args = ["%s", "hello"];
//! let b_program = "printf";
//! let b_args = ["%s", "world"];
//! let mut a_command = Command::new(a_program); a_command.args(a_args);
//! let mut b_command = Command::new(b_program); b_command.args(b_args);
//! let a_stdout = a_command.output().unwrap().stdout;
//! let b_stdout = b_command.output().unwrap().stdout;
//! assert_ne!(a_stdout, b_stdout);
//! ```
//!
//! The `assertables` crate provides macros that do the same kind of processing, by automatically converting programs and args into commands, then to standard outputs:
//!
//! ```rust
//! # use std::process::Command;
//! # use assertables::*;
//! let a_program = "printf";
//! let a_args = ["%s", "hello"];
//! let b_program = "printf";
//! let b_args = ["%s", "world"];
//! assert_program_args_stdout_ne!(a_program, a_args, b_program, b_args);
//! ```

/// Assert program args implementation preparation.
#[macro_export]
macro_rules! assert_program_args_impl_prep {
    ($program:expr, $args:expr $(,)?) => {{
        let mut command = ::std::process::Command::new($program);
        command.args($args.into_iter());
        command.output()
    }};
}

// stdout
pub mod assert_program_args_stdout_eq;
pub mod assert_program_args_stdout_ge;
pub mod assert_program_args_stdout_gt;
pub mod assert_program_args_stdout_le;
pub mod assert_program_args_stdout_lt;
pub mod assert_program_args_stdout_ne;

// stdout expr
pub mod assert_program_args_stdout_eq_x;
pub mod assert_program_args_stdout_ge_x;
pub mod assert_program_args_stdout_gt_x;
pub mod assert_program_args_stdout_le_x;
pub mod assert_program_args_stdout_lt_x;
pub mod assert_program_args_stdout_ne_x;

// stdout string
pub mod assert_program_args_stdout_contains;
pub mod assert_program_args_stdout_is_match;
pub mod assert_program_args_stdout_string_contains;
pub mod assert_program_args_stdout_string_is_match;

// stderr
pub mod assert_program_args_stderr_eq;
pub mod assert_program_args_stderr_ge;
pub mod assert_program_args_stderr_gt;
pub mod assert_program_args_stderr_le;
pub mod assert_program_args_stderr_lt;
pub mod assert_program_args_stderr_ne;

pub mod assert_program_args_stderr_eq_x;
pub mod assert_program_args_stderr_ge_x;
pub mod assert_program_args_stderr_gt_x;
pub mod assert_program_args_stderr_le_x;
pub mod assert_program_args_stderr_lt_x;
pub mod assert_program_args_stderr_ne_x;

// stderr string
pub mod assert_program_args_stderr_contains;
pub mod assert_program_args_stderr_is_match;
pub mod assert_program_args_stderr_string_contains;
pub mod assert_program_args_stderr_string_is_match;
