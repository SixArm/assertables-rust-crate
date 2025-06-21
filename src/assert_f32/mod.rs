//! Assert for comparing floating-point 32-bit numbers within 2.0 * EPSILON.
//!
//! These macros are available:
//!
//! * [`assert_f32_eq!(a, b)`](module@crate::assert_f32_eq) ≈ a = b (within 2ε)
//! * [`assert_f32_ne!(a, b)`](module@crate::assert_f32_ne) ≈ a ≠ b (within 2ε)
//!
//! # Example
//!
//! ```rust
//! use assertables::*;
//!
//! let a: f32 = 0.3333333;
//! let b: f32 = 0.3333334;
//! assert_f32_eq!(a, b);
//!
//! let a: f32 = 0.3333333;
//! let b: f32 = 0.3333336;
//! assert_f32_ne!(a, b);

//! ```

mod assert_f32_eq;
mod assert_f32_ne;
