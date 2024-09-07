//! Assert macros for `Result` {`Ok`, `Err`}
//!
//! These macros help compare a `Result` that is either `Ok`, `Err`.
//!
//! The macros use these capabilities:
//!
//! * implements `.is_ok() -> boolean`
//!
//! * implements `.unwrap_ok() -> comparable`
//!
//! * implements `.is_err() -> boolean`
//!
//! * implements `.unwrap_err() -> comparable`
//!
//! # Macros
//!
//! * [`assert_result_ok!(a)`](macro@crate::assert_result_ok) ≈ a.is_ok()
//!
//! * [`assert_result_err!(a)`](macro@crate::assert_result_err) ≈ a.is_err()
//!
//! # Example
//!
//! ```rust
//! # #[macro_use] extern crate assertables;
//!
//! # fn main() {
//! let a: Result<(), i8> = Result::Ok(());
//! assert_result_ok!(a);
//! # }
//! ```

// Assert for Result
pub mod assert_result_ok;
// pub mod assert_result_ok_eq; //TODO
// pub mod assert_result_ok_eq_expr; //TODO
pub mod assert_result_err;

