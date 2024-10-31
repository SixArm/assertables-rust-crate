//! Starters: examples of Assertables macros with many typical uses.
//!
//! The Assertables macros can be useful for runtime validation, verification,
//! sanitization, reliability engineering, chaos engineering, and the like.
//!
//! This file has typical demonstrations of the most-common use cases.
//! 
//! In most of these examples, the real value of the Assertables macros happens
//! when a test fails, because you get better failure messages, which help you
//! find bugs faster, and confirm fixes more easily.

use assertables::*;
use regex::Regex;
use std::path::PathBuf;
use std::sync::LazyLock;
use std::process::Command;

pub static BIN: LazyLock<PathBuf> = LazyLock::new(|| {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("bin")
});

/// Examples with values.
#[test]
fn examples_with_values() {
    assert_eq!(1, 1); // equal to
    assert_ne!(2, 1); // not equal to
    assert_lt!(1, 2); // less than
    assert_le!(1, 2); // less than or equal to
    assert_gt!(2, 1); // greater than
    assert_ge!(2, 1); // greater than or equal to
}

/// Examples with infix order operators.
#[test]
fn examples_with_infix_order_operators() {
    assert_infix!(1 == 1);  // equal to
    assert_infix!(1 != 2);  // not equal to
    assert_infix!(1 < 2);   // less than
    assert_infix!(1 <= 2);  // less than or equal to
    assert_infix!(2 > 1);   // greater than
    assert_infix!(2 >= 1);  // greater han or equal to
}

/// Examples with infix logic operators.
#[test]
fn examples_with_infix_logic_operators() {
    assert_infix!(true & true);   // logic "and"
    assert_infix!(true && true);  // logic lazy "and"
    assert_infix!(false | true);  // logic "or"
    assert_infix!(false || true); // logic lazy "or"
    assert_infix!(false ^ true);  // logic "xor"
}

/// Examples with strings.
#[test]
fn examples_with_strings() {
    let s = "hello world";
    assert_matches!(s, "hello world"); 
    assert_is_match!(Regex::new(r"h.* w.*").unwrap(), s);
    assert_starts_with!(s, "hello");
    assert_ends_with!(s, "world");
    assert_contains!(s, "lo");
    assert_len_eq!(s, "***********");
    assert_len_eq_x!(s, 11);
    assert_all!(s.chars(), |c: char| c < 'x');
    assert_any!(s.chars(), |c: char| c.is_whitespace());
}

/// Examples with numeric nearness.
#[test]
fn examples_with_numeric_nearness() {
    assert_approx_eq!(1.00000001, 1.00000002);
    assert_in_delta!(1, 1, 0);
    assert_in_epsilon!(1, 1, 0);
}

/// Examples with absolute difference
#[test]
fn examples_with_absolute_difference() {
    assert_abs_diff_eq!(10, 13, 3);
    assert_abs_diff_ne!(10, 13, 0);
    assert_abs_diff_lt!(10, 13, 4);
    assert_abs_diff_le!(10, 13, 4);
    assert_abs_diff_gt!(10, 13, 2);
    assert_abs_diff_ge!(10, 13, 2);
}

/// Examples with assert_starts_with.
#[test]
fn examples_with_assert_starts_with() {
    assert_starts_with!("alfa", "al");
    assert_not_starts_with!("alfa", "zz");
}

/// Examples with assert_ends_with.
#[test]
fn examples_with_assert_ends_with() {
    assert_ends_with!("alfa", "fa");
    assert_not_ends_with!("alfa", "zz");
}

/// Examples with assert_contains.
#[test]
fn examples_with_assert_contains() {
    assert_contains!("alfa", "lf");
    assert_not_contains!("alfa", "zz");
}

/// Examples with assert_len.
#[test]
fn examples_with_assert_len() {

    // Compare another
    assert_len_eq!("x", "x");
    assert_len_ne!("x", "xx");
    assert_len_lt!("x", "xx");
    assert_len_le!("x", "xx");
    assert_len_gt!("xx", "x");
    assert_len_ge!("xx", "x");

    // Compare expression
    assert_len_eq_x!("x", 1);
    assert_len_ne_x!("x", 2);
    assert_len_lt_x!("x", 2);
    assert_len_le_x!("x", 2);
    assert_len_gt_x!("xx", 1);
    assert_len_ge_x!("xx", 1);

}

