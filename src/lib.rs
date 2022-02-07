//! Assertables: Rust crate of macros "assert" and "assertable"
//!
//! This `assertables` Rust crate provides macros for `assert…!` and
//! `assertable…!`, which are useful for testing and also for runtime
//! reliability checking. By SixArm.com.
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
//! ## assert & assertable
//!
//! These macros have two styles:
//!
//!   * `assert` macros return `()` or `panic!(…)`.
//!
//!   * `assertable` macros return `Ok(())` or `Err(…)`.
//!
//! Examples of "assert less than" and "assertable less than":
//!
//! ```rust
//! # #[macro_use] extern crate assertables;
//! # use std::panic;
//! # fn main() {
//! assert_lt!(1, 2);
//! //-> ()
//!
//! # let result = panic::catch_unwind(|| {
//! assert_lt!(2, 1);
//! //-> panic!
//! // assertion failed: `assert_lt!(left, right)`
//! //   left: `2`,
//! //  right: `1`
//! # });
//! # let actual = result.unwrap_err().downcast::<String>().unwrap().to_string();
//! # let expect = "assertion failed: `assert_lt!(left, right)`\n  left: `2`,\n right: `1`";
//! # assert_eq!(actual, expect);
//!
//! let x = assertable_lt!(1, 2); 
//! //-> Ok(())
//! # assert_eq!(x.unwrap(), ());
//!
//! let x = assertable_eq!(2, 1); 
//! //-> Err("…")
//! // assertable failed: `assertable_lt!(left, right)`
//! //   left: `2`,
//! //  right: `1`
//! # assert_eq!(x.unwrap_err(), "assertable failed: `assertable_eq!(left, right)`\n  left: `2`,\n right: `1`");
//! # }
//! ```
//!
//! These two styles are useful because:
//! 
//! * `assert` macros favor compile-time tests and diagnostics.
//! 
//! * `assertable` macros favor run-time tracing and recoveries.
//!
//! The macros use abbreviations: `eq` (equals), `ne` (not equals), 
//! `lt` (less than), `le` (less than or equal to), `gt` (greater than), 
//! `ge` (greater than or equals).
//! 
//! The macros have a second form where a custom error message can be provided.
//!
//! 
//! ## assert_xx for values
//!
//! Compare values.
//!
//! * `assert_eq!(a, b)` ~ a == b
//!
//! * `assert_ne!(a, b)` ~ a != b
//!
//! * `assert_lt!(a, b)` ~ a < b
//!
//! * `assert_le!(a, b)` ~ a <= b
//!
//! * `assert_gt!(a, b)` ~ a > b
//!
//! * `assert_ge!(a, b)` ~ a >= b
//!
//! Examples:
//!
//! ```rust
//! # #[macro_use] extern crate assertables;
//! # use std::panic;
//! # fn main() {
//! assert_lt!(1, 2);
//! //-> ()
//!
//! # let result = panic::catch_unwind(|| {
//! assert_lt!(2, 1);
//! //-> panic!
//! // assertion failed: `assert_lt!(left, right)`
//! //   left: `2`
//! //  right: `1`
//! # });
//! # let actual = result.unwrap_err().downcast::<String>().unwrap().to_string();
//! # let expect = "assertion failed: `assert_lt!(left, right)`\n  left: `2`,\n right: `1`";
//! # assert_eq!(actual, expect);
//! # }
//! ```
//!
//! 
//! ## assert_f_xx for function returns
//!   
//! * `assert_f_eq!(f, a, b)` ~ f(a) == f(b)
//!   
//! * `assert_f_ne!(f, a, b)` ~ f(a) != f(b)
//!   
//! * `assert_f_lt!(f, a, b)` ~ f(a) < f(b)
//!   
//! * `assert_f_le!(f, a, b)` ~ f(a) <= f(b)
//!   
//! * `assert_f_gt!(f, a, b)` ~ f(a) > f(b)
//!   
//! * `assert_f_ge!(f, a, b)` ~ f(a) >= f(b)
//!   
//! Examples:
//! 
//! ```rust
//! # #[macro_use] extern crate assertables;
//! # use std::panic;
//! # fn main() {
//! assert_f_lt!(i32::abs, 1, -2);
//! //-> ()
//!
//! # let result = panic::catch_unwind(|| {
//! assert_f_lt!(i32::abs, -2, 1);
//! //-> panic!
//! // assertion failed: `assert_f_eq!(function, left, right)`
//! //   left input: `-2`,
//! //  right input: `1`,
//! //  left output: `2`,
//! // right output: `1`
//! # });
//! # let actual = result.unwrap_err().downcast::<String>().unwrap().to_string();
//! # let expect = "assertion failed: `assert_f_lt!(function, left, right)`\n   left input: `-2`,\n  right input: `1`,\n  left output: `2`,\n right output: `1`";
//! # assert_eq!(actual, expect);
//! # }
//! ```
//!
//! 
//! ## assert_f_ok_xx for function Result Ok values
//!
//! * `assert_f_ok_eq!(f, a, b)` ~ f(a).unwrap() == f(b).unwrap()
//! 
//! * `assert_f_ok_ne!(f, a, b)` ~ f(a).unwrap() != f(b).unwrap()
//! 
//! * `assert_f_ok_lt!(f, a, b)` ~ f(a).unwrap() < f(b).unwrap()
//! 
//! * `assert_f_ok_le!(f, a, b)` ~ f(a).unwrap() <= f(b).unwrap()
//! 
//! * `assert_f_ok_gt!(f, a, b)` ~ f(a).unwrap() > f(b).unwrap()
//! 
//! * `assert_f_ok_ge!(f, a, b)` ~ f(a).unwrap() >= f(b).unwrap()
//!
//! ```rust
//! # #[macro_use] extern crate assertables;
//! # use std::panic;
//! fn example_digit_to_string(i: isize) -> Result<String, String> {
//!     match i {
//!         0..=9 => Ok(format!("{}", i)),
//!         _ => Err(format!("{:?} is out of range", i)),
//!     }
//! }
//!
//! # fn main() {
//! assert_f_ok_lt!(example_digit_to_string, 1, 2);
//! //-> ()
//! 
//! # let result = panic::catch_unwind(|| {
//! assert_f_ok_lt!(example_digit_to_string, 2, 1);
//! //-> panic!("…")
//! // assertion failed: `assert_f_eq!(function, left, right)`
//! //    left input: `2`,
//! //   right input: `1`,
//! //   left output: `\"2\"`,
//! //  right output: `\"1\"`
//! # });
//! # let actual = result.unwrap_err().downcast::<String>().unwrap().to_string();
//! # let expect = "assertion failed: `assert_f_ok_lt!(function, left, right)`\n   left input: `2`,\n  right input: `1`,\n  left output: `\"2\"`,\n right output: `\"1\"`";
//! # assert_eq!(actual, expect);
//! # }
//! ```
//!
//! 
//! ## assert_f_err_string_xx for function Err strings
//!
//! * `assert_f_err_string_eq!(f, a, b)` ~ f(a).unwrap_err().to_string() == f(b).unwrap_err().to_string()
//! 
//! * `assert_f_err_string_ne!(f, a, b)` ~ f(a).unwrap_err().to_string() != f(b).unwrap_err().to_string()
//! 
//! * `assert_f_err_string_lt!(f, a, b)` ~ f(a).unwrap_err().to_string() < f(b).unwrap_err().to_string()
//! 
//! * `assert_f_err_string_le!(f, a, b)` ~ f(a).unwrap_err().to_string() <= f(b).unwrap_err().to_string()
//! 
//! * `assert_f_err_string_gt!(f, a, b)` ~ f(a).unwrap_err().to_string() > f(b).unwrap_err().to_string()
//! 
//! * `assert_f_err_string_ge!(f, a, b)`~ f(a).unwrap_err().to_string() >= f(b).unwrap_err().to_string()
//! 
//! Examples:
//! 
//! ```rust
//! # #[macro_use] extern crate assertables;
//! # use std::panic;
//! fn example_digit_to_string(i: isize) -> Result<String, String> {
//!     match i {
//!         0..=9 => Ok(format!("{}", i)),
//!         _ => Err(format!("{:?} is out of range", i)),
//!     }
//! }
//!
//! # fn main() {
//! assert_f_err_string_lt!(example_digit_to_string, 10, 20);
//! //-> ()
//!
//! # let result = panic::catch_unwind(|| {
//! assert_f_err_string_lt!(example_digit_to_string, 20, 10);
//! //-> panic!
//  // assertion failed: `assert_f_err_string_eq!(example_digit_to_string, left, right)`
//  //    left input: `20`,
//  //   right input: `10``,
//  //   left is err: `true`,
//  //  right is err: `true`,
//  //   left output: `\"20 is out of range\"`,
//  //  right output: `\"10 is out of range\"`
//! # });
//! # let actual = result.unwrap_err().downcast::<String>().unwrap().to_string();
//! # let expect = "assertion failed: `assert_f_err_string_lt!(function, left, right)`\n   left input: `20`,\n  right input: `10`,\n  left is err: `true`,\n right is err: `true`,\n  left output: `\"20 is out of range\"`,\n right output: `\"10 is out of range\"`";
//! # assert_eq!(actual, expect);
//! # }
//! ```
//!
//! Two functions that we use often:
//!
//!   * `assert_f_ok_eq!(i32::from_str, str1, str2); // compare parsing of numbers`
//!
//!   * `assert_f_ok_eq!(std::fs::read_to_string, file1, file2); // compare file text`
//!
//!
//! ### assert_set_xx for set comparisons
//!
//! These macros help with comparison of set parameters, such as two arrays or
//! two vectors. where the item order does not matter, and the item count does
//! not matter. The macros convert inputs into HashSet iterators.
//!
//! * `assert_set_eq!(a, b)` ~ set a == set b
//! 
//! * `assert_set_ne!(a, b)` ~ set a != set b
//! 
//! * `assert_set_subset!(a, b)` ~ set a ⊆ set b
//! 
//! * `assert_set_superset!(a, b)` ~ set a ⊇ set b
//! 
//! * `assert_set_joint!(a, b)` ~ set a is joint with set b
//! 
//! * `assert_set_disjoint!(a, b)` ~ set a is disjoint with set b
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
//! //-> panic
//! // assertion failed: `assert_set_eq!(left, right)`
//! //   left: `[1, 2]`,
//! //  right: `[3, 4]`
//! # });
//! # let actual = result.unwrap_err().downcast::<String>().unwrap().to_string();
//! # let expect = "assertion failed: `assert_set_eq!(left, right)`\n  left: `[1, 2]`,\n right: `[3, 4]`";
//! # assert_eq!(actual, expect); 
//! //-> panic!("assertion failed: `assert_set_eq!(left, right)`\n  left: `[1, 2]`,\n right: `[3, 4]`");
//! # }
//! ```
//!
//!
//! ### assert_bag_xx for bag cmparisons
//!
//! Thes macros help with comparison of bag parameters, such as comparison of
//! two arrays or two vectors, where the item order does not matter, and the
//! item count does matter. The macros convert inputs into HashMap iterators.
//!
//! * `assert_bag_eq(a, b)` ~ bag a == bag b
//!
//! * `assert_bag_ne(a, b)` ~ bag a != bag b
//!
//! * `assert_bag_subbag(a, b)` ~ bag a ⊆ bag b
//!
//! * `assert_bag_superbag(a, b)` ~ bag a ⊇ bag b
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
//! ### assert_io_xx for input/output comparisons
//!
//! These macros help with input/output checking, 
//! such as with comparison of disk files, IO streams, etc.
//!
//! * `assert_io!(a)` ~ a is true
//! 
//! * `assert_io_eq!(a, b)` ~ a == b
//! 
//! * `assert_io_ne!(a, b)` ~ a != b
//! 
//! * `assert_io_lt!(a, b)` ~ a < b
//! 
//! * `assert_io_le!(a, b)` ~ a <= b
//! 
//! * `assert_io_gt!(a, b)` ~ a > b
//! 
//! * `assert_io_ge!(a, b)` ~ a >= b
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
//!
//! ## assert_read_to_string_xx for std::io::Read comparisons
//!
//! These macros help with readers, such as file handles, byte
//! arrays, input streams, and the trait std::io::Read.
//! 
//! * `assert_read_to_string_eq!(a, b)` ~ a.read_to_string() == b.read_to_string()
//! 
//! * `assert_read_to_string_ne!(a, b)` ~ a.read_to_string() != b.read_to_string()
//! 
//! * `assert_read_to_string_lt!(a, b)` ~ a.read_to_string() < b.read_to_string()
//! 
//! * `assert_read_to_string_le!(a, b)` ~ a.read_to_string() <= b.read_to_string()
//! 
//! * `assert_read_to_string_gt!(a, b)` ~ a.read_to_string() > b.read_to_string()
//! 
//! * `assert_read_to_string_ge!(a, b)` ~ a.read_to_string() >= b.read_to_string()
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
pub mod assert_f_eq; // equal
pub mod assert_f_ne; // not equal
pub mod assert_f_lt; // less than
pub mod assert_f_le; // less than or equal to
pub mod assert_f_gt; // greater than
pub mod assert_f_ge; // greater than or equal to

