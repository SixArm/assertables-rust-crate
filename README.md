# Assertables: Rust crate of "assert" macros for testing

The `assertables` Rust crate provides many "assert" macros
to help with compile-time testing and run-time reliability

Crate:
[https://crates.io/crates/assertables](https://crates.io/crates/assertables)

Docs:
[https://docs.rs/assertables/](https://docs.rs/assertables/)

Repo:
[https://github.com/sixarm/assertables-rust-crate/](https://github.com/sixarm/assertables-rust-crate/)


## Highlights

Value macros such as:

  * `assert_gt!(value1, value2)`

  * Means `value1 > value2`

Set macros such as:

  * `assert_set_subset!(set1, set2)`

  * Which means `set1 ⊆ set2`

Function macros such as:

  * `assert_fn_eq!(function, input, output)`

  * Means `function(input) == output`

Reader macros such as:

  * `assert_read_to_string_eq!(reader, string)`

  * Means `reader.read_to_string() == string`

Commmand macros such as:

  * `assert_command_stdout_eq!(command, expr)`

  * Means `String::from_utf8(command.output().unwrap().stdout).unwrap() == expr`


## Naming conventions

Abbreviations:

*  `eq` means equal; `ne` means not equal.

* `lt` means less than; `le` means less than or equal.

* `gt` means greater than; `ge` means greater than or equal.

Shorthands:

  * `reader` means implements `.read_to_string(…)` such as `std::io::Read`.

  * `matcher` means implements `.is_match(…)` such as `regex::Regex`.

  * `containee` means usable inside `.contains(…)` such as a `std::string::String` substring.

  * `set` means a collection such as `::std::collections::BTreeSet`.

  * `bag` means a collection such as `::std::collections::BTreeMap` with key counts.


## Forms for panic! or Result Err()

The macros have forms for immediate interrupts or returning results:

   * `assert_gt!(a, b)` => `()` or `panic!(…)`

   * `assert_gt_as_result!(a, b)` => `Result Ok(())` or `Result Err(…)`


## Forms for default messages or custom messages

The macros have forms for default messages or custom messages.

Example:

   * `assert_gt(1, 2)` => `panic!("assertion failed: `assert_gt(1, 2)`…")`

   * `assert_gt(1, 2, "lorem ipsum")` => `panic!("lorem ipsum")`


## Forms for comparing an expression versus equivalent

Some macros have forms for comparing to an expression (`expr`)  versus an equivalent (`other`):

Example:

   * `assert_read_to_string_eq!(reader, expr)` => `reader.read_to_string() == expr`

   * `assert_read_to_string_eq_other!(reader1, reader2)` => `reader1.read_to_string() == reader2.read_to_string()`


## assert_* for values

Compare values.

* `assert_eq!(a, b)` ~ a == b

* `assert_ne!(a, b)` ~ a != b

* `assert_lt!(a, b)` ~ a < b

* `assert_le!(a, b)` ~ a <= b

* `assert_gt!(a, b)` ~ a > b

* `assert_ge!(a, b)` ~ a >= b


## assert_set_* for set collection comparisons

These macros help with comparison of set parameters, such as two arrays or
two vectors. where the item order does not matter, and the item count does
not matter. The macros convert inputs into HashSet iterators.

* `assert_set_eq!(a, b)` ~ set a == set b

* `assert_set_ne!(a, b)` ~ set a != set b

* `assert_set_subset!(a, b)` ~ set a ⊆ set b

* `assert_set_superset!(a, b)` ~ set a ⊇ set b

* `assert_set_joint!(a, b)` ~ set a is joint with set b

* `assert_set_disjoint!(a, b)` ~ set a is disjoint with set b


## assert_bag_* for bag collection comparisons

These macros help with comparison of bag parameters, such as comparison of
two arrays or two vectors, where the item order does not matter, and the
item count does matter. The macros convert inputs into HashMap iterators.

* `assert_bag_eq(a, b)` ~ bag a == bag b

* `assert_bag_ne(a, b)` ~ bag a != bag b

* `assert_bag_subbag(a, b)` ~ bag a ⊆ bag b

* `assert_bag_superbag(a, b)` ~ bag a ⊇ bag b


## assert_fn_* for function return comparisons

* `assert_fn_eq!(f, a, b)` ~ f(a) == b

* `assert_fn_eq_other!(f, a, b)` ~ f(a) == f(b)

* `assert_fn_ne!(f, a, b)` ~ f(a) != b

* `assert_fn_ne_other!(f, a, b)` ~ f(a) != f(b)

* `assert_fn_lt!(f, a, b)` ~ f(a) < v

* `assert_fn_lt_other!(f, a, b)` ~ f(a) < f(b)

* `assert_fn_le!(f, a, b)` ~ f(a) <= b

* `assert_fn_le_other!(f, a, b)` ~ f(a) <= f(b)

* `assert_fn_gt!(f, a, b)` ~ f(a) > b

* `assert_fn_gt_other!(f, a, b)` ~ f(a) > f(b)

* `assert_fn_ge!(f, a, b)` ~ f(a) >= b

* `assert_fn_ge_other!(f, a, b)` ~ f(a) >= f(b)


## assert_fn_ok_* for function Ok() comparisons

* `assert_fn_ok_eq!(f, a, b)` ~ f(a).unwrap() == b

* `assert_fn_ok_eq_other!(f, a, b)` ~ f(a).unwrap() == f(b).unwrap()

* `assert_fn_ok_ne!(f, a, b)` ~ f(a).unwrap() != b

* `assert_fn_ok_ne_other!(f, a, b)` ~ f(a).unwrap() != f(b).unwrap()

* `assert_fn_ok_lt!(f, a, b)` ~ f(a).unwrap() < b

* `assert_fn_ok_lt_other!(f, a, b)` ~ f(a).unwrap() < f(b).unwrap()

* `assert_fn_ok_le!(f, a, b)` ~ f(a).unwrap() <= b

* `assert_fn_ok_le_other!(f, a, b)` ~ f(a).unwrap() <= f(b).unwrap()

* `assert_fn_ok_gt!(f, a, b)` ~ f(a).unwrap() > b

* `assert_fn_ok_gt_other!(f, a, b)` ~ f(a).unwrap() > f(b).unwrap()

* `assert_fn_ok_gt!(f, a, b)` ~ f(a).unwrap() > b

* `assert_fn_ok_gt_other!(f, a, b)` ~ f(a).unwrap() > f(b).unwrap()


## assert_fn_err_* for function Err() comparisons

* `assert_fn_err_eq!(f, a, b)` ~ f(a).unwrap_err() == b

* `assert_fn_err_eq_other!(f, a, b)` ~ f(a).unwrap_err() == f(b).unwrap_err()

* `assert_fn_err_ne!(f, a, b)` ~ f(a).unwrap_err() != b

* `assert_fn_err_ne_other!(f, a, b)` ~ f(a).unwrap_err() != f(b).unwrap_err()

* `assert_fn_err_lt!(f, a, b)` ~ f(a).unwrap_err() < b

* `assert_fn_err_lt_other!(f, a, b)` ~ f(a).unwrap_err() < f(b).unwrap_err()

* `assert_fn_err_le!(f, a, b)` ~ f(a).unwrap_err() <= b

* `assert_fn_err_le_other!(f, a, b)` ~ f(a).unwrap_err() <= f(b).unwrap_err()

* `assert_fn_err_gt!(f, a, b)` ~ f(a).unwrap_err() > b

* `assert_fn_err_gt_other!(f, a, b)` ~ f(a).unwrap_err() > f(b).unwrap_err()

* `assert_fn_err_ge!(f, a, b)`~ f(a).unwrap_err() >= b

* `assert_fn_err_ge_other!(f, a, b)`~ f(a).unwrap_err() >= f(b).unwrap_err()


## assert_read_to_string_* for std::io::Read comparisons

These macros help with readers, such as file handles, byte
arrays, input streams, and the trait std::io::Read.

* `assert_read_to_string_eq!(a, b)` ~ a.read_to_string() == b

* `assert_read_to_string_eq_other!(a, b)` ~ a.read_to_string() == b.read_to_string()

* `assert_read_to_string_ne!(a, b)` ~ a.read_to_string() != b

* `assert_read_to_string_ne_other!(a, b)` ~ a.read_to_string() != b.read_to_string()

* `assert_read_to_string_lt!(a, b)` ~ a.read_to_string() < b

* `assert_read_to_string_lt_other!(a, b)` ~ a.read_to_string() < b.read_to_string()

* `assert_read_to_string_le!(a, b)` ~ a.read_to_string() <= b

* `assert_read_to_string_le_other!(a, b)` ~ a.read_to_string() <= b.read_to_string()

* `assert_read_to_string_gt!(a, b)` ~ a.read_to_string() > b

* `assert_read_to_string_gt_other!(a, b)` ~ a.read_to_string() > b.read_to_string()

* `assert_read_to_string_ge!(a, b)` ~ a.read_to_string() >= b

* `assert_read_to_string_ge_other!(a, b)` ~ a.read_to_string() >= b.read_to_string()


## assert_command_ for process command comparisons

Using standard output a.k.a. stdout:

* `assert_command_stdout_eq!(command, value)` ~ String::from_utf8(command.output().unwrap().stdout).unwrap() == value

* `assert_command_stdout_eq_other!(command, command)` ~ String::from_utf8(command.output().unwrap().stdout).unwrap() == ng::from_utf8(command.output().unwrap().stdout).unwrap()

* `assert_command_stdout_contains!(command, containee)` ~ String::from_utf8(command.output().unwrap().stdout).unwrap().contains(containee)

* `assert_command_stdout_matches!(command, matcher)` ~ regex.captures(String::from_utf8(command.output().unwrap().stdout).ap())

Using standard error a.k.a. stderr:

* `assert_command_stderr_eq!(command, value)` ~ String::from_utf8(command.output().unwrap().stderr).unwrap() == value

* `assert_command_stderr_eq_other!(command, command)` ~ String::from_utf8(command.output().unwrap().stderr).unwrap() == ng::from_utf8(command.output().unwrap().stdout).unwrap()

* `assert_command_stderr_contains!(command, containee)` ~ String::from_utf8(command.output().unwrap().stderr).unwrap().contains(containee)

* `assert_command_stderr_matches!(command, matcher)` ~ regex.captures(String::from_utf8(command.output().unwrap().stderr).ap())


## Tracking

* Package: assertables-rust-crate
* Version: 5.0.0
* Created: 2021-03-30T15:47:49Z
* Updated: 2022-02-28T15:45:38Z
* License: GPL-2.0-or-later or contact us for custom license
* Contact: Joel Parker Henderson (joel@sixarm.com)
