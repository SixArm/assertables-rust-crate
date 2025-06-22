//! Assert for comparing floating-point 32-bit numbers within 2.0 * EPSILON.
//!
//! These macros are available:
//!
//! * [`assert_f32_eq!(a, b)`](macro@crate::assert_f32_eq) ≈ a = b (within 2ε)
//! * [`assert_f32_ne!(a, b)`](macro@crate::assert_f32_ne) ≈ a ≠ b (within 2ε)
//!
//! # Example
//!
//! ```rust
//! use assertables::*;
//!
//! let a: f32 = 1.0 / 3.0;
//! let b: f32 = 0.3333333;
//! assert_f32_eq!(a, b);
//!
//! let a: f32 = 1.0 / 3.0;
//! let b: f32 = 0.3333336;
//! assert_f32_ne!(a, b);
//! ```

#[cfg(test)] pub const EQ:    f32 = 1.0 / 3.0;
#[cfg(test)] pub const EQ_LT: f32 = 0.3333333;
#[cfg(test)] pub const EQ_GT: f32 = 0.3333334;
#[cfg(test)] pub const LT:    f32 = 0.3333331;
#[cfg(test)] pub const GT:    f32 = 0.3333336;

mod assert_f32_eq;
mod assert_f32_ge;
mod assert_f32_gt;
mod assert_f32_le;
mod assert_f32_lt;
mod assert_f32_ne;
