//! Assertables: Rust crate of macros `assert`, `assume`, `assure`
//!
//! This `assertables` Rust crate provides macros `assert…!`, `assume…!`,
//! `assure…!`, all for runtime reliability checking, and all described below.
//! By SixArm.com.
//!
//! Crate:
//! [https://crates.io/crates/assertables](https://crates.io/crates/assertables)
//!
//! Docs:
//! [https://docs.rs/assertables/](https://docs.rs/assertables/)
//!
//! Repo:
//! [https://github.com/sixarm/assertables-rust-crate/](https://github.com/sixarm/assertables-rust-crate/)
//!
//!
//! ## Introduction
//!
//! This Rust crate provides macros for Rust runtime checking.
//!
//! Examples:
//!
//! * `assert_lt!(1, 2)` means check that 1 is less than 2, otherwise panic.
//!
//! * `assume_lt!(1, 2)` means check that 1 is less than 2, otherwise return error.
//!
//! * `assure_lt!(1, 2)` means check that 1 is less than 2, otherwise return false.
//!
//!
//! ### Assert
//!
//! Example to assert that x is less than y:
//!
//! ```rust
//! # #[macro_use] extern crate assertables;
//! # fn main() {
//! assert_lt!(1, 2);
//! //-> ()
//! # }
//! ```
//!
//! ```rust
//! # #[macro_use] extern crate assertables;
//! # use std::panic;
//! # fn main() {
//! # let result = panic::catch_unwind(|| {
//! assert_lt!(2, 1);
//! # });
//! # assert!(result.is_err());
//! # let s: String = result.unwrap_err().downcast::<String>().unwrap().to_string();
//! # assert_eq!(s, "assertion failed: `assert_lt(left, right)`\n  left: `2`\n right: `1`");
//! //-> panic!("assertion failed: `assert_lt(left, right)`\n  left: `2`\n right: `1`")
//! # }
//! ```
//!
//!
//! ### Assume
//!
//! Example to assume that x is less than y:
//!
//! ```rust
//! # #[macro_use] extern crate assertables;
//! # fn main() {
//! let x = assume_lt!(1, 2);
//! //-> Ok(true)
//! # }
//! ```
//!
//! ```rust
//! # #[macro_use] extern crate assertables;
//! # fn main() {
//! let x = assume_lt!(2, 1);
//! //-> Err("assumption failed: `assume_lt(left, right)`\n  left: `2`,\n right: `1`")
//! # }
//! ```
//!
//!
//! ### Assure
//!
//! Example to assure that x is less than y:
//!
//! ```rust
//! # #[macro_use] extern crate assertables;
//! # fn main() {
//! let x = assure_lt!(1, 2);
//! //-> Ok(true)
//! # }
//! ```
//!
//! ```rust
//! # #[macro_use] extern crate assertables;
//! # fn main() {
//! let x = assure_lt!(2, 1);
//! //-> Ok(false)
//! # }
//! ```
//!
//!
//! ## Macros
//!
//!
//! ### Macros for value checking
//!
//! Examples:
//!
//! ```rust
//! # #[macro_use] extern crate assertables;
//! # fn main() {
//! assert_lt!(1, 2); // check that 1 is less than 2
//! //-> Ok(true)
//! # }
//! ```
//!
//! ```rust
//! # #[macro_use] extern crate assertables;
//! # use std::panic;
//! # fn main() {
//! # let result = panic::catch_unwind(|| {
//! assert_lt!(2, 1);
//! # });
//! # assert!(result.is_err());
//! # let s: String = result.unwrap_err().downcast::<String>().unwrap().to_string();
//! # assert_eq!(s, "assertion failed: `assert_lt(left, right)`\n  left: `2`\n right: `1`");
//! //-> panic!("assertion failed: `assert_lt(left, right)`\n  left: `2`\n right: `1`")
//! # }
//! ```
//!
//!
//! ### Macros for function checking
//!
//! To compare function return values:
//!
//! ```rust
//! assert_fn_eq!(abs, 1, -1); // abs(1) == abs(-1)
//! ```
//!
//! To compare function `Result` `Ok()` values:
//!
//! ```rust
//! assert_fn_ok_eq!(::i32::from_str, "1", "1"); // ::i32::from_str("1").unwrap() == ::i32::from_str("1").unwrap()
//! ```
//!
//! Test a function `Result` `Err()` strings:
//!
//! ```rust
//! assert_fn_err_string_eq!(::i32::from_str, "foo", "goo"); // ::i32::from_str("foo").unwrap_err().to_string() == ::i32::from_str("goo").unwrap_err().to_string()
//! ```
//!
//!
//! ### Macros for set checking
//!
//! These macros help with comparison of set parameters, such as two arrays or
//! two vectors. where the item order does not matter, and the item count does
//! not matter.
//!
//! Examples:
//!
//! ```rust
//! # #[macro_use] extern crate assertables;
//! # fn main() {
//! assert_set_eq!([1, 2], [2, 1]);
//! //-> ()
//! # }
//! ```
//!
//! ```rust
//! # #[macro_use] extern crate assertables;
//! # use std::panic;
//! # fn main() {
//! # let result = panic::catch_unwind(|| {
//! assert_set_eq!([1, 2], [3, 4]);
//! # });
//! # assert!(result.is_err());
//! # let s: String = result.unwrap_err().downcast::<String>().unwrap().to_string();
//! # assert_eq!(s, "assertion failed: `assert_set_eq(left, right)`\n  left: `[1, 2]`\n right: `[3, 4]`");
//! //-> Err("assertion failed: `assert_set_eq(left, right)`\n  left: `[1, 2]`\n right: `[3, 4]`")
//! # }
//! ```
//!
//!
//! ### Macros for bag checking
//!
//! Thes macros help with comparison of bag parameters, such as comparison of
//! two arrays or two vectors, where the item order does not matter, and the
//! item count does matter.
//!
//! Examples:
//!
//! ```rust
//! # #[macro_use] extern crate assertables;
//! # fn main() {
//! assert_bag_eq!([1, 1], [1, 1]);
//! //-> ()
//! # }
//! ```
//!
//! ```rust
//! # #[macro_use] extern crate assertables;
//! # use std::panic;
//! # fn main() {
//! # let result = panic::catch_unwind(|| {
//! assert_bag_eq!([1, 1], [1, 1, 1]);
//! # });
//! # assert!(result.is_err());
//! # let s: String = result.unwrap_err().downcast::<String>().unwrap().to_string();
//! # assert_eq!(s, "assertion failed: `assert_bag_eq(left, right)`\n  left: `[1, 1]`\n right: `[1, 1, 1]`");
//! //-> Err("assertion failed: `assert_bag_eq(left, right)`\n  left: `[1, 1]`\n right: `[1, 1, 1]`")
//! # }
//! ```
//!
//!
//! ### Macros for IO-related checking
//!
//! These macros help with IO-related checking, such as comparison of files,
//! streams, etc. These macros return a `Result` with `Ok(true)` or
//! `Err(std::io::Error::new(std::io::ErrorKind::InvalidInput, message))`.
//!
//! Examples:
//!
//! ```rust
//! # #[macro_use] extern crate assertables;
//! # fn main() {
//! assert_io_lt!(1, 2);
//! //-> ()
//! # }
//! ```
//!
//! ```rust
//! # #[macro_use] extern crate assertables;
//! # use std::panic;
//! # fn main() {
//! # let result = panic::catch_unwind(|| {
//! assert_io_lt!(2, 1);
//! # });
//! # assert!(result.is_err());
//! # let s: String = result.unwrap_err().downcast::<String>().unwrap().to_string();
//! # assert_eq!(s, "assertion failed: `assert_io_lt(left, right)`\n  left: `2`\n right: `1`");
//! //-> panic!("assertion failed: `assert_io_lt(left, right)`\n  left: `2`\n right: `1`")
//! # }
//! ```
//!
//! The return is especially helpful for the macro `assume…` error:
//!
//! ```rust
//! # #[macro_use] extern crate assertables;
//! # fn main() {
//! let x = assume_io_lt!(2, 1);
//! //-> Err(
//! //       std::io::Error::new(
//! //           std::io::ErrorKind::InvalidInput,
//! //           "assumption failed: `assume_io_lt(left, right)`\n  left: `2`\n right: `1`")]
//! //       )
//! //   )
//! # }
//! ```
//!
//!
//! ## Extras
//!
//!
//! ### Custom error messages
//!
//! The macros have a second form where a custom error message can be provided.
//!
//!
//! ### Comparison abbreviations
//!
//! The comparison macros use abbreviations such as `eq` (equals), `ne` (not equals), `lt` (less than), `le` (less than or equal to), `gt` (greater than), `ge` (greater than or equals).

