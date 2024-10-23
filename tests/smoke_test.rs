use assertables::*;
use regex::Regex;
use std::path::PathBuf;
use once_cell::sync::Lazy;
use std::process::Command;

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
    assert_len_eq2!(s, "***********");
    assert_len_eq!(s, 11);
    assert_not_empty!(s);
    assert_all!(s.chars(), |c: char| c < 'x');
    assert_any!(s.chars(), |c: char| c.is_whitespace());
}

mod readme_returns {

    mod assert_ok_etc {
        use assertables::*;

        #[test]
        fn one() {
            let result: Result<i8, i8> = Ok(2);
            let inner = assert_ok_ne!(result, 1);
            assert_eq!(inner, &2);
        }

        #[test]
        fn two() {
            let a_result: Result<i8, i8> = Ok(1);
            let b_result: Result<i8, i8> = Ok(2);
            let (a_inner, b_inner) = assert_ok_ne2!(a_result, b_result);
            assert_eq!(a_inner, &1);
            assert_eq!(b_inner, &2);
        }

    }

    mod return_assert_fs_read_to_string_etc {
        use assertables::*;

        #[test]
        fn one() {
            let path = "alfa.txt";
            let string = assert_fs_read_to_string_ne!(path, "");
            assert_eq!(string, String::from("alfa\n"));
        }

        #[test]
        fn two() {
            let a_path = "alfa.txt";
            let b_path = "bravo.txt";
            let (a_string, b_string) = assert_fs_read_to_string_ne2!(a_path, b_path);
            assert_eq!(a_string, String::from("alfa\n"));
            assert_eq!(b_string, String::from("bravo\n"));
        }

    }

    mod assert_command_stdout_etc {
        use assertables::*;
        use std::process::Command;

        #[test]
        fn one() {
            let program = crate::BIN.join("printf-stdout");
            let mut command = Command::new(&program);
            command.args(["%s", "alfa"]);
            let stdout = assert_command_stdout_ne!(command, vec![b'a']);
            assert_eq!(stdout, vec![b'a', b'l', b'f', b'a']);
        }

        #[test]
        fn two() {
            let program = crate::BIN.join("printf-stdout");
            let mut a_command = Command::new(&program);
            a_command.args(["%s", "alfa"]);
            let mut b_command = Command::new(&&program);
            b_command.args(["%s", "bravo"]);
            let (a_stdout, b_stdout) = assert_command_stdout_ne2!(a_command, b_command);
            assert_eq!(a_stdout, vec![b'a', b'l', b'f', b'a']);
            assert_eq!(b_stdout, vec![b'b', b'r', b'a', b'v', b'o']);
        }
    }
    
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
    assert_len_eq2!("x", "x");
    assert_len_ne2!("x", "xx");
    assert_len_lt2!("x", "xx");
    assert_len_le2!("x", "xx");
    assert_len_gt2!("xx", "x");
    assert_len_ge2!("xx", "x");
    // Compare expr
    assert_len_eq!("x", 1);
    assert_len_ne!("x", 2);
    assert_len_lt!("x", 2);
    assert_len_le!("x", 2);
    assert_len_gt!("xx", 1);
    assert_len_ge!("xx", 1);
}

#[test]
fn assert_count() {
    // Compare other
    assert_count_eq2!("x".chars(), "x".chars());
    assert_count_ne2!("x".chars(), "xx".chars());
    assert_count_lt2!("x".chars(), "xx".chars());
    assert_count_le2!("x".chars(), "xx".chars());
    assert_count_gt2!("xx".chars(), "x".chars());
    assert_count_ge2!("xx".chars(), "x".chars());
    // Compare expr
    assert_count_eq!("x".chars(), 1);
    assert_count_ne!("x".chars(), 2);
    assert_count_lt!("x".chars(), 2);
    assert_count_le!("x".chars(), 2);
    assert_count_gt!("xx".chars(), 1);
    assert_count_ge!("xx".chars(), 1);
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
    assert_ok_eq2!(a, b);

    let a: Result<i8, i8> = Result::Ok(1);
    let b: Result<i8, i8> = Result::Ok(2);
    assert_ok_ne2!(a, b);
}

#[test]
fn assert_err() {
    let a: Result<i8, i8> = Result::Err(1);
    assert_err!(a);

    let a: Result<i8, i8> = Result::Err(1);
    let b: Result<i8, i8> = Result::Err(1);
    assert_err_eq2!(a, b);

    let a: Result<i8, i8> = Result::Err(1);
    let b: Result<i8, i8> = Result::Err(2);
    assert_err_ne2!(a, b);

}

//// Option Some/None

