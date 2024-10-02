//! Assert macros for comparing file system path contents.
//!
//! These macros help with file system paths, such as disk files, `Path`,
//! `PathBuf`, the trait `AsRef<Path>`, and anything that is readable via
//! `std::fs::read_to_string(…)`.
//!
//! Compare a path with another path:
//!
//! * [`assert_fs_read_to_string_eq!(path1, path2)`](macro@crate::assert_fs_read_to_string_eq) ≈ std::fs::read_to_string(path1) = std::fs::read_to_string(path2)
//!
//! * [`assert_fs_read_to_string_ne!(path1, path2)`](macro@crate::assert_fs_read_to_string_ne) ≈ std::fs::read_to_string(path1) ≠ std::fs::read_to_string(path2)
//!
//! * [`assert_fs_read_to_string_ge!(path1, path2)`](macro@crate::assert_fs_read_to_string_ge) ≈ std::fs::read_to_string(path1) ≥ std::fs::read_to_string(path2)
//!
//! * [`assert_fs_read_to_string_gt!(path1, path2)`](macro@crate::assert_fs_read_to_string_gt) ≈ std::fs::read_to_string(path1) > std::fs::read_to_string(path2)
//!
//! * [`assert_fs_read_to_string_le!(path1, path2)`](macro@crate::assert_fs_read_to_string_le) ≈ std::fs::read_to_string(path1) ≤ std::fs::read_to_string(path2)
//!
//! * [`assert_fs_read_to_string_lt!(path1, path2)`](macro@crate::assert_fs_read_to_string_lt) ≈ std::fs::read_to_string(path1) < std::fs::read_to_string(path2)
//!
//! Compare a path with an expression:
//!
//! * [`assert_fs_read_to_string_eq_expr!(path, expr)`](macro@crate::assert_fs_read_to_string_eq_expr) ≈ std::fs::read_to_string(path) = expr
//!
//! * [`assert_fs_read_to_string_ne_expr!(path, expr)`](macro@crate::assert_fs_read_to_string_ne_expr) ≈ std::fs::read_to_string(path) ≠ expr
//!
//! * [`assert_fs_read_to_string_ge_expr!(path, expr)`](macro@crate::assert_fs_read_to_string_ge_expr) ≈ std::fs::read_to_string(path) ≥ expr
//!
//! * [`assert_fs_read_to_string_gt_expr!(path, expr)`](macro@crate::assert_fs_read_to_string_gt_expr) ≈ std::fs::read_to_string(path) > expr
//!
//! * [`assert_fs_read_to_string_le_expr!(path, expr)`](macro@crate::assert_fs_read_to_string_le_expr) ≈ std::fs::read_to_string(path) ≤ expr
//!
//! * [`assert_fs_read_to_string_lt_expr!(path, expr)`](macro@crate::assert_fs_read_to_string_lt_expr) ≈ std::fs::read_to_string(path) < expr
//!
//! Compare a path with its contents:
//!
//! * [`assert_fs_read_to_string_contains!(path, containee)`](macro@crate::assert_fs_read_to_string_contains) ≈ std::fs::read_to_string(path).contains(containee)
//!
//! * [`assert_fs_read_to_string_matches!(path, matcher)`](macro@crate::assert_fs_read_to_string_matches) ≈ matcher.is_match(std::fs::read_to_string(path))
//!
//!
//! # Example
//!
//! ```rust
//! use assertables::*;
//! use std::io::Read;
//!
//! # fn main() {
//! let a ="alfa.txt";
//! let b = "alfa.txt";
//! assert_fs_read_to_string_eq!(&a, &b);
//! # }
//! ```


// Comparisons
pub mod assert_fs_read_to_string_eq;
pub mod assert_fs_read_to_string_ne;
pub mod assert_fs_read_to_string_lt;
pub mod assert_fs_read_to_string_le;
pub mod assert_fs_read_to_string_gt;
pub mod assert_fs_read_to_string_ge;

// Compare with expression
pub mod assert_fs_read_to_string_eq_expr;
pub mod assert_fs_read_to_string_ne_expr;
pub mod assert_fs_read_to_string_lt_expr;
pub mod assert_fs_read_to_string_le_expr;
pub mod assert_fs_read_to_string_gt_expr;
pub mod assert_fs_read_to_string_ge_expr;

// Specializations
pub mod assert_fs_read_to_string_contains;
pub mod assert_fs_read_to_string_matches;
