//! Tutorial examples: Assertables macro examples with many typical uses.
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

/// Assertables macro examples with values.
#[test]
fn examples_with_values() {
    assert_eq!(1, 1);
    assert_ne!(2, 1);
    assert_lt!(1, 2);
    assert_le!(1, 2);
    assert_gt!(2, 1);
    assert_ge!(2, 1);
}

/// Assertables macro examples with strings.
#[test]
fn examples_with_strings() {
    let s = "hello world";
    assert_matches!(s, "hello world");
    assert_is_match!(Regex::new(r"h.* w.*").unwrap(), s);
    assert_starts_with!(s, "hello");
    assert_ends_with!(s, "world");
    assert_contains!(s, "o");
    assert_len_eq!(s, 11);
    assert_all!(s.chars(), |c: char| c < 'x');
    assert_any!(s.chars(), |c: char| c.is_whitespace());

}

/// Assertables macro examples with infix order operators.
#[test]
fn examples_with_infix_order_operators() {
    assert_infix!(1 == 1);
    assert_infix!(1 != 2);
    assert_infix!(1 < 2);
    assert_infix!(1 <= 2);
    assert_infix!(2 > 1);
    assert_infix!(2 >= 1);
}

/// Assertables macro examples with infix logic operators.
#[test]
fn examples_with_infix_logic_operators() {
    assert_infix!(true & true);
    assert_infix!(true && true);
    assert_infix!(false | true);
    assert_infix!(false || true);
    assert_infix!(false ^ true);
}

/// Assertables macro examples with numeric nearness.
#[test]
fn examples_with_numeric_nearness() {
    assert_approx_eq!(1.00000001, 1.00000002);
    assert_in_delta!(1, 1, 0);
    assert_in_epsilon!(1, 1, 0);
}

/// Assertables macro examples with assert_starts_with.
#[test]
fn examples_with_assert_starts_with() {
    assert_starts_with!("alfa", "al");
    assert_not_starts_with!("alfa", "zz");
}

/// Assertables macro examples with assert_ends_with.
#[test]
fn examples_with_assert_ends_with() {
    assert_ends_with!("alfa", "fa");
    assert_not_ends_with!("alfa", "zz");
}

/// Assertables macro examples with assert_len.
#[test]
fn examples_with_assert_len() {
    //// Solo
    assert_len_eq!("x", 1);
    assert_len_ne!("x", 2);
    assert_len_lt!("x", 2);
    assert_len_le!("x", 2);
    assert_len_gt!("xx", 1);
    assert_len_ge!("xx", 1);
    //// Pair
    assert_len_eq2!("x", "x");
    assert_len_ne2!("x", "xx");
    assert_len_lt2!("x", "xx");
    assert_len_le2!("x", "xx");
    assert_len_gt2!("xx", "x");
    assert_len_ge2!("xx", "x");
}

/// Assertables macro examples with assert_count.
#[test]
fn examples_with_assert_count() {
    //// Solo
    assert_count_eq!("x".chars(), 1);
    assert_count_ne!("x".chars(), 2);
    assert_count_lt!("x".chars(), 2);
    assert_count_le!("x".chars(), 2);
    assert_count_gt!("xx".chars(), 1);
    assert_count_ge!("xx".chars(), 1);
    //// Pairs
    assert_count_eq2!("x".chars(), "x".chars());
    assert_count_ne2!("x".chars(), "xx".chars());
    assert_count_lt2!("x".chars(), "xx".chars());
    assert_count_le2!("x".chars(), "xx".chars());
    assert_count_gt2!("xx".chars(), "x".chars());
    assert_count_ge2!("xx".chars(), "x".chars());
}

/// Assertables macro examples with assert_empty.
#[test]
fn examples_with_assert_is_empty() {
    assert_is_empty!("");
    assert_not_empty!("alfa");
}

/// Assertables macro examples with assert_contains.
#[test]
fn examples_with_assert_contains() {
    assert_contains!("alfa", "lf");
    assert_not_contains!("alfa", "zz");
}

/// Assertables macro examples with assert_is_match.
#[test]
fn examples_with_assert_is_match() {
    assert_is_match!(Regex::new(r"lf").unwrap(), "alfa");
    assert_not_match!(Regex::new(r"zz").unwrap(), "alfa");
}

