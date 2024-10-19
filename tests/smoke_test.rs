use assertables::*;
use regex::Regex;
use std::path::PathBuf;
use once_cell::sync::Lazy;

pub static BIN: Lazy<PathBuf> = Lazy::new(|| {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("bin")
});

//// README

#[test]
fn readme() {
    let s = "hello world";
    assert_matches!(s, "hello world");
    assert_is_match!(Regex::new(r"h.* w.*").unwrap(), s);
    assert_starts_with!(s, "hello");
    assert_ends_with!(s, "world");
    assert_contains!(s, "o");
    assert_len_eq!(s, "***********");
    assert_len_eq_expr!(s, 11);
    assert_not_empty!(s);
    assert_all!(s.chars(), |c: char| c < 'x');
    assert_any!(s.chars(), |c: char| c.is_whitespace());
}

//// Values

#[test]
fn assert_values() {
    assert_eq!(1, 1);
    assert_ne!(2, 1);
    assert_lt!(1, 2);
    assert_le!(1, 2);
    assert_gt!(2, 1);
    assert_ge!(2, 1);
}

#[test]
fn assert_infix() {
    assert_infix!(1 == 1);
    assert_infix!(1 != 2);
    assert_infix!(1 < 2);
    assert_infix!(1 <= 2);
    assert_infix!(2 > 1);
    assert_infix!(2 >= 1);
    assert_infix!(true & true);
    assert_infix!(true && true);
    assert_infix!(false | true);
    assert_infix!(false || true);
    assert_infix!(false ^ true);
}

#[test]
fn assert_nearness() {
    assert_in_delta!(1, 1, 0);
    assert_in_epsilon!(1, 1, 0);
}

//// Strings

#[test]
fn assert_starts_with() {
    assert_starts_with!("alfa", "al");
    assert_not_starts_with!("alfa", "zz");
}

#[test]
fn assert_ends_with() {
    assert_ends_with!("alfa", "fa");
    assert_not_ends_with!("alfa", "zz");
}

//// Lengths

#[test]
fn assert_len() {
    // Compare other
    assert_len_eq!("x", "x");
    assert_len_ne!("x", "xx");
    assert_len_lt!("x", "xx");
    assert_len_le!("x", "xx");
    assert_len_gt!("xx", "x");
    assert_len_ge!("xx", "x");
    // Compare expr
    assert_len_eq_expr!("x", 1);
    assert_len_ne_expr!("x", 2);
    assert_len_lt_expr!("x", 2);
    assert_len_le_expr!("x", 2);
    assert_len_gt_expr!("xx", 1);
    assert_len_ge_expr!("xx", 1);
}

#[test]
fn assert_count() {
    // Compare other
    assert_count_eq!("x".chars(), "x".chars());
    assert_count_ne!("x".chars(), "xx".chars());
    assert_count_lt!("x".chars(), "xx".chars());
    assert_count_le!("x".chars(), "xx".chars());
    assert_count_gt!("xx".chars(), "x".chars());
    assert_count_ge!("xx".chars(), "x".chars());
    // Compare expr
    assert_count_eq_expr!("x".chars(), 1);
    assert_count_ne_expr!("x".chars(), 2);
    assert_count_lt_expr!("x".chars(), 2);
    assert_count_le_expr!("x".chars(), 2);
    assert_count_gt_expr!("xx".chars(), 1);
    assert_count_ge_expr!("xx".chars(), 1);
}

#[test]
fn assert_is_empty() {
    assert_is_empty!("");
    assert_not_empty!("alfa");
}

//// Matching

#[test]
fn assert_contains() {
    assert_contains!("alfa", "lf");
    assert_not_contains!("alfa", "zz");
}

#[test]
fn assert_is_match() {
    assert_is_match!(Regex::new(r"lf").unwrap(), "alfa");
    assert_not_match!(Regex::new(r"zz").unwrap(), "alfa");
}

//// Return Ok/Err

