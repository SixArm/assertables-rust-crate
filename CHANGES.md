# Changes

Changes highlights for recent major versions.


## Version 9.x

* 9.2.0: Add process macros: `assert_process_status_code_value*`.

* 9.1.0: Add absolute difference macros: `assert_abs_diff*`.

* 9.0.0: Breaking change: many macros now return data upon success.

* 9.0.0: Breaking change: rename macros from `assert_*_expr` into `assert_*_x`.


## Version 8.x

* Add matches macros: `assert_matches`, `assert_not_matches`.

* Add Iterator macros: `assert_iter_all`, `assert_iter_any`, `assert_iter_eq`, etc.

* Add Result macros: `assert_ok`, `assert_err`, etc.

* Add Option macros: `assert_some`, `assert_none`, etc.

* Add Poll macros: `assert_ready`, `assert_pending`, etc.

* Add length & count macros: `assert_len`, `assert_count`, `assert_is_empty`, etc.

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