/// Examples with assert_count.
#[test]
fn examples_with_assert_count() {

    // Compare another
    assert_count_eq!("x".chars(), "x".chars());
    assert_count_ne!("x".chars(), "xx".chars());
    assert_count_lt!("x".chars(), "xx".chars());
    assert_count_le!("x".chars(), "xx".chars());
    assert_count_gt!("xx".chars(), "x".chars());
    assert_count_ge!("xx".chars(), "x".chars());

    // Compare expression
    assert_count_eq_x!("x".chars(), 1);
    assert_count_ne_x!("x".chars(), 2);
    assert_count_lt_x!("x".chars(), 2);
    assert_count_le_x!("x".chars(), 2);
    assert_count_gt_x!("xx".chars(), 1);
    assert_count_ge_x!("xx".chars(), 1);

}

/// Examples with assert_empty.
#[test]
fn examples_with_assert_is_empty() {
    assert_is_empty!("");
    assert_not_empty!("alfa");
}

/// Examples with assert_is_match.
#[test]
fn examples_with_assert_is_match() {
    assert_is_match!(Regex::new(r"lf").unwrap(), "alfa");
    assert_not_match!(Regex::new(r"zz").unwrap(), "alfa");
}

/// Examples with assert_ok.
#[test]
fn examples_with_assert_ok() {
    let a: Result<i8, i8> = Ok(1);
    let b: Result<i8, i8> = Ok(2);
    assert_ok!(a);

    // Compare another
    assert_ok_eq!(a, a);
    assert_ok_ne!(a, b);

    // Compare expression
    assert_ok_eq_x!(a, 1);
    assert_ok_ne_x!(a, 2);
}

/// Examples with assert_err.
#[test]
fn examples_with_assert_err() {
    let a: Result<i8, i8> = Err(1);
    let b: Result<i8, i8> = Err(2);
    assert_err!(a);

    // Compare another
    assert_err_eq!(a, a);
    assert_err_ne!(a, b);

    // Compare expression
    assert_err_eq_x!(a, 1);
    assert_err_ne_x!(a, 2);
}

/// Examples with assert_some.
#[test]
fn examples_with_assert_some() {
    let a: Option<i8> = Some(1);
    let b: Option<i8> = Some(2);
    assert_some!(a);
 
    // Compare another
    assert_some_eq!(a, a);
    assert_some_ne!(a, b);
    
    // Compare expression
    assert_some_eq_x!(a, 1);
    assert_some_ne_x!(a, 2);
}

/// Examples with assert_none.
#[test]
fn examples_with_assert_none() {
    let a: Option<i8> = Option::None;
    assert_none!(a);
}

//// Poll Ready/Pending

use std::task::Poll;
use std::task::Poll::*;

/// Examples with assert_ready.
#[test]
fn examples_with_assert_ready() {
    let a: Poll<i8> = Ready(1);
    let b: Poll<i8> = Ready(2);
    assert_ready!(a);

    // Compare another
    assert_ready_eq!(a, a);
    assert_ready_ne!(a, b);

    // Compare expression
    assert_ready_eq_x!(a, 1);
    assert_ready_ne_x!(a, 2);
}

/// Examples with assert_pending.
#[test]
fn examples_with_assert_pending() {
    let a: Poll<i8> = Pending;
    assert_pending!(a);
}

//// Collections

/// Examples with assert_iter.
#[test]
fn examples_with_assert_iter() {
    assert_all!([1, 2, 3].into_iter(), |x: i8| x > 0);
    assert_any!([1, 2, 3].into_iter(), |x: i8| x > 0);
    assert_iter_eq!([1], [1]);
    assert_iter_ne!([1], [2]);
    assert_iter_lt!([1, 2], [3, 4]);
    assert_iter_le!([1, 2], [3, 4]);
    assert_iter_gt!([3, 4], [1, 2]);
    assert_iter_ge!([3, 4], [1, 2]);
}

/// Examples with assert_bag.
#[test]
fn examples_with_assert_bag() {
    assert_bag_eq!([1], [1]);
    assert_bag_ne!([1], [2]);
    assert_bag_subbag!([1], [1, 2]);
    assert_bag_superbag!([1, 2], [1]);
}

