# Assertables: Rust crate of assert macros for testing

The `assertables` Rust crate provides many assert macros to improve your
compile-time tests and run-time reliability.

* Crate: [https://crates.io/crates/assertables](https://crates.io/crates/assertables)
* Docs: [https://docs.rs/assertables/](https://docs.rs/assertables/)
* Repo: [https://github.com/sixarm/assertables-rust-crate/](https://github.com/sixarm/assertables-rust-crate/)
* Contact: [joel@joelparkerhenderson.com](mailto:joel@joelparkerhenderson.com)


## Introduction

The Rust programming language provides assert macros such as `assert!(x)` to test code.

* [`assert!(a);`](https://doc.rust-lang.org/std/macro.assert.html) `// a is true`
* [`assert_eq!(a, b);`](https://doc.rust-lang.org/std/macro.assert_eq.html) `// a is equal to b`
* [`assert_ne!(a, b);`](https://doc.rust-lang.org/std/macro.assert_ne.html) `// a is not equal to b`

The assertables crate provides many more, to help you work with numbers,
strings, results, options, polls, iterators, files, streams, commands, and more. 

Examples:

```rust
use assertables::*;
let s = "hello world";
assert_matches!(s, "hello world");
assert_starts_with!(s, "hello");
assert_ends_with!(s, "world");
assert_contains!(s, "o");
assert_len_eq!(s, "***********");
assert_all!(s.chars(), |c: char| c < 'x');
assert_any!(s.chars(), |c: char| c.is_whitespace());
```

To use the macros, add this to your `Cargo.toml` file:

```toml
[dev-dependencies]
assertables = "*"
```

Top benefits:

1. You can write better tests to improve reliability and maintainability.
2. You can handle more corner cases without needing to write custom code.
3. You can troubleshoot faster because error messages show specifics.

Top features:

1. Easy to use: everything is well-documented with runnable examples.
2. Zero overhead: if you don't use a macro, then it's never compiled.
3. Runtime options: all the assertables macros have runtime versions.

Top comparison crates: 
    [`assert_matches`](https://crates.io/crates/assert_matches),
    [`assert_approx_eq`](https://crates.io/crates/assert_approx_eq),
    [`more_asserts`](https://crates.io/crates/more_asserts),
    [`cool_asserts`](https://crates.io/crates/cool_asserts).
    [`claims`](https://crates.io/crates/claims).

## Highlights

Values:

* [`assert_eq!(a, b);`](https://docs.rs/assertables/8.18.0/assertables/assert_eq) `// equal to`
* [`assert_ne!(a, b);`](https://docs.rs/assertables/8.18.0/assertables/assert_ne) `// not equal to`
* [`assert_lt!(a, b);`](https://docs.rs/assertables/8.18.0/assertables/assert_lt) `// less than`
* [`assert_le!(a, b);`](https://docs.rs/assertables/8.18.0/assertables/assert_le) `// less than or equal to`
* [`assert_gt!(a, b);`](https://docs.rs/assertables/8.18.0/assertables/assert_gt) `// greater than`
* [`assert_ge!(a, b);`](https://docs.rs/assertables/8.18.0/assertables/assert_ge) `// greater than or equal to`

Approximations:

* [`assert_approx_eq!(number, number);`](https://docs.rs/assertables/8.18.0/assertables/assert_approx) `// |a-b| ≤ 1e-6`
* [`assert_in_delta!(number, number, delta);`](https://docs.rs/assertables/8.18.0/assertables/assert_in_delta) `// |a-b| ≤ delta`
* [`assert_in_epsilon!(number, number, epsilon);`](https://docs.rs/assertables/8.18.0/assertables/assert_in_epsilon) `// |a-b| ≤ epsilon * min(a,b)`

Groups for iterators, chars, etc.:

* [`assert_all!(group, predicate);`](https://docs.rs/assertables/8.18.0/assertables/assert_all) `// group.all(predicate)`
* [`assert_any!(group, predicate);`](https://docs.rs/assertables/8.18.0/assertables/assert_any) `// group.any(predicate)`

Infix for order operators, logic operators, etc.:

* [`assert_infix!(a == b);`](https://docs.rs/assertables/8.18.0/assertables/assert_infix) `// order: == != < <= > >=`
* [`assert_infix!(a && b);`](https://docs.rs/assertables/8.18.0/assertables/assert_infix) `// logic: && || ^ & |`

Parts for strings, vectors, etc.:

* [`assert_starts_with!(whole, part);`](https://docs.rs/assertables/8.18.0/assertables/assert_starts_with) `// whole.starts_with(part)`
* [`assert_ends_with!(whole, part);`](https://docs.rs/assertables/8.18.0/assertables/assert_ends_with) `// whole.ends_with(part)`

Lengths and counts for strings, vectors, iterators, etc.:

* [`assert_len!(item);`](https://docs.rs/assertables/8.18.0/assertables/assert_len) `// item.len()`
* [`assert_count!(item);`](https://docs.rs/assertables/8.18.0/assertables/assert_count) `// item.count()`
* [`assert_is_empty!(item);`](https://docs.rs/assertables/8.18.0/assertables/assert_is_empty) `// item.is_empty()`

Matching for strings, regex, etc.:

* [`assert_matches!(expression, pattern);`](module@crate::assert_matches) `// matches!(expression, pattern)`
* [`assert_is_match!(matcher, matchee);`](https://docs.rs/assertables/8.18.0/assertables/assert_is_match) `// matcher.is_match(matchee)`
* [`assert_contains!(container, containee);`](https://docs.rs/assertables/8.18.0/assertables/assert_contains) `// container.contains(containee)`

Collections for arrays, vectors, iterators, sets, maps:

* [`assert_iter_eq!(arr1, arr2);`](https://docs.rs/assertables/8.18.0/assertables/assert_iter) `// eq ne lt le gt ge`
* [`assert_set_eq!(vec1, vec2);`](https://docs.rs/assertables/8.18.0/assertables/assert_set) `// eq ne lt le gt ge etc.`
* [`assert_bag_eq!(map1, map2);`](https://docs.rs/assertables/8.18.0/assertables/assert_bag) `// eq ne lt le gt ge etc.`

Result Ok/Err:

* [`assert_ok!(result);`](https://docs.rs/assertables/8.18.0/assertables/assert_ok) `// eq ne lt le gt ge`
* [`assert_err!(result);`](https://docs.rs/assertables/8.18.0/assertables/assert_err) `// eq ne lt le gt ge`
  
Option Some/None:

* [`assert_some!(option);`](https://docs.rs/assertables/8.18.0/assertables/assert_some) `// eq ne lt le gt ge`
* [`assert_none!(option);`](https://docs.rs/assertables/8.18.0/assertables/assert_none)

Poll Ready/Pending:

* [`assert_ready!(poll);`](https://docs.rs/assertables/8.18.0/assertables/assert_ready) `// eq ne lt le gt ge`
* [`assert_pending!(poll);`](https://docs.rs/assertables/8.18.0/assertables/assert_pending)

Read file system paths and input/output streams:

* [`assert_fs_read_to_string_eq!(path1, path2);`](https://docs.rs/assertables/8.18.0/assertables/assert_fs_read_to_string) `// eq ne lt le gt ge`
* [`assert_io_read_to_string_eq!(stream1, stream2);`](https://docs.rs/assertables/8.18.0/assertables/assert_io_read_to_string) `// eq ne lt le gt ge`

Run commands and programs then assert on stdout or stderr:

* [`assert_command_stdout_eq!(command1, command2);`](https://docs.rs/assertables/8.18.0/assertables/assert_command) `// eq ne lt le gt ge etc.`
* [`assert_program_args_stdout_eq!(program1, args1, program2, args2);`](https://docs.rs/assertables/8.18.0/assertables/assert_program_args) `// eq ne lt le gt ge etc.`

Function comparisons, which are especially good for refactoring:

* [`assert_fn_eq!(fn1, fn2);`](https://docs.rs/assertables/8.18.0/assertables/assert_fn) `// functions that return values`
* [`assert_fn_ok_eq!(fn1, fn2);`](https://docs.rs/assertables/8.18.0/assertables/assert_fn_ok) `// functions that return Ok`
* [`assert_fn_err_eq!(fn1, fn2);`](https://docs.rs/assertables/8.18.0/assertables/assert_fn_err) `// functions that return Err`


## Forms

All assertables macros have forms for different outcomes:

* [`assert_gt!(a, b);`](https://docs.rs/assertables/8.18.0/assertables/macro.assert_gt.html) `// panic during typical test`
* [`assert_gt_as_result!(a, b);`](https://docs.rs/assertables/8.18.0/assertables/macro.assert_gt_as_result.html) `// return Ok or Err`
* [`debug_assert_gt!(a, b);`](https://docs.rs/assertables/8.18.0/assertables/macro.debug_assert_gt.html) `// panic when in debug mode`

All assertables macros have forms for an optional message:

* [`assert_gt!(a, b);`](https://docs.rs/assertables/8.18.0/assertables/macro.assert_gt.html) `// automatic error message`
* [`assert_gt!(a, b, "your text");`](https://docs.rs/assertables/8.18.0/assertables/macro.assert_gt.html) `// custom error message`

Many assertables macros have forms for comparing left hand side (LHS) and right hand side (RHS) as the same type or as an arbitrary expression:

* [`assert_ok_eq!(a, b);`](https://docs.rs/assertables/8.18.0/assertables/macro.assert_ok_eq.html) `// Ok(…) = Ok(…)`
* [`assert_ok_eq_expr!(a, b);`](https://docs.rs/assertables/8.18.0/assertables/macro.assert_ok_eq_expr.html) `// Ok(…) = expression`


## Tracking

* Package: assertables-rust-crate
* Version: 8.18.0
* Created: 2021-03-30T15:47:49Z
* Updated: 2024-10-09T19:23:11Z
* License: MIT or Apache-2.0 or GPL-2.0 or GPL-3.0 or contact us for more
* Contact: Joel Parker Henderson (joel@sixarm.com)