/// Assertables macro examples with assert_ok.
#[test]
fn examples_with_assert_ok() {

    //// Solo
    let a: Result<i8, i8> = Result::Ok(1);
    assert_ok!(a);

    //// Solo eq
    let a: Result<i8, i8> = Result::Ok(1);
    assert_ok_eq!(a, 1);
    
    //// Pair eq
    let a: Result<i8, i8> = Result::Ok(1);
    let b: Result<i8, i8> = Result::Ok(1);
    assert_ok_eq2!(a, b);
    
    //// Solo ne
    let a: Result<i8, i8> = Result::Ok(1);
    assert_ok_ne!(a, 2);
    
    //// Pair ne
    let a: Result<i8, i8> = Result::Ok(1);
    let b: Result<i8, i8> = Result::Ok(2);
    assert_ok_ne2!(a, b);

}

/// Assertables macro examples with assert_err.
#[test]
fn examples_with_assert_err() {

    //// Solo
    let a: Result<i8, i8> = Result::Err(1);
    assert_err!(a);

    //// Solo eq
    let a: Result<i8, i8> = Result::Err(1);
    assert_err_eq!(a, 1);
    
    //// Pair eq
    let a: Result<i8, i8> = Result::Err(1);
    let b: Result<i8, i8> = Result::Err(1);
    assert_err_eq2!(a, b);
    
    //// Solo ne
    let a: Result<i8, i8> = Result::Err(1);
    assert_err_ne!(a, 2);
    
    //// Pair ne
    let a: Result<i8, i8> = Result::Err(1);
    let b: Result<i8, i8> = Result::Err(2);
    assert_err_ne2!(a, b);

}

/// Assertables macro examples with assert_some.
#[test]
fn examples_with_assert_some() {

    //// Solo
    let a: Option<i8> = Option::Some(1);
    assert_some!(a);

    //// Solo eq
    let a: Option<i8> = Option::Some(1);
    assert_some_eq!(a, 1);
    
    //// Pair eq
    let a: Option<i8> = Option::Some(1);
    let b: Option<i8> = Option::Some(1);
    assert_some_eq2!(a, b);
    
    //// Solo ne
    let a: Option<i8> = Option::Some(1);
    assert_some_ne!(a, 2);
    
    //// Pair ne
    let a: Option<i8> = Option::Some(1);
    let b: Option<i8> = Option::Some(2);
    assert_some_ne2!(a, b);

}

/// Assertables macro examples with assert_none.
#[test]
fn examples_with_assert_none() {
    let a: Option<i8> = Option::None;
    assert_none!(a);
}

//// Poll Ready/Pending

use std::task::Poll;
use std::task::Poll::*;

/// Assertables macro examples with assert_ready.
#[test]
fn examples_with_assert_ready() {
    
    //// Solo
    let a: Poll<i8> = Poll::Ready(1);
    assert_ready!(a);

    //// Solo eq
    let a: Poll<i8> = Poll::Ready(1);
    assert_ready_eq!(a, 1);
    
    //// Pair eq
    let a: Poll<i8> = Poll::Ready(1);
    let b: Poll<i8> = Poll::Ready(1);
    assert_ready_eq2!(a, b);
    
    //// Solo ne
    let a: Poll<i8> = Poll::Ready(1);
    assert_ready_ne!(a, 2);
    
    //// Pair ne
    let a: Poll<i8> = Poll::Ready(1);
    let b: Poll<i8> = Poll::Ready(2);
    assert_ready_ne2!(a, b);

}

/// Assertables macro examples with assert_pending.
#[test]
fn examples_with_assert_pending() {
    let a: Poll<i8> = Pending;
    assert_pending!(a);
}

//// Collections

/// Assertables macro examples with assert_iter.
#[test]
fn examples_with_assert_iter() {
    assert_all!([1, 2, 3].into_iter(), |x: i8| x > 0);
    assert_any!([1, 2, 3].into_iter(), |x: i8| x > 0);
    assert_iter_eq2!([1], [1]);
    assert_iter_ne2!([1], [2]);
    assert_iter_lt2!([1, 2], [3, 4]);
    assert_iter_le2!([1, 2], [3, 4]);
    assert_iter_gt2!([3, 4], [1, 2]);
    assert_iter_ge2!([3, 4], [1, 2]);
}

