# Rust assert macros: how to use assertables to improve your code

Work in progress...


### Macros for values

* `assert_eq!`

* `assert_ne!`

* `assert_gt!`

* `assert_ge!`

* `assert_lt!`

* `assert_le!`


### Macros for strings

* `assert_starts_with!` and `assert_not_starts_with!`

* `assert_ends_with!` and `assert_not_ends_with!`

* `assert_contains!` and `assert_not_contains!`

* `assert_is_match!` and `assert_not_match!`


### Macros for functions

A **function** is a typical Rust function.

Compare two functions with inputs:

```rust
let output1 = function1(input1);
let output2 = function2(input2);
assert_eq!(output1, output2);
```

Rust `assertables` provides the macro `assert_fn_eq!` that does the same kind of processing, by automatically calling functions with inputs, then comparing the outputs:

```rust
assert_fn_eq!(function1, input1, function2, input2);
```

The `assertables` message looks like:

```text
assertion failed: `assert_fn_eq!(left_function, left_input, right_function, right_input)`,
  left_function label: `function1`,
     left_input label: `input1`,
     left_input debug: `…`,
 right_function label: `function2`,
     right_expr label: `input2`,
     right_expr debug: `…`,
                 left: `…`,
                right: `…`
```

Rust `assertables` provides these macros for functions:

* `assert_fn_eq!`

* `assert_fn_eq_other!`

* `assert_fn_ge!`

* `assert_fn_ge_other!`

* `assert_fn_gt!`

* `assert_fn_gt_other!`

* `assert_fn_le!`

* `assert_fn_le_other!`

* `assert_fn_lt!`

* `assert_fn_lt_other!`

* `assert_fn_ne!`

* `assert_fn_ne_other!`


### Macros for functions that return a Result

A **Result** is a Rust standard that can be either `Ok`, `Err`.

TODO

let a = 1;
let b = String::from("1");
assert_fn_ok_eq!(example_digit_to_string, a, b);
//-> ()

let a = 1;
let b = String::from("2");
// Panic with error message
let result = panic::catch_unwind(|| {
assert_fn_ok_eq!(example_digit_to_string, a, b);
//-> panic!
});
assert!(result.is_err());

```text
assertion failed: `assert_fn_ok_eq!(left_function, left_input, right_function, right_input)`
  left_function label: `example_digit_to_string`,
     left_input label: `a`,
     left_input debug: `1`,
 right_function label: `example_digit_to_string`,
    right_input label: `a`,
    right_input debug: `1`,
     right_expr label: `b`,
     right_expr debug: `"2"`,
                 left: `"1"`,
                right: `"2"`
```


Rust `assertables` provides these macros for functions that return a Result of Ok or Err:

* `assert_fn_ok_eq!`

* `assert_fn_ok_eq_other!`

* `assert_fn_ok_ge!`

* `assert_fn_ok_ge_other!`

* `assert_fn_ok_gt!`

* `assert_fn_ok_gt_other!`

* `assert_fn_ok_le!`

* `assert_fn_ok_le_other!`

* `assert_fn_ok_lt!`

* `assert_fn_ok_lt_other!`

* `assert_fn_ok_ne!`

* `assert_fn_ok_ne_other!`

* `assert_fn_err_eq!`

* `assert_fn_err_eq_other!`

* `assert_fn_err_ge!`

* `assert_fn_err_ge_other!`

* `assert_fn_err_gt!`

* `assert_fn_err_gt_other!`

* `assert_fn_err_le!`

* `assert_fn_err_le_other!`

* `assert_fn_err_lt!`

* `assert_fn_err_lt_other!`

* `assert_fn_err_ne!`

* `assert_fn_err_ne_other!`


### Macros for sets

A **set** means a collection of elements, without any ordering, and without duplicate elements.

A set is sometimes written by using mathematical notation, which looks like this:

```text
set = {1, 2, 3}
```

These sets are equal, because ordering does not matter, and the left set contains the same elements as the right set:

```
{1, 2, 3} = {3, 2, 1}
```

These sets are not equal, because the sets do not contain all of the same elements, i.e. the left set contains the element "3" but not "4", and the right set contains the element "4" but not "3":

