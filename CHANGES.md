# Changes


## 4.x summary

* 4.0: Add macros for sets using `subset`, `superset`, `joint`, `disjoint`.

* 4.0: Add macros for bags using `subbag`, `superbag`.

* 4.0: Add macros for readers using `read_to_string`.

* 4.0: Add quality by improving doc tests and unit tests.

* 4.0: Rename function macros from "fn" to "f" because it prevents keyword conflict.

* 4.0: Rename macros from `assume` to `assertable` because it's easier to understand.

* 4.0: Retire macros with `assure` because they're easy to implement by using if/then.

* 4.0: Upgrade Rust version from 2018 to 2021.

* 4.1: Add macros for commands: `assert_command_stdout_eq`, `assert_command_stderr_eq`, etc.

* 4.2: Add macros for commands with output strings: `assert_command_stdout_eq_str`, `assert_command_stderr_eq_str`, etc.

* 4.3: Add macros for commands with output substrings: `assert_command_stdout_contains_str`, `assert_command_stderr_contains_str`, etc.

* 4.4: Add macros for commands with output matching of patterns: `assert_command_stdout_regex`, `assert_command_stderr_regex`, etc.






