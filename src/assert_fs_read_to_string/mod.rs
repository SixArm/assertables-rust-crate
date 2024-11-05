//! Assert for comparing file system path contents.
//!
//! These macros help with file system paths, such as disk files, `Path`,
//! `PathBuf`, the trait `AsRef<Path>`, and anything that is readable via
//! `::std::fs::read_to_string(…)`. See tutorial below.
//!
//! Compare a path with another path:
//!
//! * [`assert_fs_read_to_string_eq!(path1, path2)`](macro@crate::assert_fs_read_to_string_eq) ≈ std::fs::read_to_string(path1) = std::fs::read_to_string(path2)
//! * [`assert_fs_read_to_string_ne!(path1, path2)`](macro@crate::assert_fs_read_to_string_ne) ≈ std::fs::read_to_string(path1) ≠ std::fs::read_to_string(path2)
//! * [`assert_fs_read_to_string_lt!(path1, path2)`](macro@crate::assert_fs_read_to_string_lt) ≈ std::fs::read_to_string(path1) < std::fs::read_to_string(path2)
//! * [`assert_fs_read_to_string_le!(path1, path2)`](macro@crate::assert_fs_read_to_string_le) ≈ std::fs::read_to_string(path1) ≤ std::fs::read_to_string(path2)
//! * [`assert_fs_read_to_string_gt!(path1, path2)`](macro@crate::assert_fs_read_to_string_gt) ≈ std::fs::read_to_string(path1) > std::fs::read_to_string(path2)
//! * [`assert_fs_read_to_string_ge!(path1, path2)`](macro@crate::assert_fs_read_to_string_ge) ≈ std::fs::read_to_string(path1) ≥ std::fs::read_to_string(path2)
//!
//! Compare a path with an expression:
//!
//! * [`assert_fs_read_to_string_eq_x!(path, expr)`](macro@crate::assert_fs_read_to_string_eq_x) ≈ std::fs::read_to_string(path) = expr
//! * [`assert_fs_read_to_string_ne_x!(path, expr)`](macro@crate::assert_fs_read_to_string_ne_x) ≈ std::fs::read_to_string(path) ≠ expr
//! * [`assert_fs_read_to_string_lt_x!(path, expr)`](macro@crate::assert_fs_read_to_string_lt_x) ≈ std::fs::read_to_string(path) < expr
//! * [`assert_fs_read_to_string_le_x!(path, expr)`](macro@crate::assert_fs_read_to_string_le_x) ≈ std::fs::read_to_string(path) ≤ expr
//! * [`assert_fs_read_to_string_gt_x!(path, expr)`](macro@crate::assert_fs_read_to_string_gt_x) ≈ std::fs::read_to_string(path) > expr
//! * [`assert_fs_read_to_string_ge_x!(path, expr)`](macro@crate::assert_fs_read_to_string_ge_x) ≈ std::fs::read_to_string(path) ≥ expr
//!
//! Compare a path with its contents:
//!
//! * [`assert_fs_read_to_string_contains!(path, containee)`](macro@crate::assert_fs_read_to_string_contains) ≈ std::fs::read_to_string(path).contains(containee)
//! * [`assert_fs_read_to_string_is_match!(path, matcher)`](macro@crate::assert_fs_read_to_string_is_match) ≈ matcher.is_match(::std::fs::read_to_string(path))
//!
//! # Example
//!
//! ```rust
//! use assertables::*;
//! use std::io::Read;
//!
//! let a ="alfa.txt";
//! let b = "alfa.txt";
//! assert_fs_read_to_string_eq!(&a, &b);
//! ```
//!
//! ## Tutorial
//!
//! Rust has a concept of a "file system" and a function "read to string"
//! that makes it easy to read a file to a string:
//!
//! ```rust
//! let path = "alfa.txt";
//! let string = ::std::fs::read_to_string(path);
//! ```
//!
//! Rust can compare a file's string to another file's string:
//!
//! ```rust
//! let path1 = "alfa.txt";
//! let path2 = "bravo.txt";
//! let a_string = ::std::fs::read_to_string(path1).unwrap();
//! let b_string = ::std::fs::read_to_string(path2).unwrap();
//! assert_ne!(a_string, b_string);
//! ```
//!
//! The `assertables` crate provides macros that do the reading for you:
//!
//! ```rust
//! # use assertables::*;
//! let path1 = "alfa.txt";
//! let path2 = "bravo.txt";
//! assert_fs_read_to_string_ne!(path1, path2);
//! ```

// Compare another
pub mod assert_fs_read_to_string_eq;
pub mod assert_fs_read_to_string_ge;
pub mod assert_fs_read_to_string_gt;
pub mod assert_fs_read_to_string_le;
pub mod assert_fs_read_to_string_lt;
pub mod assert_fs_read_to_string_ne;

// Compare expression
pub mod assert_fs_read_to_string_eq_x;
pub mod assert_fs_read_to_string_ge_x;
pub mod assert_fs_read_to_string_gt_x;
pub mod assert_fs_read_to_string_le_x;
pub mod assert_fs_read_to_string_lt_x;
pub mod assert_fs_read_to_string_ne_x;

// Specializations
pub mod assert_fs_read_to_string_contains;
pub mod assert_fs_read_to_string_is_match;
pub mod assert_fs_read_to_string_matches; // Deprecated.
