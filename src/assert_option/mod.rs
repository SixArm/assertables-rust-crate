//! Assert macros for `Option` {`Some`, `None`}
//!
//! These macros help compare an `Option` that is either `Some`, `None`.
//!
//! The macros use these capabilities:
//!
//! * implements `.is_some() -> boolean`
//!
//! * implements `.unwrap_some() -> comparable`
//!
//! * implements `.is_none() -> boolean`
//!
//! # Macros
//!
//! * [`assert_option_some!(a)`](macro@crate::assert_option_some) ≈ a.is_some()
//!
//! * [`assert_option_none!(a)`](macro@crate::assert_option_none) ≈ a.is_none()
//!
//! # Example
//! 
//! ```rust
//! # #[macro_use] extern crate assertables;
//! # fn main() {
//! let a: Option<i8> = Option::Some(1);
//! assert_option_some!(a);
//! # }
//! ```

pub mod assert_option_some;
pub mod assert_option_none;
