# Changes

Changes highlights for recent major versions.


## Version 8.x

* Add Result macros: `assert_ok`, `assert_err`, et al.
  
* Add Option macros: `assert_some`, `assert_none`, et al.

* Add Poll macros: `assert_ready`, `assert_pending`, et al.

* Add read macros: `assert_fs_read_to_string_*`, `assert_io_read_to_string_*`, et al. 
  
* Breaking change: migrate from `assert_read_to_string_*`. to `assert_io_read_to_string_*`.


## Version 7.x

* Add `assert_in_delta`, `assert_in_epsilon`.

* Add `assert_fn_*` macros with multiple arities.

* Add `cargo release` for optimized tagged releases.


## Version 6.x

* Add `assert_starts_with`, `assert_ends_with`, `assert_contains`, `assert_is_match`.

* Add `debug_assert_*` macros everywhere.

* Add `GPL-3.0` license.
