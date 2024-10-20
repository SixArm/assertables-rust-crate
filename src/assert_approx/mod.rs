//! Assert for approximations.
//!
//! These macros compare numbers, such as two floating point numbers,
//! where one number may be very close to another number but not quite equal.
//!
//! * [`assert_approx_eq!(a, b)`](macro@crate::assert_approx_eq) ≈ a is approximately equal to b
//!
//! * [`assert_approx_ne!(a, b)`](macro@crate::assert_approx_ne) ≈ a is not approximately equal to b
//!
//! # Example
//!
//! ```rust
//! use assertables::*;
//!
//! # fn main() {
//! let a: f32 = 1.0000001;
//! let b: f32 = 1.0000011;
//! assert_approx_eq!(a, b);
//! # }
//! ```

/// Format assert failure error message.
#[macro_export]
macro_rules! assert_approx_xx_impl_err {
    ($name:ident, $($arg:tt)*) => {
        format!(
            concat!(
                "assertion failed: `{}!(a, b)`\n",
                "https://docs.rs/assertables/9.0.0/assertables/macro.{}.html\n",
                "            a label: `{}`,\n",
                "            a debug: `{:?}`,\n",
                "            b label: `{}`,\n",
                "            b debug: `{:?}`,\n",
                "             approx: `{:?}`,\n",
                "          | a - b |: `{:?}`,\n",
                " | a - b | ≤ approx: false"
            ),
            stringify!($name),
            stringify!($name),
            $($arg)*
        )
    }
}

pub mod assert_approx_eq;
pub mod assert_approx_ne;
