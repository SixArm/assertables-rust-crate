//! Assert for comparing floating-point 32-bit numbers within 2.0 * EPSILON.
//!
//! These macros are available:
//!
//! * [`assert_f64_eq!(a, b)`](module@crate::assert_f64_eq) ≈ a = b (within 2ε)
//! * [`assert_f64_ne!(a, b)`](module@crate::assert_f64_ne) ≈ a ≠ b (within 2ε)
//!
//!
//! # Example
//!
//! ```rust
//! use assertables::*;
//!
//! let a: f64 = 0.3333333333333333;
//! let b: f64 = 0.3333333333333334;
//! assert_f64_eq!(a, b);
//!
//! let a: f64 = 0.3333333333333333;
//! let b: f64 = 0.3333333333333338;
//! assert_f64_ne!(a, b);
//! ```
//! ```

mod assert_f64_eq;
mod assert_f64_ne;
// mod assert_f64_lt;
// mod assert_f64_le;
// mod assert_f64_gt;
// mod assert_f64_ge;
