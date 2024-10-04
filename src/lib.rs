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
//! The Rust programming language provides a few built-in assert macros to test code:
//!
//! The Rust programming language provides assert macros to test code:
//!
//! * [`assert!()`](https://doc.rust-lang.org/std/macro.assert.html)
//! * [`assert_eq!(a, b)`](https://doc.rust-lang.org/std/macro.assert_eq.html) `// equal`
//! * [`assert_ne!(a, b)`](https://doc.rust-lang.org/std/macro.assert_ne.html) `// not equal`
//!
//! The assertables crate provides many more, so you can write smarter tests.
//!
//! Examples:
//!
//! * [`assert_lt!(1, 2)`](macro@crate::assert_lt) `// compare values using less than`
//! * [`assert_approx_eq!(1.0000001, 1.0000002)`](macro@crate::assert_approx_eq) `// compare floats`
//! * [`assert_len_eq!("hello", "world")`](macro@crate::assert_len_eq) `// compare lengths`
//! * [`assert_starts_with!("hello world", "hello")`](macro@crate::assert_starts_with) `// compare strings`
//! * [`assert_fs_read_to_string_eq!("a.txt", "b.txt")`](macro@crate::assert_fs_read_to_string_eq) `// compare files`
//!
//! Top 3 benefits:
//!
//! 1. You can write better tests to improve reliability and maintainability.
//! 2. You can handle more corner cases without needing to write custom code.
//! 3. You can troubleshoot faster because error messages show specifics.
//!
//! Top 3 features:
//!
//! 1. Easy to use: everything is well-documented with runnable examples.
//! 2. Runtime savvy: all the assertables macros have runtime versions.
//! 3. Zero overhead: if you don't use a macro, then it's never compiled.
//!
//! ## Install
//! 
//! To use this crate, add it to your `Cargo.toml` file:
//!
//! ```toml
//! [dev-dependencies]
//! assertables = "8.14.0"
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
//! For approximation:
//!
//! * [`assert_approx_eq!(a, b)`](macro@crate::assert_approx_eq)
//! * [`assert_approx_ne!(a, b)`](macro@crate::assert_approx_ne)
//!
//! For strings:
//!
//! * [`assert_starts_with!(a, b)`](macro@crate::assert_starts_with)
//! * [`assert_ends_with!(a, b)`](macro@crate::assert_ends_with)
//!
//! For lengths:
//! 
//! * [`assert_len_eq!(a, b)`](macro@crate::assert_len_eq)
//! * [`assert_is_empty!(a, b)`](macro@crate::assert_is_empty)
//! 
//! For matching:
//!
//! * [`assert_contains!(a, b)`](macro@crate::assert_contains)
//! * [`assert_is_match!(a, b)`](macro@crate::assert_is_match)
//! 
//! For infix operators:
//!
//! * [`assert_infix!(a == b)`](macro@crate::assert_infix)
//! * [`assert_infix!(a && b)`](macro@crate::assert_infix)
//!
//! For nearness:
//!
//! * [`assert_in_delta!(a, b, delta)`](macro@crate::assert_in_delta)
//! * [`assert_in_epsilon!(a, b, epsilon)`](macro@crate::assert_in_epsilon)
//!
//! For Result Ok/Err:
//!
//! * [`assert_ok!(a)`](macro@crate::assert_ok)
//! * [`assert_err!(a)`](macro@crate::assert_err)
//!
//! For Option Some/None:
//!
//! * [`assert_some!(a)`](macro@crate::assert_some)
//! * [`assert_none!(a)`](macro@crate::assert_none)
//!
//! For Poll Ready/Pending:
//!
//! * [`assert_ready!(a)`](macro@crate::assert_ready)
//! * [`assert_pending!(a)`](macro@crate::assert_pending)
//!
//! For collections such as arrays, vectors, iterators, sets, bags:
//!
//! * [`assert_iter_eq!(collection1, collection2)`](macro@crate::assert_iter_eq)
//! * [`assert_set_eq!(collection1, collection2)`](macro@crate::assert_set_eq)
//! * [`assert_bag_eq!(collection1, collection2)`](macro@crate::assert_bag_eq)
//!
//! For file system paths and input/output readers:
//!
//! * [`assert_fs_read_to_string_eq!(path1, path2)`](macro@crate::assert_fs_read_to_string_eq)
//! * [`assert_io_read_to_string_eq!(reader1, reader2)`](macro@crate::assert_io_read_to_string_eq)
//!
//! For command capture of standard output and standard error:
//!
//! * [`assert_command_stdout_eq!(command1, command2)`](macro@crate::assert_command_stdout_eq)
//! * [`assert_program_args_stdout_eq!(program1, args1, program2, args2)`](macro@crate::assert_program_args_stdout_eq);
//!
//! ## Modules
//! 
//! There are many more macros that are organized in modules.
//!
//! Modules for values:
//!
//! * [`assert_infix`](module@crate::assert_infix)
//! * [`assert_approx`](module@crate::assert_approx)
//!
//! Modules for strings:
//!
//! * [`assert_starts_with`](module@crate::assert_starts_with)
//! * [`assert_ends_with`](module@crate::assert_ends_with)
//! 
//! Modules for lengths:
//! 
//! * [`assert_len`](module@crate::assert_len)
//! * [`assert_is_empty`](module@crate::assert_is_empty)
//! 
//! Modules for matching:
//!
//! * [`assert_contains`](module@crate::assert_contains)
//! * [`assert_is_match`](module@crate::assert_is_match)
//! 
//! Modules for collections, such as arrays, vectors, lists, maps:
//!
//! * [`assert_iter`](module@crate::assert_iter) for iterator collections.
//! * [`assert_set`](module@crate::assert_set) for set collections.
//! * [`assert_bag`](module@crate::assert_bag) for bag collections.
//!
//! Modules for Result Ok/Err:
//!
//! * [`assert_ok`](module@crate::assert_ok)
//! * [`assert_err`](module@crate::assert_err)
//! 
//! Modules for Option Some/None:
//! 
//! * [`assert_some`](module@crate::assert_some)
//! * [`assert_none`](module@crate::assert_none)
//! 
//! Modules for Poll Ready/Pending:
//! 
//! * [`assert_ready`](module@crate::assert_ready)
//! * [`assert_pending`](module@crate::assert_pending)
//!
//! Modules for functions:
//!
//! * [`assert_fn`](module@crate::assert_fn) for functions in general.
//! * [`assert_fn_ok`](module@crate::assert_fn_ok) for functions that return Result::Ok.
//! * [`assert_fn_err`](module@crate::assert_fn_err) for functions that return Result::Err.
//!
//! Modules for readers:
//!
//! * [`assert_fs_read_to_string`](module@crate::assert_fs_read_to_string) for file system path contents.
//! * [`assert_io_read_to_string`](module@crate::assert_io_read_to_string) for input/output reader streams.
//!
//! Modules for external calls:
//!
//! * [`assert_command`](module@crate::assert_command) for commands and their stdout & stderr.
//! * [`assert_program_args`](module@crate::assert_program_args) for programs with args and their stdout & stderr.
//!
//! ## Forms
//!
//! All assertables macros have forms for different outcomes:
//!
//! * [`assert_gt!(a, b)`](https://docs.rs/assertables/8.14.0/assertables/macro.assert_gt.html) `// panic during typical test`
//! * [`assert_gt_as_result!(a, b)`](https://docs.rs/assertables/8.14.0/assertables/macro.assert_gt_as_result.html) `// return Ok or Err`
//! * [`debug_assert_gt!(a, b)`](https://docs.rs/assertables/8.14.0/assertables/macro.debug_assert_gt.html) `// panic when in debug mode`
//!
//! All assertables macros have forms for an optional message:
//!
//! * [`assert_gt!(a, b)`](https://docs.rs/assertables/8.14.0/assertables/macro.assert_gt.html) `// automatic message`
//! * [`assert_gt!(a, b, "Your text")`](https://docs.rs/assertables/8.14.0/assertables/macro.assert_gt.html) `// custom message`
//!
//! Many assertables macros have forms for comparing left hand side (LHS) and right hand side (RHS) as the same type or as an expression:
//!
//! * [`assert_ok_eq!(a, b)`](https://docs.rs/assertables/8.14.0/assertables/macro.assert_ok_eq.html) `// Ok(…) = Ok(…)`
//! * [`assert_ok_eq_expr!(a, b)`](https://docs.rs/assertables/8.14.0/assertables/macro.assert_ok_eq_expr.html) `// Ok(…) = expression`
//!
//!
//! ## Tracking
//!
//! * Package: assertables-rust-crate
//! * Version: 8.14.0
//! * Created: 2021-03-30T15:47:49Z
//! * Updated: 2024-10-04T23:46:38Z
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

// Experimental - work in progress - unsupported
pub mod assert_infix;