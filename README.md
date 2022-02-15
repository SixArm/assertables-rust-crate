# Assertables Rust crate of macros for "assert" and "assertable"

This `assertables` Rust crate provides macros for `assert…!` and
`assertable…!`, which are useful for testing and also for runtime
reliability checking. By SixArm.com.

Crate:
[https://crates.io/crates/assertables](https://crates.io/crates/assertables)

Docs:
[https://docs.rs/assertables/](https://docs.rs/assertables/)

Repo:
[https://github.com/sixarm/assertables-rust-crate/](https://github.com/sixarm/assertables-rust-crate/)


## assert & assertable

These macros have two styles:

  * `assert` macros return `()` or `panic!(…)`.

  * `assertable` macros return `Ok(())` or `Err(…)`.

Examples of "assert less than" and "assertable less than":

```rust
assert_lt!(1, 2); // assert 1 is less than 2
//-> ()

assert_lt!(2, 1);
//-> panic!
// assertion failed: `assert_lt!(left, right)`
//   left: `2`,
//  right: `1`

let x = assertable_lt!(1, 2);
//-> Ok(())

let x = assertable_lt!(2, 1);
//-> Err("…")
// assertable failed: `assertable_lt!(left, right)`
//   left: `2`,
//  right: `1`
```

These two styles are useful because:

* `assert` macros favor compile-time tests and diagnostics.

* `assertable` macros favor run-time tracing and recoveries.

The macros use abbreviations: `eq` (equals), `ne` (not equals),
`lt` (less than), `le` (less than or equal to), `gt` (greater than),
`ge` (greater than or equals).

The macros have a second form where a custom error message can be provided.


## assert_xx for values

Compare values.

* `assert_eq!(a, b)` ~ a == b

* `assert_ne!(a, b)` ~ a != b

* `assert_lt!(a, b)` ~ a < b

* `assert_le!(a, b)` ~ a <= b

* `assert_gt!(a, b)` ~ a > b

* `assert_ge!(a, b)` ~ a >= b

Examples:

```rust
assert_lt!(1, 2);
//-> ()

assert_lt!(2, 1);
//-> panic!
// assertion failed: `assert_lt!(left, right)`
//   left: `2`
//  right: `1`
```


## assert_f_xx for function returns
 
* `assert_f_eq!(f, a, b)` ~ f(a) == f(b)
 
* `assert_f_ne!(f, a, b)` ~ f(a) != f(b)
 
* `assert_f_lt!(f, a, b)` ~ f(a) < f(b)
 
* `assert_f_le!(f, a, b)` ~ f(a) <= f(b)
 
* `assert_f_gt!(f, a, b)` ~ f(a) > f(b)
 
* `assert_f_ge!(f, a, b)` ~ f(a) >= f(b)
 
Examples:

```rust
assert_f_lt!(i32::abs, 1, -2);
//-> ()

assert_f_lt!(i32::abs, -2, 1);
//-> panic!
// assertion failed: `assert_f_eq!(function, left, right)`
//      function: `\"i32::abs\"`,
//    left input: `-2`,
//   right input: `1`,
//   left output: `2`,
//  right output: `1`
```


## assert_f_ok_xx for function Result Ok values

* `assert_f_ok_eq!(f, a, b)` ~ f(a).unwrap() == f(b).unwrap()

* `assert_f_ok_ne!(f, a, b)` ~ f(a).unwrap() != f(b).unwrap()

* `assert_f_ok_lt!(f, a, b)` ~ f(a).unwrap() < f(b).unwrap()

* `assert_f_ok_le!(f, a, b)` ~ f(a).unwrap() <= f(b).unwrap()

* `assert_f_ok_gt!(f, a, b)` ~ f(a).unwrap() > f(b).unwrap()

* `assert_f_ok_ge!(f, a, b)` ~ f(a).unwrap() >= f(b).unwrap()

```rust
fn example_digit_to_string(i: isize) -> Result<String, String> {
    match i {
        0..=9 => Ok(format!("{}", i)),
        _ => Err(format!("{:?} is out of range", i)),
    }
}

assert_f_ok_lt!(example_digit_to_string, 1, 2);
//-> ()

assert_f_ok_lt!(example_digit_to_string, 2, 1);
//-> panic!
// assertion failed: `assert_f_eq!(function, left, right)`
//      function: `\"example_digit_to_string\"`,
//    left input: `2`,
//   right input: `1`,
//   left output: `\"2\"`,
//  right output: `\"1\"`
```


## assert_f_err_xx for function Err() results

* `assert_f_err_eq!(f, a, b)` ~ f(a).unwrap_err() == f(b).unwrap_err()

* `assert_f_err_ne!(f, a, b)` ~ f(a).unwrap_err() != f(b).unwrap_err()

* `assert_f_err_lt!(f, a, b)` ~ f(a).unwrap_err() < f(b).unwrap_err()

* `assert_f_err_le!(f, a, b)` ~ f(a).unwrap_err() <= f(b).unwrap_err()

* `assert_f_err_gt!(f, a, b)` ~ f(a).unwrap_err() > f(b).unwrap_err()

* `assert_f_err_ge!(f, a, b)`~ f(a).unwrap_err() >= f(b).unwrap_err()

Examples:

```rust
fn example_digit_to_string(i: isize) -> Result<String, String> {
    match i {
        0..=9 => Ok(format!("{}", i)),
        _ => Err(format!("{:?} is out of range", i)),
    }
}

assert_f_err_lt!(example_digit_to_string, 10, 20);
//-> ()

assert_f_err_lt!(example_digit_to_string, 20, 10);
//-> panic!
// assertion failed: `assert_f_err_eq!(example_digit_to_string, left, right)`
//      function: `example_digit_to_string`,
//    left input: `20`,
//   right input: `10``,
//   left is err: `true`,
//  right is err: `true`,
//   left output: `\"20 is out of range\"`,
//  right output: `\"10 is out of range\"`
```


## assert_f_err_string_xx for function Err() strings

* `assert_f_err_string_eq!(f, a, b)` ~ f(a).unwrap_err().to_string() == f(b).unwrap_err().to_string()

* `assert_f_err_string_ne!(f, a, b)` ~ f(a).unwrap_err().to_string() != f(b).unwrap_err().to_string()

* `assert_f_err_string_lt!(f, a, b)` ~ f(a).unwrap_err().to_string() < f(b).unwrap_err().to_string()

* `assert_f_err_string_le!(f, a, b)` ~ f(a).unwrap_err().to_string() <= f(b).unwrap_err().to_string()

* `assert_f_err_string_gt!(f, a, b)` ~ f(a).unwrap_err().to_string() > f(b).unwrap_err().to_string()

* `assert_f_err_string_ge!(f, a, b)`~ f(a).unwrap_err().to_string() >= f(b).unwrap_err().to_string()

Examples:

```rust
fn example_digit_to_string(i: isize) -> Result<String, String> {
    match i {
        0..=9 => Ok(format!("{}", i)),
        _ => Err(format!("{:?} is out of range", i)),
    }
}

assert_f_err_string_lt!(example_digit_to_string, 10, 20);
//-> ()

assert_f_err_string_lt!(example_digit_to_string, 20, 10);
//-> panic!
// assertion failed: `assert_f_err_string_eq!(example_digit_to_string, left, right)`
//      function: `\"example_digit_to_string\"`,
//    left input: `20`,
//   right input: `10``,
//   left is err: `true`,
//  right is err: `true`,
//   left output: `\"20 is out of range\"`,
//  right output: `\"10 is out of range\"`
```

Two functions that we use often:

  * `assert_f_ok_eq!(i32::from_str, str1, str2); // compare parsing of numbers`

  * `assert_f_ok_eq!(std::fs::read_to_string, file1, file2); // compare file text`


### assert_set_xx for set comparisons

These macros help with comparison of set parameters, such as two arrays or
two vectors. where the item order does not matter, and the item count does
not matter. The macros convert inputs into HashSet iterators.

* `assert_set_eq!(a, b)` ~ set a == set b

* `assert_set_ne!(a, b)` ~ set a != set b

* `assert_set_subset!(a, b)` ~ set a ⊆ set b

* `assert_set_superset!(a, b)` ~ set a ⊇ set b

* `assert_set_joint!(a, b)` ~ set a is joint with set b

* `assert_set_disjoint!(a, b)` ~ set a is disjoint with set b

Examples:

```rust
assert_set_eq!([1, 2], [2, 1]);
//-> ()

assert_set_eq!([1, 2], [3, 4]);
//-> panic
// assertion failed: `assert_set_eq!(left, right)`
//   left: `[1, 2]`,
//  right: `[3, 4]`
//-> panic!("assertion failed: `assert_set_eq!(left, right)`\n  left: `[1, 2]`,\n right: `[3, 4]`");
```


### assert_bag_xx for bag comparisons

These macros help with comparison of bag parameters, such as comparison of
two arrays or two vectors, where the item order does not matter, and the
item count does matter. The macros convert inputs into HashMap iterators.

* `assert_bag_eq(a, b)` ~ bag a == bag b

* `assert_bag_ne(a, b)` ~ bag a != bag b

* `assert_bag_subbag(a, b)` ~ bag a ⊆ bag b

* `assert_bag_superbag(a, b)` ~ bag a ⊇ bag b

Examples:

```rust
assert_bag_eq!([1, 1], [1, 1]);
//-> ()

assert_bag_eq!([1, 1], [1, 1, 1]);
//-> panic!
// assertion failed: `assert_bag_eq!(left, right)`
//   left: `[1, 1]`,
//  right: `[1, 1, 1]`
```


### assert_io_xx for input/output comparisons

These macros help with input/output checking,
such as with comparison of disk files, IO streams, etc.

* `assert_io!(a)` ~ a is true

* `assert_io_eq!(a, b)` ~ a == b

* `assert_io_ne!(a, b)` ~ a != b

* `assert_io_lt!(a, b)` ~ a < b

* `assert_io_le!(a, b)` ~ a <= b

* `assert_io_gt!(a, b)` ~ a > b

* `assert_io_ge!(a, b)` ~ a >= b

Examples:

```rust
assert_io_lt!(1, 2);
//-> ()

assert_io_lt!(2, 1);
//-> panic!
// assertion failed: `assert_io_lt!(left, right)`
//   left: `2`,
//  right: `1`
```


## assert_read_to_string_xx for std::io::Read comparisons

These macros help with readers, such as file handles, byte
arrays, input streams, and the trait std::io::Read.

* `assert_read_to_string_eq!(a, b)` ~ a.read_to_string() == b.read_to_string()

* `assert_read_to_string_ne!(a, b)` ~ a.read_to_string() != b.read_to_string()

* `assert_read_to_string_lt!(a, b)` ~ a.read_to_string() < b.read_to_string()

* `assert_read_to_string_le!(a, b)` ~ a.read_to_string() <= b.read_to_string()

* `assert_read_to_string_gt!(a, b)` ~ a.read_to_string() > b.read_to_string()

* `assert_read_to_string_ge!(a, b)` ~ a.read_to_string() >= b.read_to_string()

* `assert_read_to_string_eq_string!(readable, string)` ~ readable.read_to_string() == str

* `assert_read_to_string_contains!(readable, pattern)` ~ readable.read_to_string().contains(pattern)

* `assert_read_to_string_is_match!(readable, matchable)` ~ matchable.is_match(readable.read_to_string())

Examples:

```rust
use std::io::Read;

let mut a = "a".as_bytes();
let mut b = "b".as_bytes();
assert_read_to_string_lt!(a, b);
//-> ()

let mut a = "a".as_bytes();
let mut b = "b".as_bytes();
assert_read_to_string_lt!(b, a);
//-> panic!
// assertion failed: `assert_read_to_string_lt!(left, right)`
//   left: `\"b\"`,
//  right: `\"a\"`
```


## assert_command_stdout_xx & assert_command_stderr_xx

stdout:

* `assert_command_stdout_eq!(left_command, right_command)` ~ String::from_utf8(left_command.output().unwrap().stdout).unwrap() == String::from_utf8(right_command.output().unwrap().stdout).unwrap()

* `assert_command_stdout_eq_string!(command, string)` ~ String::from_utf8(command.output().unwrap().stdout).unwrap() == str

* `assert_command_stdout_contains!(command, pattern)` ~ String::from_utf8(command.output().unwrap().stdout).unwrap().contains(pattern)

* `assert_command_stdout_is_match!(command, regex)` ~ matchable.is_match(String::from_utf8(command.output().unwrap().stdout).unwrap())

stderr:

* `assert_command_stderr_eq!(left_command, right_command)` ~ String::from_utf8(left_command.output().unwrap().stderr).unwrap() == String::from_utf8(right_command.output().unwrap().stdout).unwrap()

* `assert_command_stderr_eq_string!(command, string)` ~ String::from_utf8(command.output().unwrap().stderr).unwrap() == str

* `assert_command_stderr_contains!(command, pattern)` ~ String::from_utf8(command.output().unwrap().stderr).unwrap().contains(pattern)

* `assert_command_stderr_is_match!(command, regex)` ~ matchable.is_match(String::from_utf8(command.output().unwrap().stderr).unwrap())

Examples:

```rust
use std::process::Command;

let mut a = Command::new("printf");
a.args(["%s", "hello"]);
let mut b = Command::new("printf");
b.args(["%s%s%s%s%s", "h", "e", "l", "l", "o"]);
assert_command_stdout_eq!(a, b);
//-> ()

let mut a = Command::new("printf");
a.args(["%s", "hello"]);
let mut b = Command::new("printf");
b.args(["%s%s%s%s%s", "w", "o", "r", "l", "d"]);
assert_command_stdout_eq!(a, b);
//-> panic!("…")
// assertion failed: `assert_command_stdout_eq!(left_command, right_command)`
//   left command program: `\"printf\"`,
//  right command program: `\"printf\"`,
//   left stdout: `\"hello\"`,
//  right stdout: `\"world\"`
```
