//! Assert for comparing programs with arguments.
//!
//! These macros help with calling external programs with arguments, then
//! capturing the standard output stream and standard error stream.
//!
//! These macros have corresponding macros in the module [`assert_command`](module@crate::assert_command).
//!
//! ## Program args stdout
//!
//! Compare program and arguments standard output to another program and arguments standard output:
//!
//! * [`assert_program_args_stdout_eq2!(program1, args1, program2, args2)`](macro@crate::assert_program_args_stdout_eq2) ≈ command using program1 and args1 to stdout = command2 with program2 and args2 to stdout
//! * [`assert_program_args_stdout_ne2!(program1, args1, program2, args2)`](macro@crate::assert_program_args_stdout_ne2) ≈ command using program1 and args1 to stdout ≠ command2 with program2 and args2 to stdout
//! * [`assert_program_args_stdout_lt2!(program1, args1, program2, args2)`](macro@crate::assert_program_args_stdout_lt2) ≈ command using program1 and args1 to stdout < command2 with program2 and args2 to stdout
//! * [`assert_program_args_stdout_le2!(program1, args1, program2, args2)`](macro@crate::assert_program_args_stdout_le2) ≈ command using program1 and args1 to stdout ≤ command2 with program2 and args2 to stdout
//! * [`assert_program_args_stdout_gt2!(program1, args1, program2, args2)`](macro@crate::assert_program_args_stdout_gt2) ≈ command using program1 and args1 to stdout > command2 with program2 and args2 to stdout
//! * [`assert_program_args_stdout_ge2!(program1, args1, program2, args2)`](macro@crate::assert_program_args_stdout_gt2) ≈ command using program1 and args1 to stdout ≥ command2 with program2 and args2 to stdout
//!
//! Compare program and arguments standard output to an expression:
//!
//! * [`assert_program_args_stdout_eq!(program, args, expr)`](macro@crate::assert_program_args_stdout_eq) ≈ command using program and args to stdout = expr
//! * [`assert_program_args_stdout_ne!(program, args, expr)`](macro@crate::assert_program_args_stdout_ne) ≈ command using program and args to stdout ≠ expr
//! * [`assert_program_args_stdout_lt!(program, args, expr)`](macro@crate::assert_program_args_stdout_lt) ≈ command using program and args to stdout < expr
//! * [`assert_program_args_stdout_le!(program, args, expr)`](macro@crate::assert_program_args_stdout_le) ≈ command using program and args to stdout ≤ expr
//! * [`assert_program_args_stdout_gt!(program, args, expr)`](macro@crate::assert_program_args_stdout_gt) ≈ command using program and args to stdout > expr
//! * [`assert_program_args_stdout_ge!(program, args, expr)`](macro@crate::assert_program_args_stdout_gt) ≈ command using program and args to stdout ≥ expr
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
//! * [`assert_program_args_stderr_eq2!(program1, args1, program2, args2)`](macro@crate::assert_program_args_stderr_eq2) ≈ command using program1 and args1 to stderr = command2 with program2 and args2 to stderr
//! * [`assert_program_args_stderr_ne2!(program1, args1, program2, args2)`](macro@crate::assert_program_args_stderr_ne2) ≈ command using program1 and args1 to stderr ≠ command2 with program2 and args2 to stderr
//! * [`assert_program_args_stderr_lt2!(program1, args1, program2, args2)`](macro@crate::assert_program_args_stderr_lt2) ≈ command using program1 and args1 to stderr < command2 with program2 and args2 to stderr
//! * [`assert_program_args_stderr_le2!(program1, args1, program2, args2)`](macro@crate::assert_program_args_stderr_le2) ≈ command using program1 and args1 to stderr ≤ command2 with program2 and args2 to stderr
//! * [`assert_program_args_stderr_gt2!(program1, args1, program2, args2)`](macro@crate::assert_program_args_stderr_gt2) ≈ command using program1 and args1 to stderr > command2 with program2 and args2 to stderr
//! * [`assert_program_args_stderr_ge2!(program1, args1, program2, args2)`](macro@crate::assert_program_args_stderr_gt2) ≈ command using program1 and args1 to stderr ≥ command2 with program2 and args2 to stderr
//!
//! Compare program and arguments standard error to an expression:
//!
//! * [`assert_program_args_stderr_eq!(program, args, expr)`](macro@crate::assert_program_args_stderr_eq) ≈ command using program and args to stderr = expr
//! * [`assert_program_args_stderr_ne!(program, args, expr)`](macro@crate::assert_program_args_stderr_ne) ≈ command using program and args to stderr ≠ expr
//! * [`assert_program_args_stderr_lt!(program, args, expr)`](macro@crate::assert_program_args_stderr_lt) ≈ command using program and args to stderr < expr
//! * [`assert_program_args_stderr_le!(program, args, expr)`](macro@crate::assert_program_args_stderr_le) ≈ command using program and args to stderr ≤ expr
//! * [`assert_program_args_stderr_gt!(program, args, expr)`](macro@crate::assert_program_args_stderr_gt) ≈ command using program and args to stderr > expr
//! * [`assert_program_args_stderr_ge!(program, args, expr)`](macro@crate::assert_program_args_stderr_gt) ≈ command using program and args to stderr ≥ expr
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
//! # fn main() {
//! let a_program = "bin/printf-stdout";
//! let a_args = ["%s", "alfa"];
//! let b_program = "bin/printf-stdout";
//! let b_args = ["%s%s%s%s", "a", "l", "f", "a"];
//! assert_program_args_stdout_eq2!(&a_program, &a_args, &b_program, &b_args);
//! # }
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
pub mod assert_program_args_stdout_eq2;
pub mod assert_program_args_stdout_ge2;
pub mod assert_program_args_stdout_gt2;
pub mod assert_program_args_stdout_le2;
pub mod assert_program_args_stdout_lt2;
pub mod assert_program_args_stdout_ne2;

// stdout expr
pub mod assert_program_args_stdout_eq;
pub mod assert_program_args_stdout_ge;
pub mod assert_program_args_stdout_gt;
pub mod assert_program_args_stdout_le;
pub mod assert_program_args_stdout_lt;
pub mod assert_program_args_stdout_ne;

// stdout string
pub mod assert_program_args_stdout_contains;
pub mod assert_program_args_stdout_is_match;
pub mod assert_program_args_stdout_string_contains;
pub mod assert_program_args_stdout_string_is_match;

// stderr
pub mod assert_program_args_stderr_eq2;
pub mod assert_program_args_stderr_ge2;
pub mod assert_program_args_stderr_gt2;
pub mod assert_program_args_stderr_le2;
pub mod assert_program_args_stderr_lt2;
pub mod assert_program_args_stderr_ne2;

pub mod assert_program_args_stderr_eq;
pub mod assert_program_args_stderr_ge;
pub mod assert_program_args_stderr_gt;
pub mod assert_program_args_stderr_le;
pub mod assert_program_args_stderr_lt;
pub mod assert_program_args_stderr_ne;

// stderr string
pub mod assert_program_args_stderr_contains;
pub mod assert_program_args_stderr_is_match;
pub mod assert_program_args_stderr_string_contains;
pub mod assert_program_args_stderr_string_is_match;
