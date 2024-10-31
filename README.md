# Assertables: Rust crate of assert macros for testing

Assertables is a Rust crate that provides many assert macros 
to improve your compile-time tests and run-time reliability.

* Crate: [https://crates.io/crates/assertables](https://crates.io/crates/assertables)
* Docs: [https://docs.rs/assertables/](https://docs.rs/assertables/)
* Repo: [https://github.com/sixarm/assertables-rust-crate/](https://github.com/sixarm/assertables-rust-crate/)
* Contact: [joel@joelparkerhenderson.com](mailto:joel@joelparkerhenderson.com)

## Introduction

The Assertables Rust crate provides many assert macros 
that can help you develop, test, and debug.

* Test values with 
  [assert_lt](https://docs.rs/assertables/9.2.0/assertables/assert_lt), 
  [assert_gt](https://docs.rs/assertables/9.2.0/assertables/assert_gt), 
  [more…](https://docs.rs/assertables/9.2.0/assertables/assert_approx)
* Test results with 
  [assert_ok](https://docs.rs/assertables/9.2.0/assertables/assert_ok),
  [assert_err](https://docs.rs/assertables/9.2.0/assertables/assert_err),
  [more…](https://docs.rs/assertables/9.2.0/assertables/assert_ok_eq_x)
* Test groups with 
  [assert_all](https://docs.rs/assertables/9.2.0/assertables/assert_all),
  [assert_any](https://docs.rs/assertables/9.2.0/assertables/assert_any),
  [more…](https://docs.rs/assertables/9.2.0/assertables/assert_iter)
* Many more below.

To use this crate, add it to your file `Cargo.toml`:

```toml
assertables = "9.2.0"
```

Benefits:

* You can write better tests to improve reliability and maintainability.
* You can handle more corner cases without needing to write custom code.
* You can troubleshoot faster because error messages show more detail.

Features:

* Easy to use: everything is well-documented with runnable examples.
* Zero overhead: if you don't use a macro, then it's not compiled.
* Multiple forms: for panic, debug, result return, success return.

Learning: 
[FAQ](https://github.com/SixArm/assertables-rust-crate/tree/main/help/faq),
[examples](https://github.com/SixArm/assertables-rust-crate/blob/main/tests/examples/),
[changes](https://github.com/SixArm/assertables-rust-crate/tree/main/CHANGES.md),
[upgrades](https://github.com/SixArm/assertables-rust-crate/tree/main/help/upgrades/upgrade-from-version-8-to-9),
[docs](https://docs.rs/assertables/).

Comparisons: 
[more_asserts](https://github.com/SixArm/assertables-rust-crate/tree/main/help/comparisons/more_asserts), [cool_asserts](https://github.com/SixArm/assertables-rust-crate/tree/main/help/comparisons/cool_asserts), 
[assert2](https://github.com/SixArm/assertables-rust-crate/tree/main/help/comparisons/assert2), 
[claims](https://github.com/SixArm/assertables-rust-crate/tree/main/help/comparisons/more_asserts),
[etc.](https://github.com/SixArm/assertables-rust-crate/tree/main/help/comparisons)

## Highlights

Values:

* [`assert_eq!(a, b)`](https://docs.rs/assertables/9.2.0/assertables/assert_eq) ≈ a = b
* [`assert_ne!(a, b)`](https://docs.rs/assertables/9.2.0/assertables/assert_ne) ≈ a ≠ b
* [`assert_lt!(a, b)`](https://docs.rs/assertables/9.2.0/assertables/assert_lt) ≈ a < b
* [`assert_le!(a, b)`](https://docs.rs/assertables/9.2.0/assertables/assert_le) ≈ a ≤ b
* [`assert_gt!(a, b)`](https://docs.rs/assertables/9.2.0/assertables/assert_gt) ≈ a > b
* [`assert_ge!(a, b)`](https://docs.rs/assertables/9.2.0/assertables/assert_ge) ≈ a ≥ b

Approximations:

* [`assert_approx_eq!(a, b)`](https://docs.rs/assertables/9.2.0/assertables/assert_approx/assert_approx_eq) ≈ |a-b| ≤ 1e-6
* [`assert_abs_diff_eq!(a, b, delta)`](https://docs.rs/assertables/9.2.0/assertables/assert_abs_diff/assert_abs_diff_eq) ≈ |a-b| = Δ
* [`assert_in_delta!(a, b, delta)`](https://docs.rs/assertables/9.2.0/assertables/assert_in/assert_in_delta) ≈ |a-b| ≤ Δ
* [`assert_in_epsilon!(a, b, epsilon)`](https://docs.rs/assertables/9.2.0/assertables/assert_in/assert_in_epsilon) ≈ |a-b| ≤ ε min(a,b)

Groups:

* [`assert_all!(group, predicate)`](https://docs.rs/assertables/9.2.0/assertables/assert_all) ≈ group.all(predicate)
* [`assert_any!(group, predicate)`](https://docs.rs/assertables/9.2.0/assertables/assert_any) ≈ group.any(predicate)
* [`assert_is_empty!(group)`](https://docs.rs/assertables/9.2.0/assertables/assert_is_empty/assert_is_empty) ≈ a.is_empty()
* [`assert_len_eq!(a, b)`](https://docs.rs/assertables/9.2.0/assertables/assert_len/assert_len_eq) ≈ a.len() = b.len()
* [`assert_count_eq!(a, b)`](https://docs.rs/assertables/9.2.0/assertables/assert_count/assert_count_eq) ≈ a.count() = b.count()

Matching:

* [`assert_starts_with!(sequence, x)`](https://docs.rs/assertables/9.2.0/assertables/assert_starts_with) ≈ sequence.starts_with(x)
* [`assert_ends_with!(sequence, x)`](https://docs.rs/assertables/9.2.0/assertables/assert_ends_with) ≈ sequence.ends_with(x)
* [`assert_contains!(container, x)`](https://docs.rs/assertables/9.2.0/assertables/assert_contains) ≈ container.contains(x)
* [`assert_is_match!(matcher, x)`](https://docs.rs/assertables/9.2.0/assertables/assert_is_match) ≈ matcher.is_match(x)
* [`assert_matches!(expr, pattern)`](https://docs.rs/assertables/9.2.0/assertables/assert_matches) ≈ matches!(expr, pattern)

Results:

* [`assert_ok!(a)`](https://docs.rs/assertables/9.2.0/assertables/assert_ok) ≈ a is Ok
* [`assert_err!(a)`](https://docs.rs/assertables/9.2.0/assertables/assert_err) ≈ a is Err
* [`assert_ok_eq_x!(a, x)`](https://docs.rs/assertables/9.2.0/assertables/assert_ok/assert_ok_eq_x) ≈ (a is Ok ⇒ unwrap) = x

Options:

* [`assert_some!(a)`](https://docs.rs/assertables/9.2.0/assertables/assert_some) ≈ a is Some
* [`assert_none!(a)`](https://docs.rs/assertables/9.2.0/assertables/assert_none) ≈ a is None
* [`assert_some_eq_x!(a, x)`](https://docs.rs/assertables/9.2.0/assertables/assert_some/assert_some_eq_x) ≈ (a is Some ⇒ unwrap) = x

Polls:

* [`assert_ready!(a)`](https://docs.rs/assertables/9.2.0/assertables/assert_ready) ≈ a is Ready
* [`assert_pending!(a)`](https://docs.rs/assertables/9.2.0/assertables/assert_pending) ≈ a is Pending
* [`assert_ready_eq_x!(a, x)`](https://docs.rs/assertables/9.2.0/assertables/assert_ready/assert_ready_eq_x) ≈ (a is Ready ⇒ unwrap) = x

Readers:

* [`assert_fs_read_to_string_eq!(a_path, b_path)`](https://docs.rs/assertables/9.2.0/assertables/assert_fs_read_to_string_eq) ≈ (a_path ⇒ string) = (b_path ⇒ string)
* [`assert_io_read_to_string_eq!(a_bytes, b_bytes)`](https://docs.rs/assertables/9.2.0/assertables/assert_io_read_to_string) ≈ (a_bytes ⇒ string) = (b_bytes ⇒ string)

Collections:

* [`assert_iter_eq!(arr1, arr2)`](https://docs.rs/assertables/9.2.0/assertables/assert_iter) ≈ a into iter = b into iter
* [`assert_set_eq!(vec1, vec2)`](https://docs.rs/assertables/9.2.0/assertables/assert_set) ≈ a into set = b into set
* [`assert_bag_eq!(map1, map2)`](https://docs.rs/assertables/9.2.0/assertables/assert_bag) ≈ a into bag = b into bag

Infix notation:

* [`assert_infix!(a == b)`](https://docs.rs/assertables/9.2.0/assertables/assert_infix) ≈ order operators == != < <= > >=
* [`assert_infix!(a && b)`](https://docs.rs/assertables/9.2.0/assertables/assert_infix) ≈ logic operators && || ^ & |

For a complete list of modules and macros, see the [docs](https://docs.rs/assertables/).


## Forms

All the macros have forms for an optional message:

* [`assert_gt!(a, b)`](https://docs.rs/assertables/9.2.0/assertables/macro.assert_gt.html) ≈ default message
* [`assert_gt!(a, b, "your text")`](https://docs.rs/assertables/9.2.0/assertables/macro.assert_gt.html) ≈ custom message

All the macros have forms for different outcomes:

* [`assert_gt!(1, 2)`](https://docs.rs/assertables/9.2.0/assertables/macro.assert_gt.html) ≈ panic
* [`assert_gt_as_result!(1, 2)`](https://docs.rs/assertables/9.2.0/assertables/macro.assert_gt_as_result.html) ≈  Result Err
* [`debug_assert_gt!(a, b)`](https://docs.rs/assertables/9.2.0/assertables/macro.debug_assert_gt.html) ≈ panic in debug mode

Many of the macros have a form "compare left item to right item" that compares
items of the same kind, and a form "compare left item to right expression" that
compares one item to any arbitrary expression:

* [`assert_len_eq!(a, b)`](https://docs.rs/assertables/9.2.0/assertables/macro.assert_ok_eq.html) ≈ a.len() = b.len()
* [`assert_len_eq_x!(a, x)`](https://docs.rs/assertables/9.2.0/assertables/macro.assert_ok_eq_x.html) ≈ a.len() = x

Many of the macros has a "success return", which means the macro returns data that you can optionally use for more testing.

* [`let inner = assert_ok!(result)`](https://docs.rs/assertables/9.2.0/assertables/macro.assert_ok.html)
* [`let string = assert_fs_read_to_string_ne!("alfa.txt", "")`](https://docs.rs/assertables/9.2.0/assertables/macro.assert_fs_read_to_string_ne.html)
* [`let stdout = assert_command_stdout_gt!("ls", vec![b' '])`](https://docs.rs/assertables/9.2.0/assertables/macro.assert_command_stdout_gt.html)


## Tracking

* Package: assertables-rust-crate
* Version: 9.2.0
* Created: 2021-03-30T15:47:49Z
* Updated: 2024-10-31T11:04:30Z
* License: MIT or Apache-2.0 or GPL-2.0 or GPL-3.0 or contact us for more
* Contact: Joel Parker Henderson (joel@joelparkerhenderson.com)
