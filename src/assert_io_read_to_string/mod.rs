//! Assert for comparing input/output reader streams.
//!
//! These macros help with input/output readers, such as file handles, byte arrays,
//! input streams, the trait `::std::io::Read`, and anything that implements a
//! method `read_to_string() -> String`.
//!
//! Compare a reader with another reader:
//!
//! * [`assert_io_read_to_string_eq2!(reader1, reader2)`](macro@crate::assert_io_read_to_string_eq2) ≈ reader1.read_to_string() = reader2.read_to_string()
//!
//! * [`assert_io_read_to_string_ne2!(reader1, reader2)`](macro@crate::assert_io_read_to_string_ne2) ≈ reader1.read_to_string() ≠ reader2.read_to_string()
//!
//! * [`assert_io_read_to_string_lt2!(reader1, reader2)`](macro@crate::assert_io_read_to_string_lt2) ≈ reader1.read_to_string() < reader2.read_to_string()
//!
//! * [`assert_io_read_to_string_le2!(reader1, reader2)`](macro@crate::assert_io_read_to_string_le2) ≈ reader1.read_to_string() ≤ reader2.read_to_string()
//!
//! * [`assert_io_read_to_string_gt2!(reader1, reader2)`](macro@crate::assert_io_read_to_string_gt2) ≈ reader1.read_to_string() > reader2.read_to_string()
//!
//! * [`assert_io_read_to_string_ge2!(reader1, reader2)`](macro@crate::assert_io_read_to_string_ge2) ≈ reader1.read_to_string() ≥ reader2.read_to_string()
//!
//! Compare a reader with an expression:
//!
//! * [`assert_io_read_to_string_eq!(reader, expr)`](macro@crate::assert_io_read_to_string_eq) ≈ reader.read_to_string() = expr
//!
//! * [`assert_io_read_to_string_ne!(reader, expr)`](macro@crate::assert_io_read_to_string_ne) ≈ reader.read_to_string() ≠ expr
//!
//! * [`assert_io_read_to_string_lt!(reader, expr)`](macro@crate::assert_io_read_to_string_lt) ≈ reader.read_to_string() < expr
//!
//! * [`assert_io_read_to_string_le!(reader, expr)`](macro@crate::assert_io_read_to_string_le) ≈ reader.read_to_string() ≤ expr
//!
//! * [`assert_io_read_to_string_gt!(reader, expr)`](macro@crate::assert_io_read_to_string_gt) ≈ reader.read_to_string() > expr
//!
//! * [`assert_io_read_to_string_ge!(reader, expr)`](macro@crate::assert_io_read_to_string_ge) ≈ reader.read_to_string() ≥ expr
//!
//! Compare a reader with its contents:
//!
//! * [`assert_io_read_to_string_contains!(reader, &containee)`](macro@crate::assert_io_read_to_string_contains) ≈ reader.read_to_string().contains(containee)
//!
//! * [`assert_io_read_to_string_is_match!(reader, &matcher)`](macro@crate::assert_io_read_to_string_is_match) ≈ matcher.is_match(reader.read_to_string())
//!
//!
//! # Example
//!
//! ```rust
//! use assertables::*;
//! use std::io::Read;
//!
//! # fn main() {
//! let mut a = "alfa".as_bytes();
//! let mut b = "alfa".as_bytes();
//! assert_io_read_to_string_eq2!(a, b);
//! # }
//! ```

// Comparisons
pub mod assert_io_read_to_string_eq2;
pub mod assert_io_read_to_string_ge2;
pub mod assert_io_read_to_string_gt2;
pub mod assert_io_read_to_string_le2;
pub mod assert_io_read_to_string_lt2;
pub mod assert_io_read_to_string_ne2;

// Compare with expression
pub mod assert_io_read_to_string_eq;
pub mod assert_io_read_to_string_ge;
pub mod assert_io_read_to_string_gt;
pub mod assert_io_read_to_string_le;
pub mod assert_io_read_to_string_lt;
pub mod assert_io_read_to_string_ne;

// Specializations
pub mod assert_io_read_to_string_contains;
pub mod assert_io_read_to_string_is_match;
pub mod assert_io_read_to_string_matches; // Deprecated.
