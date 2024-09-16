use assertables::*;
use regex::*;

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
fn assert_nearness() {
    assert_in_delta!(1, 1, 0);
    assert_in_epsilon!(1, 1, 0);
}

#[test]
fn assert_inners() {
    assert_starts_with!("alfa", "al");
    assert_not_starts_with!("alfa", "zz");
    assert_ends_with!("alfa", "fa");
    assert_not_ends_with!("alfa", "zz");
    assert_contains!("alfa", "lf");
    assert_not_contains!("alfa", "zz");
    assert_is_match!(Regex::new(r"lf").unwrap(), "alfa");
    assert_not_match!(Regex::new(r"zz").unwrap(), "alfa");
}

#[test]
fn assert_return_enum() {
    let a: Result<i8, i8> = Result::Ok(1);
    assert_result_ok!(a);

    let a: Result<i8, i8> = Result::Ok(1);
    let b: Result<i8, i8> = Result::Ok(1);
    assert_result_ok_eq!(a, b);

    let a: Result<i8, i8> = Result::Ok(1);
    let b: Result<i8, i8> = Result::Ok(2);
    assert_result_ok_ne!(a, b);

    let a: Result<i8, i8> = Result::Err(2);
    assert_result_err!(a);
}

#[test]
fn assert_option_enum() {
    let a: Option<i8> = Option::Some(1);
    assert_option_some!(a);

    let a: Option<i8> = Option::Some(1);
    let b: Option<i8> = Option::Some(1);
    assert_option_some_eq!(a, b);

    let a: Option<i8> = Option::Some(1);
    let b: Option<i8> = Option::Some(2);
    assert_option_some_ne!(a, b);

    let a: Option<i8> = Option::None;
    assert_option_none!(a);
}

#[test]
fn assert_poll_enum() {
    use std::task::Poll;
    use std::task::Poll::*;

    let a: Poll<i8> = Ready(1);
    assert_poll_ready!(a);

    let a: Poll<i8> = Ready(1);
    let b: Poll<i8> = Ready(1);
    assert_poll_ready_eq!(a, b);

    let a: Poll<i8> = Ready(1);
    let b: Poll<i8> = Ready(2);
    assert_poll_ready_ne!(a, b);

    let a: Poll<i8> = Pending;
    assert_poll_pending!(a);
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

// TODO
#[test]
fn assert_command() {
    // assert_command_stderr_contains!();
    // assert_command_stderr_eq!();
    // assert_command_stderr_eq_expr!();
    // assert_command_stderr_is_match!();
    // assert_command_stdout_contains!();
    // assert_command_stdout_eq!();
    // assert_command_stdout_eq_expr!();
    // assert_command_stdout_is_match!();
}

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

    // Comparisons with expressions
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

    // Comparisons with expressions
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

    // Comparisons with expressions
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

    // Comparisons with expressions
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

    // Comparisons with expressions
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

// TODO
#[test]
fn assert_program_args() {
    // assert_program_args_stderr_contains!();
    // assert_program_args_stderr_eq!();
    // assert_program_args_stderr_eq_expr!();
    // assert_program_args_stderr_ge!();
    // assert_program_args_stderr_ge_expr!();
    // assert_program_args_stderr_gt!();
    // assert_program_args_stderr_gt_expr!();
    // assert_program_args_stderr_is_match!();
    // assert_program_args_stderr_le!();
    // assert_program_args_stderr_le_expr!();
    // assert_program_args_stderr_lt!();
    // assert_program_args_stderr_lt_expr!();
    // assert_program_args_stderr_ne!();
    // assert_program_args_stderr_ne_expr!();
    // assert_program_args_stdout_contains!();
    // assert_program_args_stdout_eq!();
    // assert_program_args_stdout_eq_expr!();
    // assert_program_args_stdout_ge!();
    // assert_program_args_stdout_ge_expr!();
    // assert_program_args_stdout_gt!();
    // assert_program_args_stdout_gt_expr!();
    // assert_program_args_stdout_is_match!();
    // assert_program_args_stdout_le!();
    // assert_program_args_stdout_le_expr!();
    // assert_program_args_stdout_lt!();
    // assert_program_args_stdout_lt_expr!();
    // assert_program_args_stdout_ne!();
    // assert_program_args_stdout_ne_expr!();
}

#[test]
fn experimental() {
    assert_infix!(1 == 1);
    assert_infix!(1 != 2);
    assert_infix!(1 < 2);
    assert_infix!(1 <= 2);
    assert_infix!(2 > 1);
    assert_infix!(2 >= 1);
    assert_infix!(true && true);
    assert_infix!(false || true);
}

