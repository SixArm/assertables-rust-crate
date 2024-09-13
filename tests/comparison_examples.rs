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
#[test]
fn greater_than() {

    // Basic Rust
    assert!(2 > 1);
    
    // Assertables
    assert_infix!(2 > 1);

}

/// Are two numbers near each other?
/// 
/// This example shows two numbers and some basic math for nearness.
/// 
#[test]
fn are_two_numbers_near() {

    let a = 10.0;
    let b = 11.0;
    let delta = 1.5;

    // Basic Rust
    assert!(Math::abs(a - b) <= delta);
    
    // Assertables
    assert_in_delta!(a, b, delta);

}

/// Compare text file strings.
/// 
/// This example shows two file system paths, and how to read them into strings.
/// 
#[test]
fn compare_text_file_strings() {

    let path1 = "alfa1.txt";
    let path2 = "alfa2.txt";

    // Basic Rust
    assert_eq!(std::fs::read_to_string(path1).unwrap(), std::fs::read_to_string(path2).unwrap());

    // Assertables
    assert_fs_read_to_string_eq!(path1, path2);

}

/// Verify a command standard output stream contains a target string.
/// 
/// This example shows how to create a command, run it, and capture the output.
/// 
#[test]
fn verify_command_stdout_contains_str() {

    use std::process::Command;
    let mut command = Command::new("echo");
    command.args(["alfa"]);

    // Basic Rust
    assert!(String::from_utf8(command.output().unwrap().stdout).unwrap().contains("fa"));

    // Assertables
    assert_command_stdout_contains!(command, "fa");

}
