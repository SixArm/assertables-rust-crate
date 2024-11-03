//! Assert for comparing status concepts.
//!
//! These macros help with commands, program, processes, and anything else that
//! provides a method `status()`, and optionally status methods such as:
//!
//! * `success() => bool`
//! * `code() => Result(T, E)`
//!
//! Try success/failure:
//!
//! * [`assert_status_success!(a)`](macro@crate::assert_status_success) ≈ a.status().success() = true``
//! * [`assert_status_success_false!(a)`](macro@crate::assert_status_success_false) ≈ a.status().success() = false``
//!
//! Compare a status code with another status code:
//!
//! * [`assert_status_code_value_eq!(a, b)`](macro@crate::assert_status_code_value_eq) ≈ a.len() = b.len()
//! * [`assert_status_code_value_ne!(a, b)`](macro@crate::assert_status_code_value_ne) ≈ a.len() ≠ b.len()
//! * [`assert_status_code_value_lt!(a, b)`](macro@crate::assert_status_code_value_lt) ≈ a.len() < b.len()
//! * [`assert_status_code_value_le!(a, b)`](macro@crate::assert_status_code_value_le) ≈ a.len() ≤ b.len()
//! * [`assert_status_code_value_gt!(a, b)`](macro@crate::assert_status_code_value_gt) ≈ a.len() > b.len()
//! * [`assert_status_code_value_ge!(a, b)`](macro@crate::assert_status_code_value_ge) ≈ a.len() ≥ b.len()
//!
//! Compare a status code with an expression:
//!
//! * [`assert_status_code_value_eq_x!(a, expr)`](macro@crate::assert_status_code_value_eq_x) ≈ a.len() = expr
//! * [`assert_status_code_value_ne_x!(a, expr)`](macro@crate::assert_status_code_value_ne_x) ≈ a.len() ≠ expr
//! * [`assert_status_code_value_lt_x!(a, expr)`](macro@crate::assert_status_code_value_lt_x) ≈ a.len() < expr
//! * [`assert_status_code_value_le_x!(a, expr)`](macro@crate::assert_status_code_value_le_x) ≈ a.len() ≤ expr
//! * [`assert_status_code_value_gt_x!(a, expr)`](macro@crate::assert_status_code_value_gt_x) ≈ a.len() > expr
//! * [`assert_status_code_value_ge_x!(a, expr)`](macro@crate::assert_status_code_value_ge_x) ≈ a.len() ≥ expr
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
//! assert_status_code_value_eq!(a, b);
//! # }
//! ```

// For success/failure
pub mod assert_status_success;
pub mod assert_status_success_false;

// Compare another
pub mod assert_status_code_value_eq;
pub mod assert_status_code_value_ge;
pub mod assert_status_code_value_gt;
pub mod assert_status_code_value_le;
pub mod assert_status_code_value_lt;
pub mod assert_status_code_value_ne;

// Compare expression
pub mod assert_status_code_value_eq_x;
pub mod assert_status_code_value_ge_x;
pub mod assert_status_code_value_gt_x;
pub mod assert_status_code_value_le_x;
pub mod assert_status_code_value_lt_x;
pub mod assert_status_code_value_ne_x;