/// Examples with assert_set.
#[test]
fn examples_with_assert_set() {
    assert_set_eq!([1], [1]);
    assert_set_ne!([1], [2]);
    assert_set_subset!([1], [1, 2]);
    assert_set_superset!([1, 2], [1]);
    assert_set_joint!([1], [1]);
    assert_set_disjoint!([1], [2]);
}

//// Commands

/// Examples with assert_command_stdout.
#[test]
fn examples_with_assert_command_stdout() {

    let program = BIN.join("printf-stdout");
    let mut a = Command::new(&program);
    a.args(["%s", "alfa"]);

    // Compare another
    let mut b = Command::new(&program); b.args(["%s%s%s%s", "a", "l", "f", "a"]); assert_command_stdout_eq!(a, b);
    let mut b = Command::new(&program); b.args(["%s%s%s%s", "x", "x"]); assert_command_stdout_ne!(a, b);
    let mut b = Command::new(&program); b.args(["%s%s%s%s", "z", "z"]); assert_command_stdout_lt!(a, b);
    let mut b = Command::new(&program); b.args(["%s%s%s%s", "z", "z"]); assert_command_stdout_le!(a, b);
    let mut b = Command::new(&program); b.args(["%s%s%s%s", "a", "a"]); assert_command_stdout_gt!(a, b);
    let mut b = Command::new(&program); b.args(["%s%s%s%s", "a", "a"]); assert_command_stdout_ge!(a, b);

    // Compare expression
    assert_command_stdout_eq_x!(a, vec![b'a', b'l', b'f', b'a']);
    assert_command_stdout_ne_x!(a, vec![b'x', b'x']);
    assert_command_stdout_lt_x!(a, vec![b'z', b'z']);
    assert_command_stdout_le_x!(a, vec![b'z', b'z']);
    assert_command_stdout_gt_x!(a, vec![b'a', b'a']);
    assert_command_stdout_ge_x!(a, vec![b'a', b'a']);

    // Matching
    assert_command_stdout_string_contains!(a, "lf");
    assert_command_stdout_string_is_match!(a, Regex::new(r"lf").unwrap());

}

/// Examples with assert_command_stderr.
#[test]
fn examples_with_assert_command_stderr() {

    let program = BIN.join("printf-stderr");
    let mut a = Command::new(&program);
    a.args(["%s", "alfa"]);

    // Compare another
    let mut b = Command::new(&program); b.args(["%s%s%s%s", "a", "l", "f", "a"]); assert_command_stderr_eq!(a, b);
    let mut b = Command::new(&program); b.args(["%s%s%s%s", "x", "x"]); assert_command_stderr_ne!(a, b);
    let mut b = Command::new(&program); b.args(["%s%s%s%s", "z", "z"]); assert_command_stderr_lt!(a, b);
    let mut b = Command::new(&program); b.args(["%s%s%s%s", "z", "z"]); assert_command_stderr_le!(a, b);
    let mut b = Command::new(&program); b.args(["%s%s%s%s", "a", "a"]); assert_command_stderr_gt!(a, b);
    let mut b = Command::new(&program); b.args(["%s%s%s%s", "a", "a"]); assert_command_stderr_ge!(a, b);

    // Compare expression
    assert_command_stderr_eq_x!(a, vec![b'a', b'l', b'f', b'a']);
    assert_command_stderr_ne_x!(a, vec![b'x', b'x']);
    assert_command_stderr_lt_x!(a, vec![b'z', b'z']);
    assert_command_stderr_le_x!(a, vec![b'z', b'z']);
    assert_command_stderr_gt_x!(a, vec![b'a', b'a']);
    assert_command_stderr_ge_x!(a, vec![b'a', b'a']);

    // Matching
    assert_command_stderr_string_contains!(a, "lf");
    assert_command_stderr_string_is_match!(a, Regex::new(r"lf").unwrap());

}

//// Programs & Args

