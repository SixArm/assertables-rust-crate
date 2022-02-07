# Changes

## 4.0.0 summary

Add capabilities for sets using `subset`, `superset`, `joint`, `disjoint`.

Add capabilities for bags using `subbag`, `superbag`.

Add capabilities for readers using `read_to_string`.

Add quality by improving doc tests and unit tests.

Rename function macros from "fn" to "f" because it prevents keyword conflict.

Rename macros from `assume` to `assertable` because it's easier to understand.

Retire macros with `assure` because they're easy to implement by using if/then.

Upgrade Rust version from 2018 to 2021.
