//! Assertables: Rust crate of macros for assert and assertable
//!
//! This `assertables` Rust crate provides macros for `assert…!` and
//! `assertable…!`, which are useful for testing and also for runtime
//! reliability checking. By SixArm.com.
//!
//! Crate:
//! [https://crates.io/crates/assertables](https://crates.io/crates/assertables)
//!
//! Docs: [https://docs.rs/assertables/](https://docs.rs/assertables/)
//!
//! Repo:
//! [https://github.com/sixarm/assertables-rust-crate/](https://github.com/sixarm/assertables-rust-crate/)
//!
//!
//! ### assert and assertable
//!
//! These macros have two styles:
//!
//!   * `assert` macros return `()` or `panic!(…)`.
//!
//!   * `assertable` macros return `Ok(())` or `Err(…)`
//!
//! Example:
//!
//! ```rust
//! # #[macro_use] extern crate assertables;
//! # use std::panic;
//! # fn main() {
//! assert_eq!(1, 1);
//! //-> ()
//!
//! # let result = panic::catch_unwind(|| {
//! assert_eq!(1, 2);
//! //-> panic!("…")
//! // assertion failed: `(left == right)`
//! //   left: `1`,
//! //  right: `2`
//! # });
//! # let actual = result.unwrap_err().downcast::<String>().unwrap().to_string();
//! # let expect = "assertion failed: `(left == right)`\n  left: `1`,\n right: `2`";
//! # assert_eq!(actual, expect);
//!
//! let x = assertable_eq!(1, 1); 
//! //-> Ok(())
//! # assert_eq!(x.unwrap(), ());
//!
//! let x = assertable_eq!(1, 2); 
//! //-> Err("…")
//! // assertable failed: `assertable_eq!(left, right)`
//! //   left: `1`,
//! //  right: `2`
//! # assert_eq!(x.unwrap_err(), "assertable failed: `assertable_eq!(left, right)`\n  left: `1`,\n right: `2`");
//! # }
//! ```
//!
//! These two styles are useful because the `assert` macros favor compile-time
//! tests and diagnostics, whereas the `assertable` macros favor run-time
//! reliability and tracing.
//!
//!
//! ## assert_xx
//!
//! Compare values.
//!
//! * `assert_eq!(a, b)`: a == b
//!
//! * `assert_ne!(a, b)`: a !=b
//!
//! * `assert_lt!(a, b)`: a < b
//!
//! * `assert_le!(a, b)`: a <= b
//!
//! * `assert_gt!(a, b)`: a > b
//!
//! * `assert_ge!(a, b)`: a >= b
//!
//! Examples:
//!
//! ```rust
//! # #[macro_use] extern crate assertables;
//! # use std::panic;
//! # fn main() {
//! assert_eq!(1, 1);
//! //-> ()
//!
//! # let result = panic::catch_unwind(|| {
//! assert_eq!(1, 2);
//! //-> panic!("…")
//! // assertion failed: `(left == right)`
//! //   left: `1`
//! // right: `2`
//! # });
//! # let actual = result.unwrap_err().downcast::<String>().unwrap().to_string();
//! # let expect = "assertion failed: `(left == right)`\n  left: `1`,\n right: `2`";
//! # assert_eq!(actual, expect);
//! # }
//! ```
//!
//! Examples:
//!
//! ```rust
//! # #[macro_use] extern crate assertables;
//! # use std::panic;
//! # fn main() {
//! assert_lt!(1, 2); // check that 1 is less than 2
//! //-> ()
//!
//! # let result = panic::catch_unwind(|| {
//! assert_lt!(2, 1);
//! //-> panic!("…")
//! // assertion failed: `assert_lt!(left, right)`
//! //   left: `2`,
//! //  right: `1`
//! # });
//! # let actual = result.unwrap_err().downcast::<String>().unwrap().to_string();
//! # let expect = "assertion failed: `assert_lt!(left, right)`\n  left: `2`,\n right: `1`";
//! # assert_eq!(actual, expect);
//! # }
//! ```
//!
//!
//! ### Macros for function checking
//!
//! Compare function return values:
//!
//! ```rust
//! # #[macro_use] extern crate assertables;
//! # fn main() {
//! assert_fn_eq!(i32::abs, 1, -1); // abs(1) == abs(-1)
//! //-> ()
//! # }
//! ```
//!
//! Compare function result Ok(…) values:
//!
//! ```rust
//! # #[macro_use] extern crate assertables;
//! use std::str::FromStr;
//!
//! # fn main() {
//! assert_fn_ok_eq!(i32::from_str, "1", "1"); // i32::from_str("1").unwrap() == i32::from_str("1").unwrap()
//! //-> Ok()
//! # }
//! ```
//!
//! Compare function result Err(…) strings:
//!
//! ```rust
//! # #[macro_use] extern crate assertables;
//! use std::str::FromStr;
//!
//! # fn main() {
//! assert_fn_err_string_eq!(i32::from_str, "foo", "goo"); // i32::from_str("foo").unwrap_err().to_string() == i32::from_str("goo").unwrap_err().to_string()
//! //-> ()
//! # }
//! ```
//!
//! Two functions that are our favorites to use in our tests:
//!
//!   * `assert_fn_ok_eq!(i32::from_str, str1, str2); // compare parsed numbers`
//!
//!   * `assert_fn_ok_eq!(::std::fs::read_to_string, file1, file2); // compare
//!     file text`
//!
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
//! # use std::panic;
//! # fn main() {
//! assert_set_eq!([1, 2], [2, 1]);
//! //-> ()
//!
//! # let result = panic::catch_unwind(|| {
//! assert_set_eq!([1, 2], [3, 4]);
//! # });
//! # let actual = result.unwrap_err().downcast::<String>().unwrap().to_string();
//! # let expect = "assertion failed: `assert_set_eq!(left, right)`\n  left: `[1, 2]`,\n right: `[3, 4]`";
//! # assert_eq!(actual, expect); 
//! //-> panic!("assertion failed: `assert_set_eq!(left, right)`\n  left: `[1, 2]`,\n right: `[3, 4]`");
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
//! # use std::panic;
//! # fn main() {
//! assert_bag_eq!([1, 1], [1, 1]);
//! //-> ()
//!
//! # let result = panic::catch_unwind(|| {
//! assert_bag_eq!([1, 1], [1, 1, 1]);
//! //-> panic!("…")
//! // assertion failed: `assert_bag_eq!(left, right)`
//! //   left: `[1, 1]`,
//! //  right: `[1, 1, 1]`
//! # });
//! # let actual = result.unwrap_err().downcast::<String>().unwrap().to_string();
//! # let expect = "assertion failed: `assert_bag_eq!(left, right)`\n  left: `[1, 1]`,\n right: `[1, 1, 1]`";
//! # assert_eq!(actual, expect);
//! # }
//! ```
//!
//!
//! ### Macros for IO-related checking
//!
//! These macros help with IO-related checking, such as comparison of files,
//! streams, etc.
//!
//! Examples:
//!
//! ```rust
//! # #[macro_use] extern crate assertables;
//! # use std::panic;
//! # fn main() {
//! assert_io_lt!(1, 2);
//! //-> ()
//!
//! # let result = panic::catch_unwind(|| {
//! assert_io_lt!(2, 1);
//! //-> panic!("…")
//! // assertion failed: `assert_io_lt!(left, right)`
//! //   left: `2`,
//! //  right: `1`
//! # });
//! # let actual = result.unwrap_err().downcast::<String>().unwrap().to_string();
//! # let expect = "assertion failed: `assert_io_lt!(left, right)`\n  left: `2`,\n right: `1`";
//! # assert_eq!(actual, expect);
//! # }
//! ```
//!
//! The return is especially helpful for the macro `assure…` error:
//!
//! ```rust
//! # #[macro_use] extern crate assertables;
//! # fn main() {
//! let x = assertable_io_lt!(2, 1);
//! //-> Err(
//! //       std::io::Error::new(
//! //           std::io::ErrorKind::InvalidInput,
//! //           "assertable failed: `assertable_io_lt!(left, right)`\n  left: `2`,\n right: `1`")]
//! //       )
//! //   )
//! # }
//! ```
//!
//!
//! ### Macros for read_to_string()
//!
//! These macros help with standard input/output read checking, such as
//! comparison of files, streams, etc.
//!
//! Examples:
//!
//! ```rust
//! # #[macro_use] extern crate assertables;
//! # use std::panic;
//! use std::io::Read;
//!
//! # fn main() {
//! let mut a = "a".as_bytes();
//! let mut b = "b".as_bytes();
//! assert_read_to_string_lt!(a, b);
//! //-> ()
//!
//! # let result = panic::catch_unwind(|| {
//! let mut a = "a".as_bytes();
//! let mut b = "b".as_bytes();
//! assert_read_to_string_lt!(b, a);
//! //-> panic!("…")
//! // assertion failed: `assert_read_to_string_lt!(left, right)`
//! //   left: `\"b\"`,
//! //  right: `\"a\"`
//! # });
//! # let actual = result.unwrap_err().downcast::<String>().unwrap().to_string();
//! # let expect = "assertion failed: `assert_read_to_string_lt!(left, right)`\n  left: `\"b\"`,\n right: `\"a\"`";
//! # assert_eq!(actual, expect);
//! # }
//! ```
//!
//! The return is especially helpful for the macro `assure…` error:
//!
//! ```rust
//! # #[macro_use] extern crate assertables;
//! use std::io::Read;
//!
//! # fn main() {
//! let mut a = "a".as_bytes();
//! let mut b = "b".as_bytes();
//! let x = assertable_read_to_string_lt!(b, a);
//! //-> Err(
//! //       std::io::Error::new(
//! //           std::io::ErrorKind::InvalidInput,
//! //           "assertable failed: `assertable_read_to_string_lt!(left, right)`\n  left: `b`,\n right: `a`")]
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
//! The comparison macros use abbreviations such as `eq` (equals), `ne` (not
//! equals), `lt` (less than), `le` (less than or equal to), `gt` (greater
//! than), `ge` (greater than or equals).

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