```text
{1, 2, 3} = {1, 2, 4}
```

This example is not a set, because sets cannot have duplicate elements, i.e. the element "3" occurs twice:

```text
{1, 2, 3, 3}
```

Rust can create a list of elements by using the standard array syntax:

```
let array = [1, 2, 3];
```

Rust arrays care about their order, so cannot be directly compared as sets:

```rust
let array1 = [1, 2, 3];
let array2 = [3, 2, 1];
assert_eq!(array1, array2); //=> panic
```

Convert an array into a set, by using `std::collections::BTreeSet`:

```rust
let set1: std::collections::BTreeSet<_> = array1.into_iter().collect();
```

Convert two arrays into sets, then use `assert_eq!` to compare them as sets:

```rust
let array1 = [1, 2];
let array2 = [3, 4];
let set1: std::collections::BTreeSet<_> = array1.into_iter().collect();
let set2: std::collections::BTreeSet<_> = array2.into_iter().collect();
assert_eq!(set1, set2);
```

Rust `assertables` provides the macro `assert_set_eq!` that does the same kind of processing, by automatically converting inputs into sets, then comparing them as sets:

```rust
assert_set_eq!(array1, array2);
```

The `assertables` message looks like:

```text
assertion failed: `assert_set_eq!(left_set, right_set)`
  left_set label: `&array1`,
  left_set debug: `[1, 2]`,
 right_set label: `&array2`,
 right_set debug: `[3, 4]`,
            left: `{1, 2}`,
           right: `{3, 4}`
```

Rust `assertables` provides these macros for sets:

* `assert_set_eq`

* `assert_set_ne`

* `assert_set_subset`

* `assert_set_superset`

* `assert_set_joint`

* `assert_set_disjoint`


### Macros for bags

A **bag** means a collection of elements, without any ordering, and allowing duplicate elements.

A bag is sometimes written by using mathematical notation, which looks like this:

```text
bag = {1, 1, 1, 2, 3}
```

These bags are equal, because ordering does not matter, and the bags contain all the same elements in the same numer:

```text
{1, 1, 1, 2, 3} = {1, 3, 1, 2, 1}
```

These bags are not equal, because the bags contain an element in a different number, i.e. the left bag has the element "3" once, and right bag has element "3" twice:

```text
{1, 2, 3} = {1, 2, 3, 3}
```

Rust can create a list of elements by using the standard array syntax:

```rust
let array = [1, 1, 1, 2, 3];
```

Rust arrays care about their order, so cannot be directly compared as bags:

```rust
let array1 = [1, 1, 1, 2, 3];
let array2 = [1, 3, 1, 2, 1];
assert_eq!(array1, array2); //=> panic
```

Convert an array into a bag, by using `std::collections::BTreeMap`, and tracking each element's count:

```rust
let mut bag1: ::std::collections::BTreeMap<_, usize> = ::std::collections::BTreeMap::new();
for x in array1.into_iter() {
    let n = bag1.entry(x).or_insert(0);
    *n += 1;
}
```

Convert two arrays into bags, then use `assert_eq!` to compare them as bags:

```rust
let mut bag1: ::std::collections::BTreeMap<_, usize> = ::std::collections::BTreeMap::new();
for x in array1.into_iter() {
    let n = bag1.entry(x).or_insert(0);
    *n += 1;
}
let mut bag1: ::std::collections::BTreeMap<_, usize> = ::std::collections::BTreeMap::new();
for x in array2.into_iter() {
    let n = bag2.entry(x).or_insert(0);
    *n += 1;
}
assert_eq!(bag1, bag2);
```

Rust `assertables` provides the macro `assert_bag_eq!` that does the same kind of processing, by automatically converting inputs into sets, then comparing them as bags:

```rust
assert_bag_eq!(array1, array2);
```

The `aasertables` message looks like:

```text
assertion failed: `assert_bag_eq!(left_bag, right_bag)`
  left_bag label: `&array1`,
  left_bag debug: `[1, 1]`,
 right_bag label: `&array2`,
 right_bag debug: `[1, 1, 1, 2, 2, 2, 2]`
            left: `{1: 2}`,
           right: `{1: 3, 2: 4}`
```