/// Examples with assert_program_args_stdout.
#[test]
fn examples_with_assert_program_args_stdout() {

    let a_program = BIN.join("printf-stdout");
    let b_program = BIN.join("printf-stdout");
    let a_args = ["%s", "alfa"];

    // Compare another
    assert_program_args_stdout_eq!(&a_program, &a_args, &b_program, ["%s%s%s%s", "a", "l", "f", "a"]);
    assert_program_args_stdout_ne!(&a_program, &a_args, &b_program, ["%s%s%s%s%s", "x"]);
    assert_program_args_stdout_lt!(&a_program, &a_args, &b_program, ["%s%s%s%s%s", "z", "z"]);
    assert_program_args_stdout_le!(&a_program, &a_args, &b_program, ["%s%s%s%s%s", "z", "z"]);
    assert_program_args_stdout_gt!(&a_program, &a_args, &b_program, ["%s%s%s%s%s", "a", "a"]);
    assert_program_args_stdout_ge!(&a_program, &a_args, &b_program, ["%s%s%s%s%s", "a", "a"]);

    // Compare expression
    assert_program_args_stdout_eq_x!(&a_program, &a_args, vec![b'a', b'l', b'f', b'a']);
    assert_program_args_stdout_ne_x!(&a_program, &a_args, vec![b'x']);
    assert_program_args_stdout_lt_x!(&a_program, &a_args, vec![b'z', b'z']);
    assert_program_args_stdout_le_x!(&a_program, &a_args, vec![b'z', b'z']);
    assert_program_args_stdout_gt_x!(&a_program, &a_args, vec![b'a', b'a']);
    assert_program_args_stdout_ge_x!(&a_program, &a_args, vec![b'a', b'a']);

    // Matching
    assert_program_args_stdout_string_contains!(&a_program, &a_args, "lf");
    assert_program_args_stdout_string_is_match!(&a_program, &a_args, Regex::new(r"lf").unwrap());

}

/// Examples with assert_program_args_stderr.
#[test]
fn examples_with_assert_program_args_stderr() {

    let a_program = BIN.join("printf-stderr");
    let b_program = BIN.join("printf-stderr");
    let a_args = ["%s", "alfa"];

    // Compare another
    assert_program_args_stderr_eq!(&a_program, &a_args, &b_program, ["%s%s%s%s", "a", "l", "f", "a"]);
    assert_program_args_stderr_ne!(&a_program, &a_args, &b_program, ["%s%s%s%s%s", "x"]);
    assert_program_args_stderr_lt!(&a_program, &a_args, &b_program, ["%s%s%s%s%s", "z", "z"]);
    assert_program_args_stderr_le!(&a_program, &a_args, &b_program, ["%s%s%s%s%s", "z", "z"]);
    assert_program_args_stderr_gt!(&a_program, &a_args, &b_program, ["%s%s%s%s%s", "a", "a"]);
    assert_program_args_stderr_ge!(&a_program, &a_args, &b_program, ["%s%s%s%s%s", "a", "a"]);

    // Compare expression
    assert_program_args_stderr_eq_x!(&a_program, &a_args, vec![b'a', b'l', b'f', b'a']);
    assert_program_args_stderr_ne_x!(&a_program, &a_args, vec![b'x']);
    assert_program_args_stderr_lt_x!(&a_program, &a_args, vec![b'z', b'z']);
    assert_program_args_stderr_le_x!(&a_program, &a_args, vec![b'z', b'z']);
    assert_program_args_stderr_gt_x!(&a_program, &a_args, vec![b'a', b'a']);
    assert_program_args_stderr_ge_x!(&a_program, &a_args, vec![b'a', b'a']);

    // Matching
    assert_program_args_stderr_string_contains!(&a_program, &a_args, "lf");
    assert_program_args_stderr_string_is_match!(&a_program, &a_args, Regex::new(r"lf").unwrap());

}

/// Examples with assert_process_status_code_value_*
#[test]
fn examples_with_assert_process_status_code_value() {

    let program = BIN.join("exit-with-arg");
    let mut a = Command::new(&program);
    a.arg("1");

    // Compare another
    let mut b = Command::new(&program); b.arg("1"); assert_process_status_code_value_eq!(a, b);
    let mut b = Command::new(&program); b.arg("2"); assert_process_status_code_value_ne!(a, b);
    let mut b = Command::new(&program); b.arg("2"); assert_process_status_code_value_lt!(a, b);
    let mut b = Command::new(&program); b.arg("2"); assert_process_status_code_value_le!(a, b);
    let mut b = Command::new(&program); b.arg("0"); assert_process_status_code_value_gt!(a, b);
    let mut b = Command::new(&program); b.arg("0"); assert_process_status_code_value_ge!(a, b);

    // Compare expression
    assert_process_status_code_value_eq_x!(a, 1);
    assert_process_status_code_value_ne_x!(a, 2);
    assert_process_status_code_value_lt_x!(a, 2);
    assert_process_status_code_value_le_x!(a, 2);
    assert_process_status_code_value_gt_x!(a, 0);
    assert_process_status_code_value_ge_x!(a, 0);

}