// Assert std::io::read comparison
pub mod assert_read_to_string_eq; // equal
pub mod assert_read_to_string_ne; // not equal
pub mod assert_read_to_string_lt; // less than
pub mod assert_read_to_string_le; // less than or equal to
pub mod assert_read_to_string_gt; // greater than
pub mod assert_read_to_string_ge; // greater than or equal to

// Assertable truth
pub mod assertable; // condition

// Assertable value comparison
pub mod assertable_eq; // equal
pub mod assertable_ne; // not equal
pub mod assertable_lt; // less than
pub mod assertable_le; // less than or equal to
pub mod assertable_gt; // greater than
pub mod assertable_ge; // greater than or equal to

// Assertable function output comparison
pub mod assertable_fn_eq; // equal
pub mod assertable_fn_ne; // not equal
pub mod assertable_fn_lt; // less than
pub mod assertable_fn_le; // less than or equal to
pub mod assertable_fn_gt; // greater than
pub mod assertable_fn_ge; // greater than or equal to

// Assertable function ok() comparison
pub mod assertable_fn_ok_eq; // equal
pub mod assertable_fn_ok_ne; // not equal
pub mod assertable_fn_ok_lt; // less than
pub mod assertable_fn_ok_le; // less than or equal to
pub mod assertable_fn_ok_gt; // greater than
pub mod assertable_fn_ok_ge; // greater than or equal to

