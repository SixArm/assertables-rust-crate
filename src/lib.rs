//! # Assertables: Rust crate of assert macros for testing
//!
//! The `assertables` Rust crate provides many assert macros to improve your
//! compile-time tests and run-time reliability.
//!
//! * Crate:
//!   [https://crates.io/crates/assertables](https://crates.io/crates/assertables)
//! * Docs: [https://docs.rs/assertables/](https://docs.rs/assertables/)
//! * Repo:
//!   [https://github.com/sixarm/assertables-rust-crate/](https://github.com/sixarm/assertables-rust-crate/)
//! * Contact:
//!   [joel@joelparkerhenderson.com](mailto:joel@joelparkerhenderson.com)
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
//! ```rust
//! use assertables::*;
//! # fn main() {
//! let s = "hello world";
//! assert_starts_with!(s, "hello");
//! assert_contains!(s, "o");
//! assert_len_eq!(s, "***********");
//! assert_all!(s.chars(), |c: char| c < 'x');
//! assert_any!(s.chars(), |c: char| c.is_whitespace());
//! # }
//!  ```
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
//! 2. Runtime savvy: all the assertables macros have runtime versions.
//! 3. Zero overhead: if you don't use a macro, then it's never compiled.
//!
//! To use the macros, add this to your `Cargo.toml` file:
//!
//! ```toml
//! [dev-dependencies]
//! assertables = "*"
//! ```
//!
//! ## Highlights
//!
//! For values:
//!
//! * [`assert_lt!(a, b)`](macro@crate::assert_lt) `// less than`
//! * [`assert_le!(a, b)`](macro@crate::assert_le) `// less than or equal to`
//! * [`assert_gt!(a, b)`](macro@crate::assert_gt) `// greater than`
//! * [`assert_ge!(a, b)`](macro@crate::assert_ge) `// greater than or equal to`
//!
//! For approximations:
//!
//! * [`assert_approx_eq!(a, b)`](macro@crate::assert_approx_eq)
//! * [`assert_approx_ne!(a, b)`](macro@crate::assert_approx_ne)
//!
//! For nearness:
//!
//! * [`assert_in_delta!(a, b, delta)`](macro@crate::assert_in_delta)
//! * [`assert_in_epsilon!(a, b, epsilon)`](macro@crate::assert_in_epsilon)
//! 
//! For groups:
//!
//! * [`assert_all!(group, predicate)`](macro@crate::assert_all)
//! * [`assert_any!(group, predicate)`](macro@crate::assert_any)
//!
//! ## Modules
//! 
//! There are many more macros that are organized in modules.
//!
//! For values:
//!
//! * [`assert_infix`](module@crate::assert_infix)
//! * [`assert_approx`](module@crate::assert_approx)
//!
//! For parts:
//!
//! * [`assert_starts_with`](module@crate::assert_starts_with)
//! * [`assert_ends_with`](module@crate::assert_ends_with)
//! 
//! For lengths:
//! 
//! * [`assert_len`](module@crate::assert_len)
//! * [`assert_is_empty`](module@crate::assert_is_empty)
//! 
//! For matching:
//!
//! * [`assert_contains`](module@crate::assert_contains)
//! * [`assert_is_match`](module@crate::assert_is_match)
//! 
//! For collections, such as arrays, vectors, lists, maps:
//!
//! * [`assert_iter`](module@crate::assert_iter) for iterator collections.
//! * [`assert_set`](module@crate::assert_set) for set collections.
//! * [`assert_bag`](module@crate::assert_bag) for bag collections.
//!
//! For Result Ok/Err:
//!
//! * [`assert_ok`](module@crate::assert_ok)
//! * [`assert_err`](module@crate::assert_err)
//! 
//! For Option Some/None:
//! 
//! * [`assert_some`](module@crate::assert_some)
//! * [`assert_none`](module@crate::assert_none)
//! 
//! For Poll Ready/Pending:
//! 
//! * [`assert_ready`](module@crate::assert_ready)
//! * [`assert_pending`](module@crate::assert_pending)
//!
//! For functions:
//!
//! * [`assert_fn`](module@crate::assert_fn) for functions in general.
//! * [`assert_fn_ok`](module@crate::assert_fn_ok) for functions that return Result::Ok.
//! * [`assert_fn_err`](module@crate::assert_fn_err) for functions that return Result::Err.
//!
//! For reading file systems and input/output streams:
//!
//! * [`assert_fs_read_to_string`](module@crate::assert_fs_read_to_string) for file system path contents.
//! * [`assert_io_read_to_string`](module@crate::assert_io_read_to_string) for input/output reader streams.
//!
//! For commands to capture stdout and stderr:
//!
//! * [`assert_command`](module@crate::assert_command) for commands and their stdout & stderr.
//! * [`assert_program_args`](module@crate::assert_program_args) for programs with args and their stdout & stderr.
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
//!
//! ## Tracking
//!
//! * Package: assertables-rust-crate
//! * Version: 8.16.0
//! * Created: 2021-03-30T15:47:49Z
//! * Updated: 2024-10-08T15:07:34Z
//! 
//! * License: MIT or Apache-2.0 or GPL-2.0 or GPL-3.0 or contact us for more
//! * Contact: Joel Parker Henderson (joel@sixarm.com)

// Assert truth
pub mod assert; // (in addition to what's provided by Rust `std`)

// Assert value comparison
pub mod assert_eq; // (in addition to what's provided by Rust `std`)
pub mod assert_ge;
pub mod assert_gt;
pub mod assert_le;
pub mod assert_lt;
pub mod assert_ne;

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
pub mod assert_is_empty;
pub mod assert_is_match;
pub mod assert_contains;
pub mod assert_starts_with;
pub mod assert_ends_with;
pub mod assert_len;

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