// Assert function ok() comparison
pub mod assert_f_ok_eq; // equal
pub mod assert_f_ok_ne; // not equal
pub mod assert_f_ok_lt; // less than
pub mod assert_f_ok_le; // less than or equal to
pub mod assert_f_ok_gt; // greater than
pub mod assert_f_ok_ge; // greater than or equal to

// Assert function err() comparison
pub mod assert_f_err_string_eq; // equal
pub mod assert_f_err_string_ne; // not equal
pub mod assert_f_err_string_lt; // less than
pub mod assert_f_err_string_le; // less than or equal to
pub mod assert_f_err_string_gt; // greater than
pub mod assert_f_err_string_ge; // greater than or equal to

// Assert iterator-related set-based comparison
pub mod assert_set_eq; // equal; set a == set b
pub mod assert_set_ne; // not equal; set a != set b
pub mod assert_set_subset; // set a ⊆ set b
pub mod assert_set_superset; // set a ⊇ set b
pub mod assert_set_joint; // set is joint with another set
pub mod assert_set_disjoint; // set is disjoint with another set

// Assert iterator-related bag-based comparison
pub mod assert_bag_eq; // equal; bag a == bag b
pub mod assert_bag_ne; // not equal; bag a != bag b
pub mod assert_bag_subbag; // bag a ⊆ bag b
pub mod assert_bag_superbag; // bag a ⊇ bag b

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
pub mod assertable_f_eq; // equal
pub mod assertable_f_ne; // not equal
pub mod assertable_f_lt; // less than
pub mod assertable_f_le; // less than or equal to
pub mod assertable_f_gt; // greater than
pub mod assertable_f_ge; // greater than or equal to

