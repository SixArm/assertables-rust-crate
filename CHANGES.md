# Changes

Changes as highlights for recent major versions.


## Version 9.x

* 9.6.1 Fix pass-by-reference tests by adding loop.

* 9.6.0 Add `assert_email_address`
  
* 9.5.6 Fix idempotent access testing via once: `assert_count/`, `assert_len/`, `assert_bag/`, `assert_set/`, `assert_in/`, `assert_is_empty/`, `assert_is_match/`, `assert_err/`, `assert_starts_with/`, `assert_ends_with/`,
`assert_program_args_*/`.

* 9.5.5 Fix documentation link version number also in README.

* 9.5.4 Fix documentation link version number.

* 9.5.3 Fix idempotent access testing via once: `assert_{lt,le,gt,ge,ne}/`, `assert_in_{delta,epsilon,range}/`, `assert_abs_diff_*/`, `assert_diff_*/`, `assert_approx_*/`.

* 9.5.2 Fix idempotent access: `assert_ok/`, `assert_err/`.

* 9.5.1 Fix assert_in_range for range syntax `a..=b`.

* 9.5.0 Add difference comparison macros: `assert_diff_*_x`, `assert_abs_diff_*_x`.

* 9.4.0 Add `assert_in`, `assert_in_range`.

* 9.3.0 Add status macros: `assert_status_success`,  `assert_status_success_false`.
ge

* 9.2.0 Add status comparison macros: `assert_status_code_value*`.

* 9.1.0 Add absolute difference macros: `assert_abs_diff*`.

* 9.0.0 Breaking change: many macros now return data upon success.

* 9.0.0 Breaking change: rename macros from `assert_*_expr` into `assert_*_x`.


## Version 8.x

* Add matches macros: `assert_matches`, `assert_not_matches`.

* Add iterator macros: `assert_iter_all`, `assert_iter_any`, `assert_iter_eq`, etc.

* Add result macros: `assert_ok`, `assert_err`, etc.

* Add option macros: `assert_some`, `assert_none`, etc.

* Add poll macros: `assert_ready`, `assert_pending`, etc.

* Add list macros: `assert_len`, `assert_count`, `assert_is_empty`, etc.

* Add read macros: `assert_fs_read_to_string`, `assert_io_read_to_string`, etc.

* Add approx macros: `assert_approx_eq`, `assert_approx_ne`.

* Breaking change: change from `assert_read_to_string_*`. to `assert_io_read_to_string_*`.


## Version 7.x

* Add `assert_in_delta`, `assert_in_epsilon`.

* Add `assert_fn_*` macros with multiple arities.

* Add `cargo release` for optimized tagged releases.


## Version 6.x

* Add `assert_starts_with`, `assert_ends_with`, `assert_contains`, `assert_is_match`.

* Add `debug_assert_*` macros everywhere.

* Add `GPL-3.0` license.
