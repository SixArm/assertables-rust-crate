//! Before and after examples
//!
//! The Assertables macros can be useful for runtime validation, verification,
//! sanitization, reliability engineering, chaos engineering, and the like.
//!
//! This file has some very simple demonstrations of "before" code that uses
//! just the build-in Rust assert macros, then of "after" code that shows the
//! better Assertables macros.

use assertables::*;

/// Is a number greater than another number?
///
/// This example shows how to get better error messages for infix comparisons.
///
/// Compare the first assert which is Rust standard versus As
///
/// ```ignore
/// let a = 1;
/// let b = 2;
///
/// assert!(a > b);
/// // assertion failed: a > b
///
/// assert_infix!(a > b);
/// // assertion failed: `assert_infix!(a > b)`
/// //  a label: `a`,
/// //  a debug: `1`,
/// //  b label: `b`,
/// //  b debug: `2`
/// ```
///
#[test]
fn greater_than() {

    let a = 1;
    let b = 2;

    // Basic Rust
    assert!(b > a);

    // Assertables
    assert_infix!(b > a);

}

/// Are two numbers near each other?
///
/// This example shows two numbers and some basic math for nearness.
///
/// ```ignore
/// let a = 10;
/// let b = 12;
/// let delta = 1;
///
/// assert!(f64::abs(a - b) <= delta);
/// // assertion failed: i32::abs(a - b) <= delta
///
/// assert_infix!(a, b, delta);
/// // assertion failed: `assert_in_delta!(a, b, delta)`
/// //            a label: `a`,
/// //            a debug: `10`,
/// //            b label: `b`,
/// //            b debug: `12`,
/// //        delta label: `delta`,
/// //        delta debug: `1`,
/// //          | a - b |: `2`,\n",
/// //  | a - b | ≤ delta: false"
/// ```
///
#[test]
fn are_two_numbers_near() {

    let a = 10;
    let b = 12;
    let delta = 2;

    // Basic Rust
    assert!(i32::abs(a - b) <= delta);

    // Assertables
    assert_in_delta!(a, b, delta);

}

/// Compare text file strings.
///
/// This example shows two file system paths, and how to read them into strings.
///
/// ```ignore
/// let a_path = "alfa.txt";
/// let b_path = "bravo.txt";
///
/// assert_eq!(std::fs::read_to_string(a_path).unwrap(), std::fs::read_to_string(b_path).unwrap());
/// // assertion `left == right` failed
/// //   left: "alfa\n"
/// //  right: "bravo\n"
///
/// assert_fs_read_to_string_eq!(a_path, b_path);
/// // assertion failed: `assert_fs_read_to_string_eq!(a_path, b_path)`
/// //   a_path label: `a_path`,
/// //   a_path debug: `"alfa.txt"`,
/// //   b_path label: `b_path`,
/// //   b_path debug: `"bravo.txt"`,
/// //       a string: `"alfa\n"`,
/// //       b string: `"bravo\n"`
/// ```
///
#[test]
fn compare_text_file_strings() {

    let a_path = "alfa.txt";
    let b_path = "alfa.txt";

    // Basic Rust
    assert_eq!(std::fs::read_to_string(a_path).unwrap(), std::fs::read_to_string(b_path).unwrap());

    // Assertables
    assert_fs_read_to_string_eq!(a_path, b_path);

}

/// Verify a command standard output stream contains a target string.
///
/// This example shows how to create a command, run it, and capture the output.
///
/// ```ignore
/// use std::process::Command;
/// let mut command = Command::new("echo");
/// command.args(["alfa"]);
/// let containee = "zz";
///
/// assert!(String::from_utf8(command.output().unwrap().stdout).unwrap().contains(&containee));
/// // assertion failed: String::from_utf8(command.output().unwrap().stdout).unwrap().contains(containee)
///
/// assert_command_stdout_contains!(command, containee);
/// // assertion failed: `assert_command_stdout_contains!(command, containee)`
/// //    command label: `command`,
/// //    command debug: `"echo" "alfa"`,
/// //  containee label: `containee`,
/// //  containee debug: `"zz"`,
/// //           stdout: `"alfa\n"`
/// ```
///
#[test]
fn verify_command_stdout_contains_text() {

    use std::process::Command;
    let mut command = Command::new("echo");
    command.args(["alfa"]);
    let containee = "fa";

    // Basic Rust
    assert!(String::from_utf8(command.output().unwrap().stdout).unwrap().contains(containee));

    // Assertables
    assert_command_stdout_contains!(command, containee);

}
