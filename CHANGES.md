# Changes

Changes highlights for recent major versions.


## Version 9.0.0

* Add returns to many macros.

* Breaking change: rename macros from `assert_*_eq` into `assert_*_eq2`, then from `assert_*_eq_expr` into `assert_*_eq`.


## Version 8.x

* Add matches macros: `assert_matches`, `assert_not_matches`.

* Add Iterator macros: `assert_iter_all`, `assert_iter_any`, `assert_iter_eq2`, etc.

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