#[test]
fn assert_ok() {
    let a: Result<i8, i8> = Result::Ok(1);
    assert_ok!(a);

    let a: Result<i8, i8> = Result::Ok(1);
    let b: Result<i8, i8> = Result::Ok(1);
    assert_ok_eq!(a, b);

    let a: Result<i8, i8> = Result::Ok(1);
    let b: Result<i8, i8> = Result::Ok(2);
    assert_ok_ne!(a, b);
}

#[test]
fn assert_err() {
    let a: Result<i8, i8> = Result::Err(1);
    assert_err!(a);

    let a: Result<i8, i8> = Result::Err(1);
    let b: Result<i8, i8> = Result::Err(1);
    assert_err_eq!(a, b);

    let a: Result<i8, i8> = Result::Err(1);
    let b: Result<i8, i8> = Result::Err(2);
    assert_err_ne!(a, b);

}

//// Option Some/None

#[test]
fn assert_some() {
    let a: Option<i8> = Option::Some(1);
    assert_some!(a);

    let a: Option<i8> = Option::Some(1);
    let b: Option<i8> = Option::Some(1);
    assert_some_eq!(a, b);

    let a: Option<i8> = Option::Some(1);
    let b: Option<i8> = Option::Some(2);
    assert_some_ne!(a, b);
}

#[test]
fn assert_none() {
    let a: Option<i8> = Option::None;
    assert_none!(a);
}

//// Poll Ready/Pending

use std::task::Poll;
use std::task::Poll::*;

#[test]
fn assert_ready() {
    let a: Poll<i8> = Ready(1);
    assert_ready!(a);

    let a: Poll<i8> = Ready(1);
    let b: Poll<i8> = Ready(1);
    assert_ready_eq!(a, b);

    let a: Poll<i8> = Ready(1);
    let b: Poll<i8> = Ready(2);
    assert_ready_ne!(a, b);
}

#[test]
fn assert_pending() {
    let a: Poll<i8> = Pending;
    assert_pending!(a);
}

//// Collections

#[test]
fn assert_iter() {
    assert_all!([1, 2, 3].into_iter(), |x: i8| x > 0);
    assert_any!([1, 2, 3].into_iter(), |x: i8| x > 0);
    assert_iter_eq!([1], [1]);
    assert_iter_ne!([1], [2]);
    assert_iter_lt!([1, 2], [3, 4]);
    assert_iter_le!([1, 2], [3, 4]);
    assert_iter_gt!([3, 4], [1, 2]);
    assert_iter_ge!([3, 4], [1, 2]);
}

#[test]
fn assert_bag() {
    assert_bag_eq!([1], [1]);
    assert_bag_ne!([1], [2]);
    assert_bag_subbag!([1], [1, 2]);
    assert_bag_superbag!([1, 2], [1]);
}

#[test]
fn assert_set() {
    assert_set_eq!([1], [1]);
    assert_set_ne!([1], [2]);
    assert_set_subset!([1], [1, 2]);
    assert_set_superset!([1, 2], [1]);
    assert_set_joint!([1], [1]);
    assert_set_disjoint!([1], [2]);
}

//// Command