Rust `assertables` provides these macros for bags:

* `assert_bag_eq`

* `assert_bag_ne`

* `assert_bag_subbag`

* `assert_bag_superbag`

* `assert_bag_joint`

* `assert_bag_disjoint`


### Macros for readers

Rust has a concept of a "reader", such as using `std::io::Read` and its function `read_to_string()`. The concept can work with a variety of unpinnings, such as files, bytes, strings, and more.

Example reader of bytes:

```rust
use std::io::Read;
let mut reader = "hello world".as_bytes();
```

The Rust function `read_to_string()` needs an input string which will receive the text output. The function returns a result that is the number of bytes that the function read:

```rust
let mut string = String::new();
let result = reader.read_to_string(&mut string);
let number_of_bytes_read = result.unwrap();
```

Rust can compare a reader's string output to another reader's string output:

```rust
let mut string1 = String::new();
let mut string2 = String::new();
let result1 = reader1.read_to_string(&mut string1);
let result2 = reader2.read_to_string(&mut string2);
assert_eq!(string1, string2);
```

Rust `assertables` provides the macro `assert_io_read_to_string_eq!` that does the same kind of processing, by automatically calling `read_to_string()`, then comparing the outputs as strings:

```rust
assert_io_read_to_string_eq!(reader1, reader2);
```

Rust `assertables` provides these macros for readers:

* `assert_io_read_to_string_eq!`

* `assert_io_read_to_string_eq_other!`

* `assert_io_read_to_string_ne!`

* `assert_io_read_to_string_ne_other!`

* `assert_io_read_to_string_lt!`

* `assert_io_read_to_string_lt_other!`

* `assert_io_read_to_string_le!`

* `assert_io_read_to_string_le_other!`

* `assert_io_read_to_string_gt!`

* `assert_io_read_to_string_gt_other!`

* `assert_io_read_to_string_ge!`

* `assert_io_read_to_string_ge_other!`

* `assert_io_read_to_string_contains!`

* `assert_io_read_to_string_matches!`


### Macros for commands

Rust programs can call shell commands.

For example, consider this shell command, which prints the word hello:

```sh
printf %s hello
```

Rust can create the shell command, by using this code:

```rust
use std::process::Command;
let mut command = Command::new("printf");
command.args(["%s", "hello"]);
```

Run the command and capture its output:

```rust
let output = command.output();
```

Capture the standard output text, which is encoded with a system-specific encoding, not necessarily UTF-8 encoding:

```rust
let stdout = output.unwrap().stdout;
```

Convert the standard output text to UTF-8 encoding:

```rust
let string = String::from_utf8(stdout).unwrap();
```

Rust can compare a command's standard output string to another command's standard output string, by using:

```rust
let string1 = String::from_utf8(command1.output().unwrap().stdout).unwrap();
let string2 = String::from_utf8(command2.output().unwrap().stdout).unwrap();
assert_eq!(string1, string2);
```

Rust `assertables` provides the macro `assert_command_stdout_eq!` that does the same kind of processing, by automatically converting commands into standard outputs, then into UTF-8 strings, then comparing them as strings:

```rust
assert_command_eq!(command1, command2);
```

Rust `assertables` provides these macros for commands and standard output:

* assert_command_stdout_eq.rs
* assert_command_stdout_eq_other.rs
* assert_command_stdout_contains.rs
* assert_command_stdout_is_match.rs

Rust `assertables` provides these macros for commands and standard error:

* assert_command_stderr_eq.rs
* assert_command_stderr_eq_other.rs
* assert_command_stderr_contains.rs
* assert_command_stderr_is_match.rs


### Macros for commands created via program and args

The previous section showed how Rust can create a shell command, by using this code:

```rust
use std::process::Command;
let mut command = Command::new("printf");
command.args(["%s", "hello"]);
```

The previous section showed that Rust `assertables` provides the macro `assert_command_stdout_eq!` such as:

```rust
use std::process::Command;
let mut command1 = Command::new("printf");
command1.args(["%s", "hello"]);
let mut command2 = Command::new("printf");
command2.args(["%s", "hello"]);
assert_command_eq!(string1, string2);
```

