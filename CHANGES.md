# Changes


## 4.4 summary

Add macros for commands with output matching of patterns: `assert_command_stdout_regex`, `assert_command_stderr_regex`, etc.


## 4.3 summary

Add macros for commands with output substrings: `assert_command_stdout_contains_str`, `assert_command_stderr_contains_str`, etc.


## 4.2 summary

Add macros for commands with output strings: `assert_command_stdout_eq_str`, `assert_command_stderr_eq_str`, etc.


## 4.1 summary

Add macros for commands: `assert_command_stdout_eq`, `assert_command_stderr_eq`, etc.


## 4.0 summary

* Add macros for sets using `subset`, `superset`, `joint`, `disjoint`.

* Add macros for bags using `subbag`, `superbag`.

* Add macros for readers using `read_to_string`.

* Add quality by improving doc tests and unit tests.

* Rename function macros from "fn" to "f" because it prevents keyword conflict.

* Rename macros from `assume` to `assertable` because it's easier to understand.

* Retire macros with `assure` because they're easy to implement by using if/then.

* Upgrade Rust version from 2018 to 2021.