/// Assertables macro examples with assert_bag.
#[test]
fn examples_with_assert_bag() {
    assert_bag_eq2!([1], [1]);
    assert_bag_ne2!([1], [2]);
    assert_bag_subbag!([1], [1, 2]);
    assert_bag_superbag!([1, 2], [1]);
}

/// Assertables macro examples with assert_set.
#[test]
fn examples_with_assert_set() {
    assert_set_eq2!([1], [1]);
    assert_set_ne2!([1], [2]);
    assert_set_subset!([1], [1, 2]);
    assert_set_superset!([1, 2], [1]);
    assert_set_joint!([1], [1]);
    assert_set_disjoint!([1], [2]);
}

//// Commands

/// Assertables macro examples with assert_command_stdout.
#[test]
fn examples_with_assert_command_stdout() {

    let program = BIN.join("printf-stdout");
    let mut a = Command::new(&program);
    a.args(["%s", "alfa"]);

    //// Solo
    assert_command_stdout_eq!(a, vec![b'a', b'l', b'f', b'a']);
    assert_command_stdout_ne!(a, vec![b'x', b'x']);
    assert_command_stdout_lt!(a, vec![b'z', b'z']);
    assert_command_stdout_le!(a, vec![b'z', b'z']);
    assert_command_stdout_gt!(a, vec![b'a', b'a']);
    assert_command_stdout_ge!(a, vec![b'a', b'a']);

    //// Pair
    let mut b = Command::new(&program); b.args(["%s%s%s%s", "a", "l", "f", "a"]); assert_command_stdout_eq2!(a, b);
    let mut b = Command::new(&program); b.args(["%s%s%s%s", "x", "x"]); assert_command_stdout_ne2!(a, b);
    let mut b = Command::new(&program); b.args(["%s%s%s%s", "z", "z"]); assert_command_stdout_lt2!(a, b);
    let mut b = Command::new(&program); b.args(["%s%s%s%s", "z", "z"]); assert_command_stdout_le2!(a, b);
    let mut b = Command::new(&program); b.args(["%s%s%s%s", "a", "a"]); assert_command_stdout_gt2!(a, b);
    let mut b = Command::new(&program); b.args(["%s%s%s%s", "a", "a"]); assert_command_stdout_ge2!(a, b);

    //// Matching
    assert_command_stdout_contains!(a, "lf");
    assert_command_stdout_string_contains!(a, "lf");
    assert_command_stdout_is_match!(a, Regex::new(r"lf").unwrap());
    assert_command_stdout_string_is_match!(a, Regex::new(r"lf").unwrap());

}

/// Assertables macro examples with assert_command_stderr.
#[test]
fn examples_with_assert_command_stderr() {

    let program = BIN.join("printf-stderr");
    let mut a = Command::new(&program);
    a.args(["%s", "alfa"]);

    //// Solo
    assert_command_stderr_eq!(a, vec![b'a', b'l', b'f', b'a']);
    assert_command_stderr_ne!(a, vec![b'x', b'x']);
    assert_command_stderr_lt!(a, vec![b'z', b'z']);
    assert_command_stderr_le!(a, vec![b'z', b'z']);
    assert_command_stderr_gt!(a, vec![b'a', b'a']);
    assert_command_stderr_ge!(a, vec![b'a', b'a']);

    //// Pair
    let mut b = Command::new(&program); b.args(["%s%s%s%s", "a", "l", "f", "a"]); assert_command_stderr_eq2!(a, b);
    let mut b = Command::new(&program); b.args(["%s%s%s%s", "x", "x"]); assert_command_stderr_ne2!(a, b);
    let mut b = Command::new(&program); b.args(["%s%s%s%s", "z", "z"]); assert_command_stderr_lt2!(a, b);
    let mut b = Command::new(&program); b.args(["%s%s%s%s", "z", "z"]); assert_command_stderr_le2!(a, b);
    let mut b = Command::new(&program); b.args(["%s%s%s%s", "a", "a"]); assert_command_stderr_gt2!(a, b);
    let mut b = Command::new(&program); b.args(["%s%s%s%s", "a", "a"]); assert_command_stderr_ge2!(a, b);

    //// Matching
    assert_command_stderr_contains!(a, "lf");
    assert_command_stderr_string_contains!(a, "lf");
    assert_command_stderr_is_match!(a, Regex::new(r"lf").unwrap());
    assert_command_stderr_string_is_match!(a, Regex::new(r"lf").unwrap());

}