#[test]
fn assert_some() {
    let a: Option<i8> = Option::Some(1);
    assert_some!(a);

    let a: Option<i8> = Option::Some(1);
    let b: Option<i8> = Option::Some(1);
    assert_some_eq2!(a, b);

    let a: Option<i8> = Option::Some(1);
    let b: Option<i8> = Option::Some(2);
    assert_some_ne2!(a, b);
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
    assert_ready_eq2!(a, b);

    let a: Poll<i8> = Ready(1);
    let b: Poll<i8> = Ready(2);
    assert_ready_ne2!(a, b);
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
    assert_iter_eq2!([1], [1]);
    assert_iter_ne2!([1], [2]);
    assert_iter_lt2!([1, 2], [3, 4]);
    assert_iter_le2!([1, 2], [3, 4]);
    assert_iter_gt2!([3, 4], [1, 2]);
    assert_iter_ge2!([3, 4], [1, 2]);
}

#[test]
fn assert_bag() {
    assert_bag_eq2!([1], [1]);
    assert_bag_ne2!([1], [2]);
    assert_bag_subbag!([1], [1, 2]);
    assert_bag_superbag!([1, 2], [1]);
}

#[test]
fn assert_set() {
    assert_set_eq2!([1], [1]);
    assert_set_ne2!([1], [2]);
    assert_set_subset!([1], [1, 2]);
    assert_set_superset!([1, 2], [1]);
    assert_set_joint!([1], [1]);
    assert_set_disjoint!([1], [2]);
}

//// assert_command*

#[test]
fn assert_command_stdout() {
    use std::process::Command;

    let program = BIN.join("printf-stdout");
    let mut a = Command::new(&program);
    a.args(["%s", "alfa"]);

    //// one
    assert_command_stdout_eq!(a, vec![b'a', b'l', b'f', b'a']);
    assert_command_stdout_ne!(a, vec![b'x', b'x']);
    assert_command_stdout_lt!(a, vec![b'z', b'z']);
    assert_command_stdout_le!(a, vec![b'z', b'z']);
    assert_command_stdout_gt!(a, vec![b'a', b'a']);
    assert_command_stdout_ge!(a, vec![b'a', b'a']);

    //// two
    let mut b = Command::new(&program); b.args(["%s%s%s%s", "a", "l", "f", "a"]); assert_command_stdout_eq2!(a, b);
    let mut b = Command::new(&program); b.args(["%s%s%s%s", "x", "x"]); assert_command_stdout_ne2!(a, b);
    let mut b = Command::new(&program); b.args(["%s%s%s%s", "z", "z"]); assert_command_stdout_lt2!(a, b);
    let mut b = Command::new(&program); b.args(["%s%s%s%s", "z", "z"]); assert_command_stdout_le2!(a, b);
    let mut b = Command::new(&program); b.args(["%s%s%s%s", "a", "a"]); assert_command_stdout_gt2!(a, b);
    let mut b = Command::new(&program); b.args(["%s%s%s%s", "a", "a"]); assert_command_stdout_ge2!(a, b);

    //// matching
    assert_command_stdout_contains!(a, "lf");
    assert_command_stdout_string_contains!(a, "lf");
    assert_command_stdout_is_match!(a, Regex::new(r"lf").unwrap());
    assert_command_stdout_string_is_match!(a, Regex::new(r"lf").unwrap());

}

#[test]
fn assert_command_stderr() {

    let program = BIN.join("printf-stderr");
    let mut a = Command::new(&program);
    a.args(["%s", "alfa"]);

    //// one
    assert_command_stderr_eq!(a, vec![b'a', b'l', b'f', b'a']);
    assert_command_stderr_ne!(a, vec![b'x', b'x']);
    assert_command_stderr_lt!(a, vec![b'z', b'z']);
    assert_command_stderr_le!(a, vec![b'z', b'z']);
    assert_command_stderr_gt!(a, vec![b'a', b'a']);
    assert_command_stderr_ge!(a, vec![b'a', b'a']);

    //// two
    let mut b = Command::new(&program); b.args(["%s%s%s%s", "a", "l", "f", "a"]); assert_command_stderr_eq2!(a, b);
    let mut b = Command::new(&program); b.args(["%s%s%s%s", "x", "x"]); assert_command_stderr_ne2!(a, b);
    let mut b = Command::new(&program); b.args(["%s%s%s%s", "z", "z"]); assert_command_stderr_lt2!(a, b);
    let mut b = Command::new(&program); b.args(["%s%s%s%s", "z", "z"]); assert_command_stderr_le2!(a, b);
    let mut b = Command::new(&program); b.args(["%s%s%s%s", "a", "a"]); assert_command_stderr_gt2!(a, b);
    let mut b = Command::new(&program); b.args(["%s%s%s%s", "a", "a"]); assert_command_stderr_ge2!(a, b);

    //// matching
    assert_command_stderr_contains!(a, "lf");
    assert_command_stderr_string_contains!(a, "lf");
    assert_command_stderr_is_match!(a, Regex::new(r"lf").unwrap());
    assert_command_stderr_string_is_match!(a, Regex::new(r"lf").unwrap());

}

