# Comparisons: approx

https://crates.io/crates/approx

https://github.com/brendanzab/approx

Approximate floating point equality comparisons and assertions for the Rust Programming Language.

Provides these macros with various options:

* abs_diff_eq!(a, b, options)
* relative_eq!(a, b, options)
* ulps_eq!(a, b, options)

Assertables has some basic equivalents:

* assert_approx_eq!(a, b)
* assert_approx_ne!(a, b)
* assert_in_delta(a, b, delta)
* assert_in_epsilon(a, b, epsilon)

For developers who want general kinds of tests, and also want specific floating point comparison tests, we recommend using both `assertables` and `approx`.