//// Programs & Args

/// Assertables macro examples with assert_program_args_stdout.
#[test]
fn examples_with_assert_program_args_stdout() {

    let a_program = BIN.join("printf-stdout");
    let a_args = ["%s", "alfa"];

    //// Solo
    assert_program_args_stdout_eq!(&a_program, &a_args, vec![b'a', b'l', b'f', b'a']);
    assert_program_args_stdout_ne!(&a_program, &a_args, vec![b'x']);
    assert_program_args_stdout_lt!(&a_program, &a_args, vec![b'z', b'z']);
    assert_program_args_stdout_le!(&a_program, &a_args, vec![b'z', b'z']);
    assert_program_args_stdout_gt!(&a_program, &a_args, vec![b'a', b'a']);
    assert_program_args_stdout_ge!(&a_program, &a_args, vec![b'a', b'a']);

    let b_program = BIN.join("printf-stdout");

    //// Pair
    assert_program_args_stdout_eq2!(&a_program, &a_args, &b_program, ["%s%s%s%s", "a", "l", "f", "a"]);
    assert_program_args_stdout_ne2!(&a_program, &a_args, &b_program, ["%s%s%s%s%s", "x"]);
    assert_program_args_stdout_lt2!(&a_program, &a_args, &b_program, ["%s%s%s%s%s", "z", "z"]);
    assert_program_args_stdout_le2!(&a_program, &a_args, &b_program, ["%s%s%s%s%s", "z", "z"]);
    assert_program_args_stdout_gt2!(&a_program, &a_args, &b_program, ["%s%s%s%s%s", "a", "a"]);
    assert_program_args_stdout_ge2!(&a_program, &a_args, &b_program, ["%s%s%s%s%s", "a", "a"]);

    //// Matching
    assert_program_args_stdout_contains!(&a_program, &a_args, "lf");
    assert_program_args_stdout_string_contains!(&a_program, &a_args, "lf");
    assert_program_args_stdout_is_match!(&a_program, &a_args, Regex::new(r"lf").unwrap());
    assert_program_args_stdout_string_is_match!(&a_program, &a_args, Regex::new(r"lf").unwrap());

}

/// Assertables macro examples with assert_program_args_stderr.
#[test]
fn examples_with_assert_program_args_stderr() {

    let a_program = BIN.join("printf-stderr");
    let a_args = ["%s", "alfa"];

    //// Solo
    assert_program_args_stderr_eq!(&a_program, &a_args, vec![b'a', b'l', b'f', b'a']);
    assert_program_args_stderr_ne!(&a_program, &a_args, vec![b'x']);
    assert_program_args_stderr_lt!(&a_program, &a_args, vec![b'z', b'z']);
    assert_program_args_stderr_le!(&a_program, &a_args, vec![b'z', b'z']);
    assert_program_args_stderr_gt!(&a_program, &a_args, vec![b'a', b'a']);
    assert_program_args_stderr_ge!(&a_program, &a_args, vec![b'a', b'a']);

    let b_program = BIN.join("printf-stderr");

    //// Pair
    assert_program_args_stderr_eq2!(&a_program, &a_args, &b_program, ["%s%s%s%s", "a", "l", "f", "a"]);
    assert_program_args_stderr_ne2!(&a_program, &a_args, &b_program, ["%s%s%s%s%s", "x"]);
    assert_program_args_stderr_lt2!(&a_program, &a_args, &b_program, ["%s%s%s%s%s", "z", "z"]);
    assert_program_args_stderr_le2!(&a_program, &a_args, &b_program, ["%s%s%s%s%s", "z", "z"]);
    assert_program_args_stderr_gt2!(&a_program, &a_args, &b_program, ["%s%s%s%s%s", "a", "a"]);
    assert_program_args_stderr_ge2!(&a_program, &a_args, &b_program, ["%s%s%s%s%s", "a", "a"]);

    //// Matching
    assert_program_args_stderr_contains!(&a_program, &a_args, "lf");
    assert_program_args_stderr_string_contains!(&a_program, &a_args, "lf");
    assert_program_args_stderr_is_match!(&a_program, &a_args, Regex::new(r"lf").unwrap());
    assert_program_args_stderr_string_is_match!(&a_program, &a_args, Regex::new(r"lf").unwrap());

}