//// assert_program_args*

#[test]
fn assert_program_args_stdout() {

    let a_program = BIN.join("printf-stdout");
    let a_args = ["%s", "alfa"];

    //// one
    assert_program_args_stdout_eq!(&a_program, &a_args, vec![b'a', b'l', b'f', b'a']);
    assert_program_args_stdout_ne!(&a_program, &a_args, vec![b'x']);
    assert_program_args_stdout_lt!(&a_program, &a_args, vec![b'z', b'z']);
    assert_program_args_stdout_le!(&a_program, &a_args, vec![b'z', b'z']);
    assert_program_args_stdout_gt!(&a_program, &a_args, vec![b'a', b'a']);
    assert_program_args_stdout_ge!(&a_program, &a_args, vec![b'a', b'a']);

    let b_program = BIN.join("printf-stdout");

    //// two
    assert_program_args_stdout_eq2!(&a_program, &a_args, &b_program, ["%s%s%s%s", "a", "l", "f", "a"]);
    assert_program_args_stdout_ne2!(&a_program, &a_args, &b_program, ["%s%s%s%s%s", "x"]);
    assert_program_args_stdout_lt2!(&a_program, &a_args, &b_program, ["%s%s%s%s%s", "z", "z"]);
    assert_program_args_stdout_le2!(&a_program, &a_args, &b_program, ["%s%s%s%s%s", "z", "z"]);
    assert_program_args_stdout_gt2!(&a_program, &a_args, &b_program, ["%s%s%s%s%s", "a", "a"]);
    assert_program_args_stdout_ge2!(&a_program, &a_args, &b_program, ["%s%s%s%s%s", "a", "a"]);

    //// matching
    assert_program_args_stdout_contains!(&a_program, &a_args, "lf");
    assert_program_args_stdout_string_contains!(&a_program, &a_args, "lf");
    assert_program_args_stdout_is_match!(&a_program, &a_args, Regex::new(r"lf").unwrap());
    assert_program_args_stdout_string_is_match!(&a_program, &a_args, Regex::new(r"lf").unwrap());

}

#[test]
fn assert_program_args_stderr() {

    let a_program = BIN.join("printf-stderr");
    let a_args = ["%s", "alfa"];

    //// one
    assert_program_args_stderr_eq!(&a_program, &a_args, vec![b'a', b'l', b'f', b'a']);
    assert_program_args_stderr_ne!(&a_program, &a_args, vec![b'x']);
    assert_program_args_stderr_lt!(&a_program, &a_args, vec![b'z', b'z']);
    assert_program_args_stderr_le!(&a_program, &a_args, vec![b'z', b'z']);
    assert_program_args_stderr_gt!(&a_program, &a_args, vec![b'a', b'a']);
    assert_program_args_stderr_ge!(&a_program, &a_args, vec![b'a', b'a']);

    let b_program = BIN.join("printf-stderr");

    //// two
    assert_program_args_stderr_eq2!(&a_program, &a_args, &b_program, ["%s%s%s%s", "a", "l", "f", "a"]);
    assert_program_args_stderr_ne2!(&a_program, &a_args, &b_program, ["%s%s%s%s%s", "x"]);
    assert_program_args_stderr_lt2!(&a_program, &a_args, &b_program, ["%s%s%s%s%s", "z", "z"]);
    assert_program_args_stderr_le2!(&a_program, &a_args, &b_program, ["%s%s%s%s%s", "z", "z"]);
    assert_program_args_stderr_gt2!(&a_program, &a_args, &b_program, ["%s%s%s%s%s", "a", "a"]);
    assert_program_args_stderr_ge2!(&a_program, &a_args, &b_program, ["%s%s%s%s%s", "a", "a"]);

    //// matching
    assert_program_args_stderr_contains!(&a_program, &a_args, "lf");
    assert_program_args_stderr_string_contains!(&a_program, &a_args, "lf");
    assert_program_args_stderr_is_match!(&a_program, &a_args, Regex::new(r"lf").unwrap());
    assert_program_args_stderr_string_is_match!(&a_program, &a_args, Regex::new(r"lf").unwrap());

}

//// Functions

#[test]
fn assert_fn() {

    //// Functions
    fn f() -> i8 { 1 }
    fn g() -> i8 { 2 }

    //// one
    assert_fn_eq!(f, 1);
    assert_fn_ne!(f, 2);
    assert_fn_lt!(f, 2);
    assert_fn_le!(f, 2);
    assert_fn_gt!(g, 1);
    assert_fn_ge!(g, 1);

    //// two
    assert_fn_eq2!(f, f);
    assert_fn_ne2!(f, g);
    assert_fn_lt2!(f, g);
    assert_fn_le2!(f, g);
    assert_fn_gt2!(g, f);
    assert_fn_ge2!(g, f);

}