//// Functions

/// Examples with assert_fn.
#[test]
fn examples_with_assert_fn() {

    // Functions
    fn f() -> i8 { 1 }
    fn g() -> i8 { 2 }

    // Compare another
    assert_fn_eq!(f, f);
    assert_fn_ne!(f, g);
    assert_fn_lt!(f, g);
    assert_fn_le!(f, g);
    assert_fn_gt!(g, f);
    assert_fn_ge!(g, f);

    // Compare expression
    assert_fn_eq_x!(f, 1);
    assert_fn_ne_x!(f, 2);
    assert_fn_lt_x!(f, 2);
    assert_fn_le_x!(f, 2);
    assert_fn_gt_x!(g, 1);
    assert_fn_ge_x!(g, 1);


}

/// Examples with assert_fn_ok.
#[test]
fn examples_with_assert_fn_ok() {

    // Functions
    fn f() -> Result<i8, i8> { Ok(1) }
    fn g() -> Result<i8, i8> { Ok(2) }

    // Compare another
    assert_fn_ok_eq!(f, f);
    assert_fn_ok_ne!(f, g);
    assert_fn_ok_lt!(f, g);
    assert_fn_ok_le!(f, g);
    assert_fn_ok_gt!(g, f);
    assert_fn_ok_ge!(g, f);

    // Compare expression
    assert_fn_ok_eq_x!(f, 1);
    assert_fn_ok_ne_x!(f, 2);
    assert_fn_ok_lt_x!(f, 2);
    assert_fn_ok_le_x!(f, 2);
    assert_fn_ok_gt_x!(g, 1);
    assert_fn_ok_ge_x!(g, 1);
}

/// Examples with assert_fn_err.
#[test]
fn examples_with_assert_fn_err() {

    // Functions
    fn f() -> Result<i8, i8> { Err(1) }
    fn g() -> Result<i8, i8> { Err(2) }

    // Compare another
    assert_fn_err_eq!(f, f);
    assert_fn_err_ne!(f, g);
    assert_fn_err_lt!(f, g);
    assert_fn_err_le!(f, g);
    assert_fn_err_gt!(g, f);
    assert_fn_err_ge!(g, f);

    // Compare expression
    assert_fn_err_ne_x!(f, 2);
    assert_fn_err_eq_x!(f, 1);
    assert_fn_err_lt_x!(f, 2);
    assert_fn_err_le_x!(f, 2);
    assert_fn_err_gt_x!(g, 1);
    assert_fn_err_ge_x!(g, 1);

    
}

//// Readers

/// Examples with assert_fs_read_to_string.
#[test]
fn examples_with_assert_fs_read_to_string() {

    // Compare another
    assert_fs_read_to_string_eq!("alfa.txt", "alfa.txt");
    assert_fs_read_to_string_ne!("alfa.txt", "bravo.txt");
    assert_fs_read_to_string_lt!("alfa.txt", "bravo.txt");
    assert_fs_read_to_string_le!("alfa.txt", "bravo.txt");
    assert_fs_read_to_string_gt!("bravo.txt", "alfa.txt");
    assert_fs_read_to_string_ge!("bravo.txt", "alfa.txt");

    // Compare expression
    assert_fs_read_to_string_eq_x!("alfa.txt", "alfa\n");
    assert_fs_read_to_string_ne_x!("alfa.txt", "z");
    assert_fs_read_to_string_lt_x!("alfa.txt", "b");
    assert_fs_read_to_string_le_x!("alfa.txt", "b");
    assert_fs_read_to_string_ge_x!("bravo.txt", "a");
    assert_fs_read_to_string_gt_x!("bravo.txt", "a");

    // Matching
    assert_fs_read_to_string_contains!("alfa.txt", "lf");
    assert_fs_read_to_string_is_match!("alfa.txt", Regex::new("lf").unwrap());

}

