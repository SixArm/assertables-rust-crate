//! Assert for comparing lengths.
//!
//! These macros help with collection lengths, such as for strings, arrays,
//! vectors, iterators, and anything that has a typical `.len()` method.
//!
//! Compare a length with another length:
//!
//! * [`assert_process_status_code_value_eq!(a, b)`](macro@crate::assert_process_status_code_value_eq) ≈ a.len() = b.len()
//! * [`assert_process_status_code_value_ne!(a, b)`](macro@crate::assert_process_status_code_value_ne) ≈ a.len() ≠ b.len()
//! * [`assert_process_status_code_value_lt!(a, b)`](macro@crate::assert_process_status_code_value_lt) ≈ a.len() < b.len()
//! * [`assert_process_status_code_value_le!(a, b)`](macro@crate::assert_process_status_code_value_le) ≈ a.len() ≤ b.len()
//! * [`assert_process_status_code_value_gt!(a, b)`](macro@crate::assert_process_status_code_value_gt) ≈ a.len() > b.len()
//! * [`assert_process_status_code_value_ge!(a, b)`](macro@crate::assert_process_status_code_value_ge) ≈ a.len() ≥ b.len()
//!
//! Compare a length with an expression:
//!
//! * [`assert_process_status_code_value_eq_x!(a, expr)`](macro@crate::assert_process_status_code_value_eq_x) ≈ a.len() = expr
//! * [`assert_process_status_code_value_ne_x!(a, expr)`](macro@crate::assert_process_status_code_value_ne_x) ≈ a.len() ≠ expr
//! * [`assert_process_status_code_value_lt_x!(a, expr)`](macro@crate::assert_process_status_code_value_lt_x) ≈ a.len() < expr
//! * [`assert_process_status_code_value_le_x!(a, expr)`](macro@crate::assert_process_status_code_value_le_x) ≈ a.len() ≤ expr
//! * [`assert_process_status_code_value_gt_x!(a, expr)`](macro@crate::assert_process_status_code_value_gt_x) ≈ a.len() > expr
//! * [`assert_process_status_code_value_ge_x!(a, expr)`](macro@crate::assert_process_status_code_value_ge_x) ≈ a.len() ≥ expr
//!
//! # Example
//!
//! ```rust
//! use assertables::*;
//! use std::process::Command;
//!
//! # fn main() {
//! let mut a = Command::new("bin/exit-with-arg"); a.arg("1");
//! let mut b = Command::new("bin/exit-with-arg"); b.arg("1");
//! assert_process_status_code_value_eq!(a, b);
//! # }
//! ```

// Compare another
pub mod assert_process_status_code_value_eq;
pub mod assert_process_status_code_value_lt;
pub mod assert_process_status_code_value_ne;

pub mod assert_process_status_code_value_ge;
pub mod assert_process_status_code_value_gt;
pub mod assert_process_status_code_value_le;

// Compare expression
pub mod assert_process_status_code_value_eq_x;
pub mod assert_process_status_code_value_ge_x;
pub mod assert_process_status_code_value_gt_x;
pub mod assert_process_status_code_value_le_x;
pub mod assert_process_status_code_value_lt_x;
pub mod assert_process_status_code_value_ne_x;