#[test]
fn assert_fn_ok() {

    //// Functions
    fn f() -> Result<i8, i8> { Ok(1) }
    fn g() -> Result<i8, i8> { Ok(2) }

    //// one
    assert_fn_ok_eq!(f, 1);
    assert_fn_ok_ne!(f, 2);
    assert_fn_ok_lt!(f, 2);
    assert_fn_ok_le!(f, 2);
    assert_fn_ok_gt!(g, 1);
    assert_fn_ok_ge!(g, 1);

    //// two
    assert_fn_ok_eq2!(f, f);
    assert_fn_ok_ne2!(f, g);
    assert_fn_ok_lt2!(f, g);
    assert_fn_ok_le2!(f, g);
    assert_fn_ok_gt2!(g, f);
    assert_fn_ok_ge2!(g, f);

}

#[test]
fn assert_fn_err() {

    //// Functions
    fn f() -> Result<i8, i8> { Err(1) }
    fn g() -> Result<i8, i8> { Err(2) }

    //// one
    assert_fn_err_ne!(f, 2);
    assert_fn_err_eq!(f, 1);
    assert_fn_err_lt!(f, 2);
    assert_fn_err_le!(f, 2);
    assert_fn_err_gt!(g, 1);
    assert_fn_err_ge!(g, 1);

    //// two
    assert_fn_err_eq2!(f, f);
    assert_fn_err_ne2!(f, g);
    assert_fn_err_lt2!(f, g);
    assert_fn_err_le2!(f, g);
    assert_fn_err_gt2!(g, f);
    assert_fn_err_ge2!(g, f);

}

#[test]
fn assert_fs_read_to_string() {

    //// one
    assert_fs_read_to_string_eq!("alfa.txt", "alfa\n");
    assert_fs_read_to_string_ne!("alfa.txt", "z");
    assert_fs_read_to_string_lt!("alfa.txt", "b");
    assert_fs_read_to_string_le!("alfa.txt", "b");
    assert_fs_read_to_string_ge!("bravo.txt", "a");
    assert_fs_read_to_string_gt!("bravo.txt", "a");

    //// two 
    assert_fs_read_to_string_eq2!("alfa.txt", "alfa.txt");
    assert_fs_read_to_string_ne2!("alfa.txt", "bravo.txt");
    assert_fs_read_to_string_lt2!("alfa.txt", "bravo.txt");
    assert_fs_read_to_string_le2!("alfa.txt", "bravo.txt");
    assert_fs_read_to_string_gt2!("bravo.txt", "alfa.txt");
    assert_fs_read_to_string_ge2!("bravo.txt", "alfa.txt");

    //// matching
    assert_fs_read_to_string_contains!("alfa.txt", "lf");
    assert_fs_read_to_string_is_match!("alfa.txt", Regex::new("lf").unwrap());

}

#[test]
fn assert_io_read_to_string() {
    use std::io::Read;

    //// readers
    let mut a: &[u8];
    let mut b: &[u8];

    //// one
    a = "alfa".as_bytes(); assert_io_read_to_string_eq!(a, "alfa");
    a = "alfa".as_bytes(); assert_io_read_to_string_ne!(a, "b");
    a = "alfa".as_bytes(); assert_io_read_to_string_lt!(a, "b");
    a = "alfa".as_bytes(); assert_io_read_to_string_le!(a, "b");
    a = "alfa".as_bytes(); assert_io_read_to_string_gt!(a, "a");
    a = "alfa".as_bytes(); assert_io_read_to_string_ge!(a, "a");

    //// two
    a = "alfa".as_bytes(); b = "alfa".as_bytes();  assert_io_read_to_string_eq2!(a, b);
    a = "alfa".as_bytes(); b = "bravo".as_bytes(); assert_io_read_to_string_ne2!(a, b);
    a = "alfa".as_bytes(); b = "bravo".as_bytes(); assert_io_read_to_string_lt2!(a, b);
    a = "alfa".as_bytes(); b = "bravo".as_bytes(); assert_io_read_to_string_le2!(a, b);
    a = "alfa".as_bytes(); b = "bravo".as_bytes(); assert_io_read_to_string_gt2!(b, a);
    a = "alfa".as_bytes(); b = "bravo".as_bytes(); assert_io_read_to_string_ge2!(b, a);

    //// matching
    a = "alfa".as_bytes(); assert_io_read_to_string_contains!(a, "lf");
    a = "alfa".as_bytes(); assert_io_read_to_string_is_match!(a, Regex::new("lf").unwrap());

}