// Assert truth
pub mod assert; // condition (provided by Rust `std`)

// Assert value comparison
pub mod assert_eq; // equal (provided by Rust `std`)
pub mod assert_ne; // not equal (provided by Rust `std`)
pub mod assert_lt; // less than
pub mod assert_le; // less than or equal to
pub mod assert_gt; // greater than
pub mod assert_ge; // greater than or equal to

// Assert function output comparison
pub mod assert_fn_eq; // equal
pub mod assert_fn_ne; // not equal
pub mod assert_fn_lt; // less than
pub mod assert_fn_le; // less than or equal to
pub mod assert_fn_gt; // greater than
pub mod assert_fn_ge; // greater than or equal to

// Assert function ok() comparison
pub mod assert_fn_ok_eq; // equal
pub mod assert_fn_ok_ne; // not equal
pub mod assert_fn_ok_lt; // less than
pub mod assert_fn_ok_le; // less than or equal to
pub mod assert_fn_ok_gt; // greater than
pub mod assert_fn_ok_ge; // greater than or equal to

// Assert function err() comparison
pub mod assert_fn_err_string_eq; // equal
pub mod assert_fn_err_string_ne; // not equal
pub mod assert_fn_err_string_lt; // less than
pub mod assert_fn_err_string_le; // less than or equal to
pub mod assert_fn_err_string_gt; // greater than
pub mod assert_fn_err_string_ge; // greater than or equal to

