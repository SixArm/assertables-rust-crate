//! Assert for comparing floating-point 32-bit numbers within 2.0 * EPSILON.
//!
//! These macros are available:
//!
//! * [`assert_f64_eq!(a, b)`](macro@crate::assert_f64_eq) ≈ a = b (within 2ε)
//! * [`assert_f64_ne!(a, b)`](macro@crate::assert_f64_ne) ≈ a ≠ b (within 2ε)
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

#[cfg(test)] pub const EQ:    f64 = 1.0/3.0;
#[cfg(test)] pub const EQ_LT: f64 = 0.3333333333333332;
#[cfg(test)] pub const EQ_GT: f64 = 0.3333333333333335;
#[cfg(test)] pub const LT:    f64 = 0.3333333333333329;
#[cfg(test)] pub const GT:    f64 = 0.3333333333333339;

mod assert_f64_eq;
mod assert_f64_ge;
mod assert_f64_gt;
mod assert_f64_le;
mod assert_f64_lt;
mod assert_f64_ne;
