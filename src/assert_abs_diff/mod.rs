//! Assert for comparing absolute differences.
//!
//! These macros help with collection lengths, such as for strings, arrays,
//! vectors, iterators, and anything that has a typical `.len()` method.
//!
//! Compare an absolute difference with an deltaession:
//!
//! * [`assert_abs_diff_eq!(a, b, delta)`](macro@crate::assert_abs_diff_eq) ≈ | a - b | = Δ
//!
//! * [`assert_abs_diff_ne!(a, b, delta)`](macro@crate::assert_abs_diff_ne) ≈ | a - b | ≠ Δ
//!
//! * [`assert_abs_diff_lt!(a, b, delta)`](macro@crate::assert_abs_diff_lt) ≈ | a - b | < Δ
//!
//! * [`assert_abs_diff_le!(a, b, delta)`](macro@crate::assert_abs_diff_le) ≈ | a - b | ≤ Δ
//!
//! * [`assert_abs_diff_gt!(a, b, delta)`](macro@crate::assert_abs_diff_gt) ≈ | a - b | > Δ
//!
//! * [`assert_abs_diff_ge!(a, b, delta)`](macro@crate::assert_abs_diff_ge) ≈ | a - b | ≥ Δ
//!
//! # Example
//!
//! ```rust
//! use assertables::*;
//!
//! # fn main() {
//! let a = 10;
//! let b = 13;
//! let delta = 3;
//! assert_abs_diff_eq!(a, b, delta);
//! # }
//! ```

pub mod assert_abs_diff_eq;
pub mod assert_abs_diff_ge;
pub mod assert_abs_diff_gt;
pub mod assert_abs_diff_le;
pub mod assert_abs_diff_lt;
pub mod assert_abs_diff_ne;
