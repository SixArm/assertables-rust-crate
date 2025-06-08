//! Assert expression is possibly an email address or not.
//!
//! * [`assert_email_address!(a)`](macro@crate::assert_email_address) ≈ email_address(a)
//!
//! * [`assert_not_email_address!(a)`](macro@crate::assert_email_address) ≈ not_email_address(a)
//!
//! # Example
//!
//! ```rust
//! use assertables::*;
//!
//! let a = "hello@example.com";
//! assert_email_address!(a);
//! ```

pub mod assert_email_address;
pub mod assert_not_email_address;
