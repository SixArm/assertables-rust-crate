# Changes

Changes highlights for recent major versions.


## Version 8.x

8.6: Add `assert_poll_ready_eq`, `assert_poll_ready_ne`

8.5: Add `assert_option_some_eq`, `assert_option_some_ne`

8.4: Add `assert_result_ok_eq`, `assert_result_ok_ne`

8.3: Add `assert_poll_ready`, `assert_poll_pending`.

8.2: Add `assert_infix`.

8.1: Add `assert_result_ok`, `assert_result_err`, `assert_option_some`, `assert_option_none`.

8.0: Add `assert_fs_read_to_string_*`, `assert_io_read_to_string_*`. Breaking change: migrate from `assert_read_to_string_*`. to `assert_io_read_to_string_*`.


## Version 7.x

* Add `assert_in_delta`, `assert_in_epsilon`.

* Add `assert_fn_*` macros with multiple arities.

* Add `cargo release` for optimized tagged releases.


## Version 6.x

* Add `assert_starts_with`, `assert_ends_with`, `assert_contains`, `assert_is_match`.

* Add `debug_assert_*` macros everywhere.

* Add `GPL-3.0` license.