Rust `assertables` provides the macro `assert_program_args_eq` that does the same kind of processing, by automatically converting programs and args into commands, then to standard outputs, then into UTF-8 stringss, then comparing them as strings:

```rust
assert_program_args_eq!("printf", ["%s", "hello"], "printf", ["%s", "hello"]);
```

Rust `assertables` provides these macros for program args and standard output:

* `assert_program_args_stdout_eq!`

* `assert_program_args_stdout_eq_other!`

* `assert_program_args_stdout_contains!`

* `assert_program_args_stdout_is_match!`

Rust `assertables` provides these macros for program args and standard error:

* `assert_program_args_stderr_eq!`

* `assert_program_args_stderr_eq_other!`

* `assert_program_args_stderr_contains!`

* `assert_program_args_stderr_is_match!`


## Developers: how we write our test macros

We write each our test macros in three flavors:

* `assert_gt_as_result` returns a `Result` as `Ok`, `Err`. This macro contains all the logic, all the error formatting, etc. This macro is called by the other flavors below. This macros is also useful for runtime checks, such as when you want to know success or failure, yet you don't want to panic.

* `assert_gt` returns `()` or panics. This is the typical macro that most developers will use for testing. This macro wraps `assert_gt_as_result`. This macro provides two arms: one arm is for returning the error messsage as is, and one arm is for returning a developer's custom error message.

* `debug_assert_gt` return `()` or panics. This macro's statements are only enabled in non-optimized builds by default. An optimized build will not execute this macro's statements unless `-C debug-assertions` is passed to the compiler.

The sections below show each of the three flavors, using our simplest macro group: `assert_gt_as_result`, `assert_gt`, `debug_assert_gt`.


### assert_gt_as_result

The macro `assert_gt_as_result` returns a `Result` as `Ok`, `Err`.

The macro contains all the logic and all the error formatting.

The macro is called by the other flavors: directly by `assert_gt` and indirectly by `debug_assert_gt`.

The macros is useful for runtime checks, such as when you want to know success or failure, yet you don't want to panic.

Code:

```rust
#[macro_export]
macro_rules! assert_gt_as_result {
    ($a:expr, $b:expr $(,)?) => ({
        match (&$a, &$b) {
            (a_val, b_val) => {
                if a_val > b_val {
                    Ok(())
                } else {
                    Err(format!(
                        concat!(
                            "assertion failed: `assert_gt!(left, right)`\n",
                            "  left label: `{}`,\n",
                            "  left debug: `{:?}`,\n",
                            " right label: `{}`,\n",
                            " right debug: `{:?}`,\n",
                            "        left: `{:?}`,\n",
                            "       right: `{:?}`"
                        ),
                        stringify!($a), $a,
                        stringify!($b), $b,
                        a_val,
                        b_val
                    ))
                }
            }
        }
    });
}
```


### assert_gt

The macro `assert_gt` returns `()` or panics.

The macro is the typical macro that most developers will use for testing.

The macro wraps `assert_gt_as_result`.

The macro provides two arms: one arm is for returning the error messsage as is, and one arm is for returning a developer's custom error message.

Code:

```rust
#[macro_export]
macro_rules! assert_gt {
    ($a:expr, $b:expr $(,)?) => ({
        match assert_gt_result!($a, $b) {
            Ok(()) => (),
            Err(err) => panic!("{}", err),
        }
    });
    ($a:expr, $b:expr, $($message:tt)+) => ({
        match assert_gt!($a, $b) {
            Ok(()) => (),
            Err(_err) => panic!("{}", $($message)+),
        }
    });
}
```


### debug_assert_gt

The macro `debug_assert_gt` return `()` or panics.

The macro is the typical macro that most developers will use for runtime debugging during development, and possibly also for runtime debugging during production.

The macro's statements are only enabled in non-optimized builds by default. An optimized build will not execute this macro's statements unless `-C debug-assertions` is passed to the compiler.

The macro wraps `assert_gt`.

Code:

```rust
#[macro_export]
macro_rules! debug_assert_gt {
    ($($arg:tt)*) => {
        if $crate::cfg!(debug_assertions) {
            $crate::assert_gt!($($arg)*);
        }
    };
}
```
