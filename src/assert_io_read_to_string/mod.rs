//! Assert for comparing input/output reader streams.
//!
//! These macros help with input/output readers, such as file handles, byte arrays,
//! input streams, the trait `::std::io::Read`, and anything that implements a
//! method `read_to_string() -> String`. See tutorial below.
//!
//! ## Macros
//!
//! Compare a reader with another reader:
//!
//! * [`assert_io_read_to_string_eq!(reader1, reader2)`](macro@crate::assert_io_read_to_string_eq) ≈ reader1.read_to_string() = reader2.read_to_string()
//! * [`assert_io_read_to_string_ne!(reader1, reader2)`](macro@crate::assert_io_read_to_string_ne) ≈ reader1.read_to_string() ≠ reader2.read_to_string()
//! * [`assert_io_read_to_string_lt!(reader1, reader2)`](macro@crate::assert_io_read_to_string_lt) ≈ reader1.read_to_string() < reader2.read_to_string()
//! * [`assert_io_read_to_string_le!(reader1, reader2)`](macro@crate::assert_io_read_to_string_le) ≈ reader1.read_to_string() ≤ reader2.read_to_string()
//! * [`assert_io_read_to_string_gt!(reader1, reader2)`](macro@crate::assert_io_read_to_string_gt) ≈ reader1.read_to_string() > reader2.read_to_string()
//! * [`assert_io_read_to_string_ge!(reader1, reader2)`](macro@crate::assert_io_read_to_string_ge) ≈ reader1.read_to_string() ≥ reader2.read_to_string()
//!
//! Compare a reader with an expression:
//!
//! * [`assert_io_read_to_string_eq_x!(reader, expr)`](macro@crate::assert_io_read_to_string_eq_x) ≈ reader.read_to_string() = expr
//! * [`assert_io_read_to_string_ne_x!(reader, expr)`](macro@crate::assert_io_read_to_string_ne_x) ≈ reader.read_to_string() ≠ expr
//! * [`assert_io_read_to_string_lt_x!(reader, expr)`](macro@crate::assert_io_read_to_string_lt_x) ≈ reader.read_to_string() < expr
//! * [`assert_io_read_to_string_le_x!(reader, expr)`](macro@crate::assert_io_read_to_string_le_x) ≈ reader.read_to_string() ≤ expr
//! * [`assert_io_read_to_string_gt_x!(reader, expr)`](macro@crate::assert_io_read_to_string_gt_x) ≈ reader.read_to_string() > expr
//! * [`assert_io_read_to_string_ge_x!(reader, expr)`](macro@crate::assert_io_read_to_string_ge_x) ≈ reader.read_to_string() ≥ expr
//!
//! Compare a reader with its contents:
//!
//! * [`assert_io_read_to_string_contains!(reader, &containee)`](macro@crate::assert_io_read_to_string_contains) ≈ reader.read_to_string().contains(containee)
//! * [`assert_io_read_to_string_is_match!(reader, &matcher)`](macro@crate::assert_io_read_to_string_is_match) ≈ matcher.is_match(reader.read_to_string())
//!
//!
//! # Example
//!
//! ```rust
//! use assertables::*;
//! use std::io::Read;
//!
//! let mut a = "alfa".as_bytes();
//! let mut b = "alfa".as_bytes();
//! assert_io_read_to_string_eq!(a, b);
//! ```
//!
//! ## Tutorial
//!
//! Rust has a concept of a "reader", such as using `::std::io::Read` to read bytes,
//! or to use the method `read_to_string` to read bytes into a string buffer.
//!
//! ```rust
//! use std::io::Read;
//! let mut reader = "hello".as_bytes();
//! let mut string = String::new();
//! let result = reader.read_to_string(&mut string);
//! ```
//!
//! Rust can compare a reader's string to another reader's string:
//!
//! ```rust
//! use std::io::Read;
//! let mut reader1 = "hello".as_bytes();
//! let mut reader2 = "world".as_bytes();
//! let mut a_string = String::new();
//! let mut b_string = String::new();
//! let result1 = reader1.read_to_string(&mut a_string);
//! let result2 = reader2.read_to_string(&mut b_string);
//! assert_ne!(a_string, b_string);
//! ```
//!
//! The `assertables` crate provides macros that do the reading and string buffering for you:
//!
//! ```rust
//! # use std::io::Read;
//! # use assertables::*;
//! let mut reader1 = "hello".as_bytes();
//! let mut reader2 = "world".as_bytes();
//! assert_io_read_to_string_ne!(reader1, reader2);
//! ```

// Compare another
pub mod assert_io_read_to_string_eq;
pub mod assert_io_read_to_string_ge;
pub mod assert_io_read_to_string_gt;
pub mod assert_io_read_to_string_le;
pub mod assert_io_read_to_string_lt;
pub mod assert_io_read_to_string_ne;

// Compare expression
pub mod assert_io_read_to_string_eq_x;
pub mod assert_io_read_to_string_ge_x;
pub mod assert_io_read_to_string_gt_x;
pub mod assert_io_read_to_string_le_x;
pub mod assert_io_read_to_string_lt_x;
pub mod assert_io_read_to_string_ne_x;

// Specializations
pub mod assert_io_read_to_string_contains;
pub mod assert_io_read_to_string_is_match;
pub mod assert_io_read_to_string_matches; // Deprecated.