/// Examples with assert_io_read_to_string.
#[test]
fn examples_with_assert_io_read_to_string() {
    use std::io::Read;
    let mut a: &[u8];
    let mut b: &[u8];

    // Compare another
    a = "alfa".as_bytes(); b = "alfa".as_bytes();  assert_io_read_to_string_eq!(a, b);
    a = "alfa".as_bytes(); b = "bravo".as_bytes(); assert_io_read_to_string_ne!(a, b);
    a = "alfa".as_bytes(); b = "bravo".as_bytes(); assert_io_read_to_string_lt!(a, b);
    a = "alfa".as_bytes(); b = "bravo".as_bytes(); assert_io_read_to_string_le!(a, b);
    a = "alfa".as_bytes(); b = "bravo".as_bytes(); assert_io_read_to_string_gt!(b, a);
    a = "alfa".as_bytes(); b = "bravo".as_bytes(); assert_io_read_to_string_ge!(b, a);

    // Compare expression
    a = "alfa".as_bytes(); assert_io_read_to_string_eq_x!(a, "alfa");
    a = "alfa".as_bytes(); assert_io_read_to_string_ne_x!(a, "b");
    a = "alfa".as_bytes(); assert_io_read_to_string_lt_x!(a, "b");
    a = "alfa".as_bytes(); assert_io_read_to_string_le_x!(a, "b");
    a = "alfa".as_bytes(); assert_io_read_to_string_gt_x!(a, "a");
    a = "alfa".as_bytes(); assert_io_read_to_string_ge_x!(a, "a");

    // Matching
    a = "alfa".as_bytes(); assert_io_read_to_string_contains!(a, "lf");
    a = "alfa".as_bytes(); assert_io_read_to_string_is_match!(a, Regex::new("lf").unwrap());

}

//// Success returns

/// Examples of success return with results.
#[test]
fn examples_of_success_return_with_results() {

    // Compare another
    let a_result: Result<i8, i8> = Ok(1);
    let b_result: Result<i8, i8> = Ok(2);
    let (a_inner, b_inner) = assert_ok_ne!(a_result, b_result);
    assert_eq!(a_inner, 1);
    assert_eq!(b_inner, 2);

    // Compare expression
    let result: Result<i8, i8> = Ok(2);
    let inner = assert_ok_ne_x!(result, 1);
    assert_eq!(inner, 2);

}

/// Examples of success return with file strings.
#[test]
fn examples_of_success_return_with_files() {

    // Compare another
    let a_path = "alfa.txt";
    let b_path = "bravo.txt";
    let (a_string, b_string) = assert_fs_read_to_string_ne!(a_path, b_path);
    assert_eq!(a_string, String::from("alfa\n"));
    assert_eq!(b_string, String::from("bravo\n"));
    
    // Compare expression
    let path = "alfa.txt";
    let string = assert_fs_read_to_string_ne_x!(path, "");
    assert_eq!(string, String::from("alfa\n"));

}

/// Examples of success return with byte strings.
#[test]
fn examples_of_success_return_with_bytes() {
    use std::io::Read;

    // Compare another
    let mut a = "alfa".as_bytes();
    let mut b = "bravo".as_bytes();
    let (a_string, b_string) = assert_io_read_to_string_ne!(a, b);
    assert_eq!(a_string, String::from("alfa"));
    assert_eq!(b_string, String::from("bravo"));
    
    // Compare expression
    let mut a = "alfa".as_bytes();
    let string = assert_io_read_to_string_ne_x!(a, "");
    assert_eq!(string, String::from("alfa"));

}

/// Examples of success return with commands.
#[test]
fn examples_of_success_return_with_commands() {
    use std::process::Command;
    let program = BIN.join("printf-stdout");

    // Compare another
    let mut a_command = Command::new(&program);
    a_command.args(["%s", "alfa"]);
    let mut b_command = Command::new(&&program);
    b_command.args(["%s", "bravo"]);
    let (a_stdout, b_stdout) = assert_command_stdout_ne!(a_command, b_command);
    assert_eq!(a_stdout, vec![b'a', b'l', b'f', b'a']);
    assert_eq!(b_stdout, vec![b'b', b'r', b'a', b'v', b'o']);

    // Compare expression
    let mut command = Command::new(&program);
    command.args(["%s", "alfa"]);
    let stdout = assert_command_stdout_ne_x!(command, vec![b'z']);
    assert_eq!(stdout, vec![b'a', b'l', b'f', b'a']);

}