// Assertable function err().to_string() comparison
pub mod assertable_fn_err_string_eq; // equal
pub mod assertable_fn_err_string_ne; // not equal
pub mod assertable_fn_err_string_lt; // less than
pub mod assertable_fn_err_string_le; // less than or equal to
pub mod assertable_fn_err_string_gt; // greater than
pub mod assertable_fn_err_string_ge; // greater than or equal to

// Assertable iterator-related set-based comparison
pub mod assertable_set_eq; // equal
pub mod assertable_set_ne; // not equal

// Assertable iterator-related bag-based comparison
pub mod assertable_bag_eq; // equal
pub mod assertable_bag_ne; // not equal

// Assertable IO-related truth, which can return Err(std:io:Error(…))
pub mod assertable_io;

// Assertable IO-related comparison, which can return Err(std:io:Error(…))
pub mod assertable_io_eq; // equal
pub mod assertable_io_ne; // not equal
pub mod assertable_io_lt; // less than
pub mod assertable_io_le; // less than or equal to
pub mod assertable_io_gt; // greater than
pub mod assertable_io_ge; // greater than or equal to

// Assertable std::io::read comparison
pub mod assertable_read_to_string_eq; // equal
pub mod assertable_read_to_string_ne; // not equal
pub mod assertable_read_to_string_lt; // less than
pub mod assertable_read_to_string_le; // less than or equal to
pub mod assertable_read_to_string_gt; // greater than
pub mod assertable_read_to_string_ge; // greater than or equal to
