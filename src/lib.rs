//! # Assertables: Rust crate of assert macros for testing
//!
//! The `assertables` Rust crate provides many assert macros to improve your
//! compile-time tests and run-time reliability.
//!
//! * Crate: [https://crates.io/crates/assertables](https://crates.io/crates/assertables)
//! * Docs: [https://docs.rs/assertables/](https://docs.rs/assertables/)
//! * Repo: [https://github.com/sixarm/assertables-rust-crate/](https://github.com/sixarm/assertables-rust-crate/)
//! * Contact: [joel@joelparkerhenderson.com](mailto:joel@joelparkerhenderson.com)
//!
//! ## Introduction
//!
//! The Rust programming language provides a few built-in assert macros to test
//! code:
//!
//! The Rust programming language provides assert macros to test code:
//!
//! * [`assert!(a)`](https://doc.rust-lang.org/std/macro.assert.html) `// a is true`
//! * [`assert_eq!(a, b)`](https://doc.rust-lang.org/std/macro.assert_eq.html) `// a is equal to b`
//! * [`assert_ne!(a, b)`](https://doc.rust-lang.org/std/macro.assert_ne.html) `// a is not equal to b`
//!
//! The assertables crate provides many more, to help you work with numbers,
//! strings, results, options, polls, iterators, files, streams, commands, and more. 
//!
//! Examples:
//!
//! ``rust
//! use assertables::*;
//! # fn main() {
//! let s = "hello world";
//! assert_matches!(s, "hello world");
//! assert_starts_with!(s, "hello");
//! assert_contains!(s, "o");
//! assert_all!(s.chars(), |c: char| c < 'x');
//! # }
//!  ``
//! 
//! To use the macros, add this to your `Cargo.toml` file:
//!
//! ``toml
//! [dev-dependencies]
//! assertables = "*"
//! ``
//! 
//! Top benefits:
//!
//! 1. You can write better tests to improve reliability and maintainability.
//! 2. You can handle more corner cases without needing to write custom code.
//! 3. You can troubleshoot faster because error messages show specifics.
//!
//! Top features:
//!
//! 1. Easy to use: everything is well-documented with runnable examples.
//! 2. Zero overhead: if you don't use a macro, then it's never compiled.
//! 3. Runtime options: all the assertables macros have runtime versions.
//!
//! Top comparable crates:
//!    [`assert_matches`](https://crates.io/crates/assert_matches),
//!    [`assert_approx_eq`](https://crates.io/crates/assert_approx_eq),
//!    [`more_asserts`](https://crates.io/crates/more_asserts),
//!    [`cool_asserts`](https://crates.io/crates/cool_asserts),
//!    [`assert2`](https://crates.io/crates/assert2),
//!    [`claims`](https://crates.io/crates/claims),
//!    [`static_assertions`](https://crates.io/crates/static_assertions).
//!
//! ## Highlights
//!
//! Values:
//! 
//! * [`assert_eq!(a, b);`](module@crate::assert_eq) `// equal to`
//! * [`assert_ne!(a, b);`](module@crate::assert_ne) `// not equal to`
//! * [`assert_lt!(a, b);`](module@crate::assert_lt) `// less than`
//! * [`assert_le!(a, b);`](module@crate::assert_le) `// less than or equal to`
//! * [`assert_gt!(a, b);`](module@crate::assert_gt) `// greater than`
//! * [`assert_ge!(a, b);`](module@crate::assert_ge) `// greater than or equal to`
//! 
//! Approximations:
//! 
//! * [`assert_approx_eq!(a, b);`](module@crate::assert_approx) `// |a-b| ≤ 1e-6`
//! * [`assert_in_delta!(a, b, delta);`](module@crate::assert_in_delta) `// |a-b| ≤ delta`
//! * [`assert_in_epsilon!(a, b, epsilon);`](module@crate::assert_in_epsilon) `// |a-b| ≤ epsilon * min(a,b)`
//! 
//! Groups for iterators, chars, etc.:
//! 
//! * [`assert_all!(group, predicate);`](module@crate::assert_all) `// group.all(predicate)`
//! * [`assert_any!(group, predicate);`](module@crate::assert_any) `// group.any(predicate)`
//! 
//! Infix for order operators, logic operators, etc.:
//! 
//! * [`assert_infix!(a == b);`](module@crate::assert_infix) `// order: == != < <= > >=`
//! * [`assert_infix!(a && b);`](module@crate::assert_infix) `// logic: && || ^ & |`
//! 
//! Parts for strings, vectors, etc.:
//! 
//! * [`assert_starts_with!(whole, part);`](module@crate::assert_starts_with) `// whole.starts_with(part)`
//! * [`assert_ends_with!(whole, part);`](module@crate::assert_ends_with) `// whole.ends_with(part)`
//! 
//! Lengths and counts for strings, vectors, iterators, etc.:
//! 
//! * [`assert_len!(item);`](module@crate::assert_len) `// item.len()`
//! * [`assert_count!(item);`](module@crate::assert_len) `// item.count()`
//! * [`assert_is_empty!(item);`](module@crate::assert_is_empty) `// item.is_empty()`
//! 
//! Matching for strings, regex, etc.:
//! 
//! * [`assert_matches!(expression, pattern);`](module@crate::assert_matches) `// matches!(expression, pattern)`
//! * [`assert_is_match!(matcher, matchee);`](module@crate::assert_is_match) `// matcher.is_match(matchee)`
//! * [`assert_contains!(container, containee);`](module@crate::assert_contains) `// container.contains(containee)`
//! 
//! Collections for arrays, vectors, iterators, sets, maps:
//! 
//! * [`assert_iter_eq!(arr1, arr2);`](module@crate::assert_iter) `// eq ne etc.`
//! * [`assert_set_eq!(vec1, vec2);`](module@crate::assert_set) `// eq ne etc.`
//! * [`assert_bag_eq!(map1, map2);`](module@crate::assert_bag) `// eq ne etc.`
//! 
//! Result Ok/Err:
//! 
//! * [`assert_ok!(result);`](module@crate::assert_ok) `// eq ne etc.`
//! * [`assert_err!(result);`](module@crate::assert_err) `// eq ne etc.`
//!   
//! Option Some/None:
//! 
//! * [`assert_some!(option);`](module@crate::assert_some) `// eq ne etc.`
//! * [`assert_none!(option);`](module@crate::assert_none)
//! 
//! Poll Ready/Pending:
//! 
//! * [`assert_ready!(poll);`](module@crate::assert_ready) `// eq ne etc.`
//! * [`assert_pending!(poll);`](module@crate::assert_pending)
//! 
//! Read file system paths and input/output streams:
//! 
//! * [`assert_fs_read_to_string_eq!(path1, path2);`](module@crate::assert_fs_read_to_string) `// eq ne etc.`
//! * [`assert_io_read_to_string_eq!(stream1, stream2);`](module@crate::assert_io_read_to_string) `// eq ne etc.`
//! 
//! Run commands and programs then assert on stdout or stderr:
//! 
//! * [`assert_command_stdout_eq!(command1, command2);`](module@crate::assert_command) `// eq ne etc.`
//! * [`assert_program_args_stdout_eq!(program1, args1, program2, args2);`](module@crate::assert_program_args) `// eq ne etc.`
//! 
//! Function comparisons, which are especially good for refactoring:
//! 
//! * [`assert_fn_eq!(fn1, fn2);`](module@crate::assert_fn) `// functions that return values`
//! * [`assert_fn_ok_eq!(fn1, fn2);`](module@crate::assert_fn_ok) `// functions that return Ok`
//! * [`assert_fn_err_eq!(fn1, fn2);`](module@crate::assert_fn_err) `// functions that return Err`
//! 
//! ## Forms
//!
//! All assertables macros have forms for an optional message:
//!
//! * [`assert_gt!(a, b)`](macro@crate::assert_gt) `// automatic message`
//! * [`assert_gt!(a, b, "Your text")`](macro@crate::assert_gt) `// custom message`
//!
//! All assertables macros have forms for different outcomes:
//!
//! * [`assert_gt!(a, b)`](macro@crate::assert_gt) `// panic during typical test`
//! * [`assert_gt_as_result!(a, b)`](macro@crate::assert_gt_as_result) `// return Ok or Err`
//! * [`debug_assert_gt!(a, b)`](macro@crate::debug_assert_gt) `// panic when in debug mode`
//!
//! Many assertables macros have forms for comparing left hand side (LHS) and
//! right hand side (RHS) as the same type or as an arbitrary expression:
//!
//! * [`assert_ok_eq!(a, b)`](macro@crate::assert_ok_eq) `// Ok(…) = Ok(…)`
//! * [`assert_ok_eq_expr!(a, b)`](macro@crate::assert_ok_eq_expr) `// Ok(…) = expression`
//!
//! ## Tracking
//!
//! * Package: assertables-rust-crate
//! * Version: 8.18.0
//! * Created: 2021-03-30T15:47:49Z
//! * Updated: 2024-10-09T19:23:11Z
//! * License: MIT or Apache-2.0 or GPL-2.0 or GPL-3.0 or contact us for more
//! * Contact: Joel Parker Henderson (joel@sixarm.com)

