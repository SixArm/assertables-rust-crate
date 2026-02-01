//! Assert for comparing an iter collection with a single element.
//!
//! These macros help with comparison of iter parameters, such as a collection of struct
//! and a single struct.
//! These macros convert each input using the std::iter::Iterator trait.
//!
//! * [`assert_any_eq!(collection, item)`](macro@crate::assert_any_eq) ≈ iter a contains b
//! * [`assert_any_ne!(collection, item)`](macro@crate::assert_any_ne) ≈ iter a does not contain b
//! * [`assert_any_lt!(collection, item)`](macro@crate::assert_any_gt) ≈ iter a contains ≥ one element < b
//! * [`assert_any_le!(collection, item)`](macro@crate::assert_any_gt) ≈ iter a contains ≥ one element ≤ b
//! * [`assert_any_gt!(collection, item)`](macro@crate::assert_any_gt) ≈ iter a contains ≥ one element > b
//! * [`assert_any_ge!(collection, item)`](macro@crate::assert_any_ge) ≈ iter a contains ≥ one element ≥ b
//!
//! # Example
//!
//! ```rust
//! use assertables::*;
//!
//! let a = [1, 2];
//! let b = 1;
//! assert_any_eq!(&a, &b);
//! ```

// Comparisons
pub mod assert_any;
pub mod assert_any_eq;
pub mod assert_any_ge;
pub mod assert_any_gt;
pub mod assert_any_le;
pub mod assert_any_lt;
pub mod assert_any_ne;