// Assertable function ok() comparison
pub mod assertable_f_ok_eq; // equal
pub mod assertable_f_ok_ne; // not equal
pub mod assertable_f_ok_lt; // less than
pub mod assertable_f_ok_le; // less than or equal to
pub mod assertable_f_ok_gt; // greater than
pub mod assertable_f_ok_ge; // greater than or equal to

// Assertable function err().to_string() comparison
pub mod assertable_f_err_string_eq; // equal
pub mod assertable_f_err_string_ne; // not equal
pub mod assertable_f_err_string_lt; // less than
pub mod assertable_f_err_string_le; // less than or equal to
pub mod assertable_f_err_string_gt; // greater than
pub mod assertable_f_err_string_ge; // greater than or equal to

// Assertable iterator-related set-based comparison
pub mod assertable_set_eq; // equal
pub mod assertable_set_ne; // not equal
pub mod assertable_set_subset; // set a ⊆ set b
pub mod assertable_set_superset; // set a ⊇ set b
pub mod assertable_set_joint; // set is joint with another set
pub mod assertable_set_disjoint; // set is disjoint with another set

// Assertable iterator-related bag-based comparison
pub mod assertable_bag_eq; // equal
pub mod assertable_bag_ne; // not equal
pub mod assertable_bag_subbag; // bag a ⊆ bag b
pub mod assertable_bag_superbag; // bag a ⊇ bag b

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