#[test]
fn assert_command() {
    use std::process::Command;

    //// stdout
    let program = BIN.join("printf-stdout");
    let mut a = Command::new(&program);
    a.args(["%s", "alfa"]);

    //// stdout
    let mut b = Command::new(&program); b.args(["%s%s%s%s", "a", "l", "f", "a"]); assert_command_stdout_eq!(a, b);
    let mut b = Command::new(&program); b.args(["%s%s%s%s", "x", "x"]); assert_command_stdout_ne!(a, b);
    let mut b = Command::new(&program); b.args(["%s%s%s%s", "z", "z"]); assert_command_stdout_lt!(a, b);
    let mut b = Command::new(&program); b.args(["%s%s%s%s", "z", "z"]); assert_command_stdout_le!(a, b);
    let mut b = Command::new(&program); b.args(["%s%s%s%s", "a", "a"]); assert_command_stdout_gt!(a, b);
    let mut b = Command::new(&program); b.args(["%s%s%s%s", "a", "a"]); assert_command_stdout_ge!(a, b);

    //// stdout expr
    assert_command_stdout_eq_expr!(a, vec![b'a', b'l', b'f', b'a']);
    assert_command_stdout_ne_expr!(a, vec![b'x', b'x']);
    assert_command_stdout_lt_expr!(a, vec![b'z', b'z']);
    assert_command_stdout_le_expr!(a, vec![b'z', b'z']);
    assert_command_stdout_gt_expr!(a, vec![b'a', b'a']);
    assert_command_stdout_ge_expr!(a, vec![b'a', b'a']);

    //// stdout string
    assert_command_stdout_contains!(a, "lf");
    assert_command_stdout_string_contains!(a, "lf");
    assert_command_stdout_is_match!(a, Regex::new(r"lf").unwrap());
    assert_command_stdout_string_is_match!(a, Regex::new(r"lf").unwrap());

    //// stderr
    let program = BIN.join("printf-stderr");
    let mut a = Command::new(&program);
    a.args(["%s", "alfa"]);

    //// stderr
    let mut b = Command::new(&program); b.args(["%s%s%s%s", "a", "l", "f", "a"]); assert_command_stderr_eq!(a, b);
    let mut b = Command::new(&program); b.args(["%s%s%s%s", "x", "x"]); assert_command_stderr_ne!(a, b);
    let mut b = Command::new(&program); b.args(["%s%s%s%s", "z", "z"]); assert_command_stderr_lt!(a, b);
    let mut b = Command::new(&program); b.args(["%s%s%s%s", "z", "z"]); assert_command_stderr_le!(a, b);
    let mut b = Command::new(&program); b.args(["%s%s%s%s", "a", "a"]); assert_command_stderr_gt!(a, b);
    let mut b = Command::new(&program); b.args(["%s%s%s%s", "a", "a"]); assert_command_stderr_ge!(a, b);

    //// stderr expr
    assert_command_stderr_eq_expr!(a, vec![b'a', b'l', b'f', b'a']);
    assert_command_stderr_ne_expr!(a, vec![b'x', b'x']);
    assert_command_stderr_lt_expr!(a, vec![b'z', b'z']);
    assert_command_stderr_le_expr!(a, vec![b'z', b'z']);
    assert_command_stderr_gt_expr!(a, vec![b'a', b'a']);
    assert_command_stderr_ge_expr!(a, vec![b'a', b'a']);

    //// stderr string
    assert_command_stderr_contains!(a, "lf");
    assert_command_stderr_string_contains!(a, "lf");
    assert_command_stderr_is_match!(a, Regex::new(r"lf").unwrap());
    assert_command_stderr_string_is_match!(a, Regex::new(r"lf").unwrap());
}

