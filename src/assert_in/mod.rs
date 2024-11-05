//! Assert in nearness.
//!
//! These macros compare numbers, such as two numbers
//! where one number may be close to another number.
//!
//! * [`assert_in!(a, container)`](macro@crate::assert_in) ≈ a is in container
//! * [`assert_in_range!(a, range)`](macro@crate::assert_in_range) ≈ a is in range
//! * [`assert_in_delta!(a, b, delta)`](macro@crate::assert_in_delta) ≈ | a - b | ≤ Δ
//! * [`assert_in_epsilon!(a, b, epsilon)`](macro@crate::assert_in_epsilon) ≈ | a - b | ≤ ε * min(a, b)
//!
//! # Example
//!
//! ```rust
//! use assertables::*;
//!
//! let a: i8 = 10;
//! let b: i8 = 11;
//! let delta: i8 = 1;
//! assert_in_delta!(a, b, delta);
//! ```

pub mod assert_in;
pub mod assert_in_delta;
pub mod assert_in_epsilon;
pub mod assert_in_range;
