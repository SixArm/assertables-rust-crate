//! Assert in nearness.
//!
//! These macros compare numbers, such as two floating point numbers,
//! where one number may be very close to another number but not quite equal.
//!
//! * [`assert_in_delta!(a, b, delta)`](macro@crate::assert_in_delta) ≈ | a - b | ≤ Δ
//!
//! * [`assert_in_epsilon!(a, b, epsilon)`](macro@crate::assert_in_epsilon) ≈ | a - b | ≤ ε * min(a, b)
//!
//! # Example
//!
//! ```rust
//! use assertables::*;
//!
//! # fn main() {
//! let a: i8 = 10;
//! let b: i8 = 11;
//! let delta: i8 = 1;
//! assert_in_delta!(a, b, delta);
//! # }
//! ```

pub mod assert_in_delta;
pub mod assert_in_epsilon;