// Assert truth
pub mod assert; // (in addition to what's provided by Rust `std`)

// Assert value comparison
pub mod assert_eq; // (in addition to what's provided by Rust `std`)
pub mod assert_ne; // (in addition to what's provided by Rust `std`)
pub mod assert_lt;
pub mod assert_le;
pub mod assert_gt;
pub mod assert_ge;

// Assert value nearness
pub mod assert_approx;
pub mod assert_in_delta;
pub mod assert_in_epsilon;

// Assert all/any
pub mod assert_all;
pub mod assert_any;

// Infix
pub mod assert_infix;

// Matching
pub mod assert_is_match;
pub mod assert_matches;
pub mod assert_contains;
pub mod assert_starts_with;
pub mod assert_ends_with;
pub mod assert_len;
pub mod assert_count;
pub mod assert_is_empty;

// For Result Ok/Err
pub mod assert_ok;
pub mod assert_err;
pub mod assert_result; // Deprecated

// For Option Some/None
pub mod assert_some;
pub mod assert_none;
pub mod assert_option; // Deprecated

// For Poll Ready/Pending
pub mod assert_ready;
pub mod assert_pending;
pub mod assert_poll; // Deprecated

// For collections
pub mod assert_iter;
pub mod assert_set;
pub mod assert_bag;

// For functions
pub mod assert_fn;
pub mod assert_fn_ok;
pub mod assert_fn_err;

// For reading
pub mod assert_fs_read_to_string;
pub mod assert_io_read_to_string;

// For externals
pub mod assert_command;
pub mod assert_program_args;