//// Functions

/// Assertables macro examples with assert_fn.
#[test]
fn examples_with_assert_fn() {

    //// Functions
    fn f() -> i8 { 1 }
    fn g() -> i8 { 2 }

    //// Solo
    assert_fn_eq!(f, 1);
    assert_fn_ne!(f, 2);
    assert_fn_lt!(f, 2);
    assert_fn_le!(f, 2);
    assert_fn_gt!(g, 1);
    assert_fn_ge!(g, 1);

    //// Pair
    assert_fn_eq2!(f, f);
    assert_fn_ne2!(f, g);
    assert_fn_lt2!(f, g);
    assert_fn_le2!(f, g);
    assert_fn_gt2!(g, f);
    assert_fn_ge2!(g, f);

}

/// Assertables macro examples with assert_fn_ok.
#[test]
fn examples_with_assert_fn_ok() {

    //// Functions
    fn f() -> Result<i8, i8> { Ok(1) }
    fn g() -> Result<i8, i8> { Ok(2) }

    //// Solo
    assert_fn_ok_eq!(f, 1);
    assert_fn_ok_ne!(f, 2);
    assert_fn_ok_lt!(f, 2);
    assert_fn_ok_le!(f, 2);
    assert_fn_ok_gt!(g, 1);
    assert_fn_ok_ge!(g, 1);

    //// Pair
    assert_fn_ok_eq2!(f, f);
    assert_fn_ok_ne2!(f, g);
    assert_fn_ok_lt2!(f, g);
    assert_fn_ok_le2!(f, g);
    assert_fn_ok_gt2!(g, f);
    assert_fn_ok_ge2!(g, f);

}

/// Assertables macro examples with assert_fn_err.
#[test]
fn examples_with_assert_fn_err() {

    //// Functions
    fn f() -> Result<i8, i8> { Err(1) }
    fn g() -> Result<i8, i8> { Err(2) }

    //// Solo
    assert_fn_err_ne!(f, 2);
    assert_fn_err_eq!(f, 1);
    assert_fn_err_lt!(f, 2);
    assert_fn_err_le!(f, 2);
    assert_fn_err_gt!(g, 1);
    assert_fn_err_ge!(g, 1);

    //// Pair
    assert_fn_err_eq2!(f, f);
    assert_fn_err_ne2!(f, g);
    assert_fn_err_lt2!(f, g);
    assert_fn_err_le2!(f, g);
    assert_fn_err_gt2!(g, f);
    assert_fn_err_ge2!(g, f);

}

//// Readers

/// Assertables macro examples with assert_fs_read_to_string.
#[test]
fn examples_with_assert_fs_read_to_string() {

    //// Solo
    assert_fs_read_to_string_eq!("alfa.txt", "alfa\n");
    assert_fs_read_to_string_ne!("alfa.txt", "z");
    assert_fs_read_to_string_lt!("alfa.txt", "b");
    assert_fs_read_to_string_le!("alfa.txt", "b");
    assert_fs_read_to_string_ge!("bravo.txt", "a");
    assert_fs_read_to_string_gt!("bravo.txt", "a");

    //// Pair
    assert_fs_read_to_string_eq2!("alfa.txt", "alfa.txt");
    assert_fs_read_to_string_ne2!("alfa.txt", "bravo.txt");
    assert_fs_read_to_string_lt2!("alfa.txt", "bravo.txt");
    assert_fs_read_to_string_le2!("alfa.txt", "bravo.txt");
    assert_fs_read_to_string_gt2!("bravo.txt", "alfa.txt");
    assert_fs_read_to_string_ge2!("bravo.txt", "alfa.txt");

    //// Matching
    assert_fs_read_to_string_contains!("alfa.txt", "lf");
    assert_fs_read_to_string_is_match!("alfa.txt", Regex::new("lf").unwrap());

}