#[test]
fn assert_program_args() {

    //// stdout
    let a_program = BIN.join("printf-stdout");
    let a_args = ["%s", "alfa"];

    //// stdout other
    // assert_program_args_stdout_eq!(&a_program, &a_args, &b_program, ["%s%s%s%s", "a", "l", "f", "a"]);
    // assert_program_args_stdout_ne!(&a_program, &a_args, &b_program, ["%s%s%s%s%s", "x"]);
    // assert_program_args_stdout_lt!(&a_program, &a_args, &b_program, ["%s%s%s%s%s", "z", "z"]);
    // assert_program_args_stdout_le!(&a_program, &a_args, &b_program, ["%s%s%s%s%s", "z", "z"]);
    // assert_program_args_stdout_gt!(&a_program, &a_args, &b_program, ["%s%s%s%s%s", "a", "a"]);
    // assert_program_args_stdout_ge!(&a_program, &a_args, &b_program, ["%s%s%s%s%s", "a", "a"]);

    //// stdout expr
    assert_program_args_stdout_eq_expr!(&a_program, &a_args, vec![b'a', b'l', b'f', b'a']);
    assert_program_args_stdout_ne_expr!(&a_program, &a_args, vec![b'x']);
    assert_program_args_stdout_lt_expr!(&a_program, &a_args, vec![b'z', b'z']);
    assert_program_args_stdout_le_expr!(&a_program, &a_args, vec![b'z', b'z']);
    assert_program_args_stdout_gt_expr!(&a_program, &a_args, vec![b'a', b'a']);
    assert_program_args_stdout_ge_expr!(&a_program, &a_args, vec![b'a', b'a']);

    //// stdout string
    assert_program_args_stdout_contains!(&a_program, &a_args, "lf");
    assert_program_args_stdout_string_contains!(&a_program, &a_args, "lf");
    assert_program_args_stdout_is_match!(&a_program, &a_args, Regex::new(r"lf").unwrap());
    assert_program_args_stdout_string_is_match!(&a_program, &a_args, Regex::new(r"lf").unwrap());

    //// stderr
    let a_program = BIN.join("printf-stderr");
    let a_args = ["%s", "alfa"];

    //// stderr other
    // assert_program_args_stderr_eq!(&a_program, &a_args, &b_program, ["%s%s%s%s", "a", "l", "f", "a"]);
    // assert_program_args_stderr_ne!(&a_program, &a_args, &b_program, ["%s%s%s%s%s", "x"]);
    // assert_program_args_stderr_lt!(&a_program, &a_args, &b_program, ["%s%s%s%s%s", "z", "z"]);
    // assert_program_args_stderr_le!(&a_program, &a_args, &b_program, ["%s%s%s%s%s", "z", "z"]);
    // assert_program_args_stderr_gt!(&a_program, &a_args, &b_program, ["%s%s%s%s%s", "a", "a"]);
    // assert_program_args_stderr_ge!(&a_program, &a_args, &b_program, ["%s%s%s%s%s", "a", "a"]);

    //// stderr expr
    assert_program_args_stderr_eq_expr!(&a_program, &a_args, vec![b'a', b'l', b'f', b'a']);
    assert_program_args_stderr_ne_expr!(&a_program, &a_args, vec![b'x']);
    assert_program_args_stderr_lt_expr!(&a_program, &a_args, vec![b'z', b'z']);
    assert_program_args_stderr_le_expr!(&a_program, &a_args, vec![b'z', b'z']);
    assert_program_args_stderr_gt_expr!(&a_program, &a_args, vec![b'a', b'a']);
    assert_program_args_stderr_ge_expr!(&a_program, &a_args, vec![b'a', b'a']);

    //// stderr string
    assert_program_args_stderr_contains!(&a_program, &a_args, "lf");
    assert_program_args_stderr_string_contains!(&a_program, &a_args, "lf");
    assert_program_args_stderr_is_match!(&a_program, &a_args, Regex::new(r"lf").unwrap());
    assert_program_args_stderr_string_is_match!(&a_program, &a_args, Regex::new(r"lf").unwrap());

}

//// Functions

#[test]
fn assert_fn() {

    // Functions
    fn f() -> i8 { 1 }
    fn g() -> i8 { 2 }

    // Comparisons
    assert_fn_eq!(f, f);
    assert_fn_ne!(f, g);
    assert_fn_lt!(f, g);
    assert_fn_le!(f, g);
    assert_fn_gt!(g, f);
    assert_fn_ge!(g, f);

    // Compare with expression
    assert_fn_eq_expr!(f, 1);
    assert_fn_ne_expr!(f, 2);
    assert_fn_lt_expr!(f, 2);
    assert_fn_le_expr!(f, 2);
    assert_fn_gt_expr!(g, 1);
    assert_fn_ge_expr!(g, 1);

}

#[test]
fn assert_fn_ok() {

    // Functions
    fn f() -> Result<i8, i8> { Ok(1) }
    fn g() -> Result<i8, i8> { Ok(2) }

    // Comparisons
    assert_fn_ok_eq!(f, f);
    assert_fn_ok_ne!(f, g);
    assert_fn_ok_lt!(f, g);
    assert_fn_ok_le!(f, g);
    assert_fn_ok_gt!(g, f);
    assert_fn_ok_ge!(g, f);

    // Compare with expression
    assert_fn_ok_eq_expr!(f, 1);
    assert_fn_ok_ne_expr!(f, 2);
    assert_fn_ok_lt_expr!(f, 2);
    assert_fn_ok_le_expr!(f, 2);
    assert_fn_ok_gt_expr!(g, 1);
    assert_fn_ok_ge_expr!(g, 1);

}