// Assert iterator-related set-based comparison
pub mod assert_set_eq; // equal
pub mod assert_set_ne; // not equal

// Assert iterator-related bag-based comparison
pub mod assert_bag_eq; // equal
pub mod assert_bag_ne; // not equal

// Assert IO-related truth, which can return Err(std:io:Error(…))
//pub mod assert_io;

// Assert IO-related comparison, which can return Err(std:io:Error(…))
pub mod assert_io_eq; // equal
pub mod assert_io_ne; // not equal
pub mod assert_io_lt; // less than
pub mod assert_io_le; // less than or equal to
pub mod assert_io_gt; // greater than
pub mod assert_io_ge; // greater than or equal to

// Assume truth
pub mod assume; // condition

// Assume value comparison
pub mod assume_eq; // equal
pub mod assume_ne; // not equal
pub mod assume_lt; // less than
pub mod assume_le; // less than or equal to
pub mod assume_gt; // greater than
pub mod assume_ge; // greater than or equal to

// Assume function output comparison
pub mod assume_fn_eq; // equal
pub mod assume_fn_ne; // not equal
pub mod assume_fn_lt; // less than
pub mod assume_fn_le; // less than or equal to
pub mod assume_fn_gt; // greater than
pub mod assume_fn_ge; // greater than or equal to

// Assume function ok() comparison
pub mod assume_fn_ok_eq; // equal
pub mod assume_fn_ok_ne; // not equal
pub mod assume_fn_ok_lt; // less than
pub mod assume_fn_ok_le; // less than or equal to
pub mod assume_fn_ok_gt; // greater than
pub mod assume_fn_ok_ge; // greater than or equal to

// Assume function err().to_string() comparison
pub mod assume_fn_err_string_eq; // equal
pub mod assume_fn_err_string_ne; // not equal
pub mod assume_fn_err_string_lt; // less than
pub mod assume_fn_err_string_le; // less than or equal to
pub mod assume_fn_err_string_gt; // greater than
pub mod assume_fn_err_string_ge; // greater than or equal to

// Assume iterator-related set-based comparison
pub mod assume_set_eq; // equal
pub mod assume_set_ne; // not equal

// Assume iterator-related bag-based comparison
pub mod assume_bag_eq; // equal
pub mod assume_bag_ne; // not equal

// Assume IO-related truth, which can return Err(std:io:Error(…))
pub mod assume_io;

// Assume IO-related comparison, which can return Err(std:io:Error(…))
pub mod assume_io_eq; // equal
pub mod assume_io_ne; // not equal
pub mod assume_io_lt; // less than
pub mod assume_io_le; // less than or equal to
pub mod assume_io_gt; // greater than
pub mod assume_io_ge; // greater than or equal to

// Assure truth
pub mod assure; // condition

// Assure value comparison
pub mod assure_eq; // equal
pub mod assure_ne; // not equal
pub mod assure_lt; // less than
pub mod assure_le; // less than or equal to
pub mod assure_gt; // greater than
pub mod assure_ge; // greater than or equal to

// Assure function output comparison
pub mod assure_fn_eq; // equal
pub mod assure_fn_ne; // not equal
pub mod assure_fn_lt; // less than
pub mod assure_fn_le; // less than or equal to
pub mod assure_fn_gt; // greater than
pub mod assure_fn_ge; // greater than or equal to

// Assure function ok() comparison
pub mod assure_fn_ok_eq; // equal
pub mod assure_fn_ok_ne; // not equal
pub mod assure_fn_ok_lt; // less than
pub mod assure_fn_ok_le; // less than or equal to
pub mod assure_fn_ok_gt; // greater than
pub mod assure_fn_ok_ge; // greater than or equal to

// Assure function err().to_string() comparison
pub mod assure_fn_err_string_eq; // equal
pub mod assure_fn_err_string_ne; // not equal
pub mod assure_fn_err_string_lt; // less than
pub mod assure_fn_err_string_le; // less than or equal to
pub mod assure_fn_err_string_gt; // greater than
pub mod assure_fn_err_string_ge; // greater than or equal to

// Assure iterator-related set-based comparison
pub mod assure_set_eq; // equal
pub mod assure_set_ne; // not equal

// Assure iterator-related bag-based comparison
pub mod assure_bag_eq; // equal
pub mod assure_bag_ne; // not equal

// Assure IO-related truth, which can return Err(std:io:Error(…))
pub mod assure_io;

// Assure IO-related comparison, which can return Err(std:io:Error(…))
pub mod assure_io_eq; // equal
pub mod assure_io_ne; // not equal
pub mod assure_io_lt; // less than
pub mod assure_io_le; // less than or equal to
pub mod assure_io_gt; // greater than
pub mod assure_io_ge; // greater than or equal to