/// Assertables macro examples with assert_io_read_to_string.
#[test]
fn examples_with_assert_io_read_to_string() {
    use std::io::Read;

    //// readers
    let mut a: &[u8];
    let mut b: &[u8];

    //// Solo
    a = "alfa".as_bytes(); assert_io_read_to_string_eq!(a, "alfa");
    a = "alfa".as_bytes(); assert_io_read_to_string_ne!(a, "b");
    a = "alfa".as_bytes(); assert_io_read_to_string_lt!(a, "b");
    a = "alfa".as_bytes(); assert_io_read_to_string_le!(a, "b");
    a = "alfa".as_bytes(); assert_io_read_to_string_gt!(a, "a");
    a = "alfa".as_bytes(); assert_io_read_to_string_ge!(a, "a");

    //// Pair
    a = "alfa".as_bytes(); b = "alfa".as_bytes();  assert_io_read_to_string_eq2!(a, b);
    a = "alfa".as_bytes(); b = "bravo".as_bytes(); assert_io_read_to_string_ne2!(a, b);
    a = "alfa".as_bytes(); b = "bravo".as_bytes(); assert_io_read_to_string_lt2!(a, b);
    a = "alfa".as_bytes(); b = "bravo".as_bytes(); assert_io_read_to_string_le2!(a, b);
    a = "alfa".as_bytes(); b = "bravo".as_bytes(); assert_io_read_to_string_gt2!(b, a);
    a = "alfa".as_bytes(); b = "bravo".as_bytes(); assert_io_read_to_string_ge2!(b, a);

    //// Matching
    a = "alfa".as_bytes(); assert_io_read_to_string_contains!(a, "lf");
    a = "alfa".as_bytes(); assert_io_read_to_string_is_match!(a, Regex::new("lf").unwrap());

}

//// Success returns

/// Assertables macro examples of success return with results.
#[test]
fn examples_of_success_return_with_results() {

    //// Solo
    let result: Result<i8, i8> = Ok(2);
    let inner = assert_ok_ne!(result, 1);
    assert_eq!(inner, &2);

    //// Pair
    let a_result: Result<i8, i8> = Ok(1);
    let b_result: Result<i8, i8> = Ok(2);
    let (a_inner, b_inner) = assert_ok_ne2!(a_result, b_result);
    assert_eq!(a_inner, &1);
    assert_eq!(b_inner, &2);

}


/// Assertables macro examples of success return with files.
#[test]
fn examples_of_success_return_with_files() {

    //// Solo
    let path = "alfa.txt";
    let string = assert_fs_read_to_string_ne!(path, "");
    assert_eq!(string, String::from("alfa\n"));

    //// Pair
    let a_path = "alfa.txt";
    let b_path = "bravo.txt";
    let (a_string, b_string) = assert_fs_read_to_string_ne2!(a_path, b_path);
    assert_eq!(a_string, String::from("alfa\n"));
    assert_eq!(b_string, String::from("bravo\n"));
    
}

/// Assertables macro examples of success return with commands.
#[test]
fn examples_of_success_return_with_commands() {
    use std::process::Command;
    let program = crate::BIN.join("printf-stdout");

    //// Solo
    let mut command = Command::new(&program);
    command.args(["%s", "alfa"]);
    let stdout = assert_command_stdout_ne!(command, vec![b'z']);
    assert_eq!(stdout, vec![b'a', b'l', b'f', b'a']);

    //// Pair
    let program = crate::BIN.join("printf-stdout");
    let mut a_command = Command::new(&program);
    a_command.args(["%s", "alfa"]);
    let mut b_command = Command::new(&&program);
    b_command.args(["%s", "bravo"]);
    let (a_stdout, b_stdout) = assert_command_stdout_ne2!(a_command, b_command);
    assert_eq!(a_stdout, vec![b'a', b'l', b'f', b'a']);
    assert_eq!(b_stdout, vec![b'b', b'r', b'a', b'v', b'o']);

}
