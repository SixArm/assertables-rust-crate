<!--
tags: #assert #assertion #rust #testing #macros #tdd #testdrivendevelopment
-->

# Assertables: Rust crate of assert macros for testing

Assertables is a Rust crate of assert macros to improve your compile-time tests and run-time reliability.

* Crate: [https://crates.io/crates/assertables](https://crates.io/crates/assertables)
* Docs: [https://docs.rs/assertables/](https://docs.rs/assertables/)
* Repo: [https://github.com/sixarm/assertables-rust-crate/](https://github.com/sixarm/assertables-rust-crate/)
* Contact: [joel@joelparkerhenderson.com](mailto:joel@joelparkerhenderson.com)

## Introduction

The Assertables Rust crate provides many assert macros
that can help you develop, test, and debug.

* Test values with
  [assert_lt](https://docs.rs/assertables/9.6.1/assertables/assert_lt),
  [assert_gt](https://docs.rs/assertables/9.6.1/assertables/assert_gt),
  [assert_in](https://docs.rs/assertables/9.6.1/assertables/assert_in),
  […](https://docs.rs/assertables)
* Test groups with
  [assert_all](https://docs.rs/assertables/9.6.1/assertables/assert_all),
  [assert_any](https://docs.rs/assertables/9.6.1/assertables/assert_any),
  [assert_iter](https://docs.rs/assertables/9.6.1/assertables/assert_iter),
  […](https://docs.rs/assertables)
* Test wrappers with
  [assert_ok](https://docs.rs/assertables/9.6.1/assertables/assert_ok),
  [assert_some](https://docs.rs/assertables/9.6.1/assertables/assert_some),
  [assert_ready](https://docs.rs/assertables/9.6.1/assertables/assert_ready),
  […](https://docs.rs/assertables)
* Test matching with
  [assert_matches](https://docs.rs/assertables/9.6.1/assertables/assert_matches),
  [assert_is_match](https://docs.rs/assertables/9.6.1/assertables/assert_is_match),
  […](https://docs.rs/assertables)
* Test nearness with
  [assert_approx](https://docs.rs/assertables/9.6.1/assertables/assert_approx),
  [assert_abs_diff](https://docs.rs/assertables/9.6.1/assertables/assert_abs_diff),
  […](https://docs.rs/assertables/)
* Test programs with
  [assert_command](https://docs.rs/assertables/9.6.1/assertables/assert_command),
  [assert_status](https://docs.rs/assertables/9.6.1/assertables/assert_staus),
  […](https://docs.rs/assertables)
* Many more below.

To use this crate, add it to your file `Cargo.toml`:

```toml
assertables = "9.5.8"
```

Benefits:

* You can write better tests to improve reliability and maintainability.
* You can handle more corner cases without needing to write custom code.
* You can troubleshoot faster because error messages show more detail.

Learning:
[FAQ](https://github.com/SixArm/assertables-rust-crate/tree/main/help/faq),
[docs](https://docs.rs/assertables/),
[examples](https://github.com/SixArm/assertables-rust-crate/blob/main/tests/examples/),
[changes](https://github.com/SixArm/assertables-rust-crate/tree/main/CHANGES.md),
[upgrades](https://github.com/SixArm/assertables-rust-crate/tree/main/help/upgrades/upgrade-from-version-8-to-9),
[developing](https://github.com/SixArm/assertables-rust-crate/tree/main/help/developing/).

Comparisons:
[more_asserts](https://github.com/SixArm/assertables-rust-crate/tree/main/help/comparisons/more_asserts),
[cool_asserts](https://github.com/SixArm/assertables-rust-crate/tree/main/help/comparisons/cool_asserts),
[assert2](https://github.com/SixArm/assertables-rust-crate/tree/main/help/comparisons/assert2),
[claims](https://github.com/SixArm/assertables-rust-crate/tree/main/help/comparisons/claims),
[etc.](https://github.com/SixArm/assertables-rust-crate/tree/main/help/comparisons)

## Examples

Examples with numbers:

```rust
let i = 1;
assert_lt!(i, 5);
assert_diff_eq_x!(i, 5, 4);
assert_in_range!(&i, &(1..5));
```

Examples with strings:

```rust
let s = "hello";
assert_starts_with!(s, "h");
assert_contains!(s, "e");
assert_is_match!(Regex::new(r"h.*o").unwrap(), s);
```

Examples with arrays:

```rust
let a = [1, 2, 3];
assert_not_empty!(a);
assert_len_eq_x!(a, 3);
assert_all!(a.into_iter(), |i: i32| i < 4);
```

## Highlights

Values:

* [`assert_eq!(a, b)`](https://docs.rs/assertables/9.6.1/assertables/assert_eq) ≈ a = b ≈ equal to
* [`assert_ne!(a, b)`](https://docs.rs/assertables/9.6.1/assertables/assert_ne) ≈ a ≠ b ≈ not equal to
* [`assert_lt!(a, b)`](https://docs.rs/assertables/9.6.1/assertables/assert_lt) ≈ a < b ≈ less than
* [`assert_le!(a, b)`](https://docs.rs/assertables/9.6.1/assertables/assert_le) ≈ a ≤ b ≈ less than or equal to
* [`assert_gt!(a, b)`](https://docs.rs/assertables/9.6.1/assertables/assert_gt) ≈ a > b ≈ greater than
* [`assert_ge!(a, b)`](https://docs.rs/assertables/9.6.1/assertables/assert_ge) ≈ a ≥ b ≈ greater than or equal to

Nearness:

* [`assert_approx_eq!(a, b)`](https://docs.rs/assertables/9.6.1/assertables/assert_approx/assert_approx_eq) ≈ |a-b| ≤ 1e-6
* [`assert_diff_eq_x!(a, b, x)`](https://docs.rs/assertables/9.6.1/assertables/assert_diff/assert_diff_eq_x) ≈ (b-a) = x
* [`assert_abs_diff_eq_x!(a, b, x)`](https://docs.rs/assertables/9.6.1/assertables/assert_abs_diff/assert_abs_diff_eq_x) ≈ |b-a| = x
* [`assert_in_delta!(a, b, delta)`](https://docs.rs/assertables/9.6.1/assertables/assert_in/assert_in_delta) ≈ |a-b| ≤ Δ
* [`assert_in_epsilon!(a, b, epsilon)`](https://docs.rs/assertables/9.6.1/assertables/assert_in/assert_in_epsilon) ≈ |a-b| ≤ ε min(a,b)
* [`assert_in_range!(a, range)`](https://docs.rs/assertables/9.6.1/assertables/assert_in/assert_in_range) ≈ a ∈ range
* [`assert_eq_f32!(a, b)`](https://docs.rs/assertables/9.6.1/assertables/assert_eq_f32/assert_eq_f32) ≈ |a-b| ≤ f32::ε * 2 (v9.7)
* [`assert_eq_f64!(a, b)`](https://docs.rs/assertables/9.6.1/assertables/assert_eq_f64/assert_eq_f64) ≈ |a-b| ≤ f64::ε * 2 (v9.7)

Groups:

* [`assert_all!(group, predicate)`](https://docs.rs/assertables/9.6.1/assertables/assert_all) ≈ group.all(predicate)
* [`assert_any!(group, predicate)`](https://docs.rs/assertables/9.6.1/assertables/assert_any) ≈ group.any(predicate)
* [`assert_is_empty!(group)`](https://docs.rs/assertables/9.6.1/assertables/assert_is_empty/assert_is_empty) ≈ a.is_empty()
* [`assert_len_eq!(a, b)`](https://docs.rs/assertables/9.6.1/assertables/assert_len/assert_len_eq) ≈ a.len() = b.len()
* [`assert_count_eq!(a, b)`](https://docs.rs/assertables/9.6.1/assertables/assert_count/assert_count_eq) ≈ a.count() = b.count()

Matching:

* [`assert_starts_with!(sequence, x)`](https://docs.rs/assertables/9.6.1/assertables/assert_starts_with) ≈ sequence.starts_with(x)
* [`assert_ends_with!(sequence, x)`](https://docs.rs/assertables/9.6.1/assertables/assert_ends_with) ≈ sequence.ends_with(x)
* [`assert_contains!(container, x)`](https://docs.rs/assertables/9.6.1/assertables/assert_contains) ≈ container.contains(x)
* [`assert_is_match!(matcher, x)`](https://docs.rs/assertables/9.6.1/assertables/assert_is_match) ≈ matcher.is_match(x)
* [`assert_matches!(expr, pattern)`](https://docs.rs/assertables/9.6.1/assertables/assert_matches) ≈ matches!(expr, pattern)
* [`assert_email_address!(expr)`](https://docs.rs/assertables/9.6.1/assertables/assert_email_address) ≈ expr has local@domain

Results:

* [`assert_ok!(a)`](https://docs.rs/assertables/9.6.1/assertables/assert_ok) ≈ a is Ok
* [`assert_err!(a)`](https://docs.rs/assertables/9.6.1/assertables/assert_err) ≈ a is Err
* [`assert_ok_eq_x!(a, x)`](https://docs.rs/assertables/9.6.1/assertables/assert_ok/assert_ok_eq_x) ≈ a is Ok ⇒ unwrap = x

Options:

* [`assert_some!(a)`](https://docs.rs/assertables/9.6.1/assertables/assert_some) ≈ a is Some
* [`assert_none!(a)`](https://docs.rs/assertables/9.6.1/assertables/assert_none) ≈ a is None
* [`assert_some_eq_x!(a, x)`](https://docs.rs/assertables/9.6.1/assertables/assert_some/assert_some_eq_x) ≈ a is Some ⇒ unwrap = x

Polls:

* [`assert_ready!(a)`](https://docs.rs/assertables/9.6.1/assertables/assert_ready) ≈ a is Ready
* [`assert_pending!(a)`](https://docs.rs/assertables/9.6.1/assertables/assert_pending) ≈ a is Pending
* [`assert_ready_eq_x!(a, x)`](https://docs.rs/assertables/9.6.1/assertables/assert_ready/assert_ready_eq_x) ≈ a is Ready ⇒ unwrap = x

Readers:

* [`assert_fs_read_to_string_eq_x!(path, x`](https://docs.rs/assertables/9.6.1/assertables/assert_fs_read_to_string) ≈ path ⇒ file ⇒ string = x
* [`assert_io_read_to_string_eq_x!(reader, x)`](https://docs.rs/assertables/9.6.1/assertables/assert_io_read_to_string) ≈ reader ⇒ bytes ⇒ string = x

Collections:

* [`assert_iter_eq!(a, b)`](https://docs.rs/assertables/9.6.1/assertables/assert_iter) ≈ a into iter = b into iter
* [`assert_set_eq!(a, b)`](https://docs.rs/assertables/9.6.1/assertables/assert_set) ≈ a into set = b into set
* [`assert_bag_eq!(a, b)`](https://docs.rs/assertables/9.6.1/assertables/assert_bag) ≈ a into bag = b into bag

Commands:

* [`assert_command_stdout_eq_x!(command, x)`](https://docs.rs/assertables/9.6.1/assertables/assert_command) ≈ command ⇒ stdout = x
* [`assert_program_args_stdout_eq_x!(program, args, x)`](https://docs.rs/assertables/9.6.1/assertables/assert_program_args) ≈ program.args ⇒ stdout = x

Status:

* [`assert_status_success!(a)`](https://docs.rs/assertables/9.6.1/assertables/assert_status/assert_status_success) ≈ a.status().success()
* [`assert_status_code_value_eq_x!(a, x)`](https://docs.rs/assertables/9.6.1/assertables/assert_status/assert_status_code_value_eq_x) ≈ a.status().code().unwrap() = x

Infix:

* [`assert_infix!(a == b)`](https://docs.rs/assertables/9.6.1/assertables/assert_infix) ≈ order operators == != < <= > >=
* [`assert_infix!(a && b)`](https://docs.rs/assertables/9.6.1/assertables/assert_infix) ≈ logic operators && || ^ & |

For a complete list of modules and macros, see the [docs](https://docs.rs/assertables/).


## Forms

All the macros have forms for an optional message:

* [`assert_gt!(a, b)`](https://docs.rs/assertables/9.6.1/assertables/macro.assert_gt.html) ≈ default message
* [`assert_gt!(a, b, "your text")`](https://docs.rs/assertables/9.6.1/assertables/macro.assert_gt.html) ≈ custom message

All the macros have forms for different outcomes:

* [`assert_gt!(1, 2)`](https://docs.rs/assertables/9.6.1/assertables/macro.assert_gt.html) ≈ panic
* [`assert_gt_as_result!(1, 2)`](https://docs.rs/assertables/9.6.1/assertables/macro.assert_gt_as_result.html) ≈  Result Err
* [`debug_assert_gt!(a, b)`](https://docs.rs/assertables/9.6.1/assertables/macro.debug_assert_gt.html) ≈ panic in debug mode

Many of the macros have a form "compare left item to right item" that compares
items of the same kind, and a form "compare left item to right expression" that
compares one item to any arbitrary expression:

* [`assert_len_eq!(a, b)`](https://docs.rs/assertables/9.6.1/assertables/macro.assert_ok_eq.html) ≈ a.len() = b.len()
* [`assert_len_eq_x!(a, x)`](https://docs.rs/assertables/9.6.1/assertables/macro.assert_ok_eq_x.html) ≈ a.len() = x

Many of the macros has a "success return", which means the macro returns data that you can optionally use for more testing.

* [`let inner = assert_ok!(result)`](https://docs.rs/assertables/9.6.1/assertables/macro.assert_ok.html)
* [`let string = assert_fs_read_to_string_ne!("alfa.txt", "")`](https://docs.rs/assertables/9.6.1/assertables/macro.assert_fs_read_to_string_ne.html)
* [`let stdout = assert_command_stdout_gt!("ls", vec![b' '])`](https://docs.rs/assertables/9.6.1/assertables/macro.assert_command_stdout_gt.html)

## Tracking

* Package: assertables-rust-crate
* Version: 9.5.8
* Created: 2021-03-30T15:47:49Z
* Updated: 2025-06-20T14:05:28Z
* License: MIT or Apache-2.0 or GPL-2.0 or GPL-3.0 or contact us for more
* Contact: Joel Parker Henderson <joel@joelparkerhenderson.com>