#[test]
fn assert_fn_err() {

    // Functions
    fn f() -> Result<i8, i8> { Err(1) }
    fn g() -> Result<i8, i8> { Err(2) }

    // Comparisons
    assert_fn_err_eq!(f, f);
    assert_fn_err_ne!(f, g);
    assert_fn_err_lt!(f, g);
    assert_fn_err_le!(f, g);
    assert_fn_err_gt!(g, f);
    assert_fn_err_ge!(g, f);

    // Compare with expression
    assert_fn_err_ne_expr!(f, 2);
    assert_fn_err_eq_expr!(f, 1);
    assert_fn_err_lt_expr!(f, 2);
    assert_fn_err_le_expr!(f, 2);
    assert_fn_err_gt_expr!(g, 1);
    assert_fn_err_ge_expr!(g, 1);

}

#[test]
fn assert_fs_read_to_string() {

    // Comparisons
    assert_fs_read_to_string_eq!("alfa.txt", "alfa.txt");
    assert_fs_read_to_string_ne!("alfa.txt", "bravo.txt");
    assert_fs_read_to_string_lt!("alfa.txt", "bravo.txt");
    assert_fs_read_to_string_le!("alfa.txt", "bravo.txt");
    assert_fs_read_to_string_gt!("bravo.txt", "alfa.txt");
    assert_fs_read_to_string_ge!("bravo.txt", "alfa.txt");

    // Compare with expression
    assert_fs_read_to_string_eq_expr!("alfa.txt", "alfa\n");
    assert_fs_read_to_string_ne_expr!("alfa.txt", "z");
    assert_fs_read_to_string_lt_expr!("alfa.txt", "b");
    assert_fs_read_to_string_le_expr!("alfa.txt", "b");
    assert_fs_read_to_string_ge_expr!("bravo.txt", "a");
    assert_fs_read_to_string_gt_expr!("bravo.txt", "a");

    // Specializations
    assert_fs_read_to_string_contains!("alfa.txt", "lf");
    assert_fs_read_to_string_matches!("alfa.txt", Regex::new("lf").unwrap());

}

#[test]
fn assert_io_read_to_string() {
    use std::io::Read;

    // Readers
    let mut a: &[u8];
    let mut b: &[u8];

    // Comparisons
    a = "alfa".as_bytes(); b = "alfa".as_bytes();  assert_io_read_to_string_eq!(a, b);
    a = "alfa".as_bytes(); b = "bravo".as_bytes(); assert_io_read_to_string_ne!(a, b);
    a = "alfa".as_bytes(); b = "bravo".as_bytes(); assert_io_read_to_string_lt!(a, b);
    a = "alfa".as_bytes(); b = "bravo".as_bytes(); assert_io_read_to_string_le!(a, b);
    a = "alfa".as_bytes(); b = "bravo".as_bytes(); assert_io_read_to_string_gt!(b, a);
    a = "alfa".as_bytes(); b = "bravo".as_bytes(); assert_io_read_to_string_ge!(b, a);

    // Compare with expression
    a = "alfa".as_bytes(); assert_io_read_to_string_eq_expr!(a, "alfa");
    a = "alfa".as_bytes(); assert_io_read_to_string_ne_expr!(a, "b");
    a = "alfa".as_bytes(); assert_io_read_to_string_lt_expr!(a, "b");
    a = "alfa".as_bytes(); assert_io_read_to_string_le_expr!(a, "b");
    a = "alfa".as_bytes(); assert_io_read_to_string_gt_expr!(a, "a");
    a = "alfa".as_bytes(); assert_io_read_to_string_ge_expr!(a, "a");

    // Specializations
    a = "alfa".as_bytes(); assert_io_read_to_string_contains!(a, "lf");
    a = "alfa".as_bytes(); assert_io_read_to_string_matches!(a, Regex::new("lf").unwrap());

}
