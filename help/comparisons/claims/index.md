# Comparisons: claims and similar crates

There are various Rust crates that provide assert macros:

* [`claims`](https://crates.io/crates/claims)

* [`claim`](https://crates.io/crates/claim) which was forked to create `claims`.

Each of these crates are doing the same surface-level thing as Assertables, by adding new assert macros.

Assertables has two major differences:

* More assertions

* More logic leverage


## Assertables has more assertions

If there's an assertion from any of those crates that you would like us to add to Assertables, then let us know, or create a merge request, and we'll add it.

| Category  | Assertables | claims & claim |
|---------|--------|-------------|
| Version | 8.3 |  0.7 |
| Updated | 2024 | 2022 |
| Compare  | [`assert_lt`](https://docs.rs/assertables/latest/assertables/assert_lt) <br> [`assert_le`](https://docs.rs/assertables/latest/assertables/assert_le) <br> [`assert_gt`](https://docs.rs/assertables/latest/assertables/assert_gt) <br> [`assert_ge`](https://docs.rs/assertables/latest/assertables/assert_ge) | `assert_lt` <br> `assert_le` <br> `assert_gt` <br> `assert_ge` |
| Nearness  | [`assert_in_delta`](https://docs.rs/assertables/latest/assertables/assert_in_delta) <br> [`assert_in_epsilon`](https://docs.rs/assertables/latest/assertables/assert_in_epsilon) | - <br> - |
| Match    | [`assert_is_match`](https://docs.rs/assertables/latest/assertables/assert_is_match) <br> [`assert_not_match`](https://docs.rs/assertables/latest/assertables/assert_not_match) | `assert_matches` <br> - |
| Contains | [`assert_contains`](https://docs.rs/assertables/latest/assertables/assert_contains) <br> [`assert_not_contains`](https://docs.rs/assertables/latest/assertables/assert_not_contains) | - <br> - |
| Starts/Ends | [`assert_starts_with`](https://docs.rs/assertables/latest/assertables/assert_starts_with) <br> [`assert_not_starts_with`](https://docs.rs/assertables/latest/assertables/assert_not_starts_with) <br> [`assert_ends_with`](https://docs.rs/assertables/latest/assertables/assert_ends_with) <br> [`assert_not_ends_with`](https://docs.rs/assertables/latest/assertables/assert_not_ends_with) | - <br> - <br> - <br> - |
| Result  | [`assert_result_ok`](https://docs.rs/assertables/latest/assertables/assert_result/assert_result_ok) <br> [`assert_result_ok_eq`](https://docs.rs/assertables/latest/assertables/assert_result/assert_result_ok_eq) <br> [`assert_result_err`](https://docs.rs/assertables/latest/assertables/assert_result/assert_result_err) | `assert_result_ok` <br> `assert_result_ok_eq` <br> `assert_result_err` |
| Option  | [`assert_option_some`](https://docs.rs/assertables/latest/assertables/assert_option/assert_option_some) <br> [`assert_option_some_eq`](https://docs.rs/assertables/latest/assertables/assert_option/assert_option_some_eq) <br> [`assert_option_none`](https://docs.rs/assertables/latest/assertables/assert_option/assert_option_none) | `assert_option_some` <br> `assert_option_some_eq` <br> `assert_option_none` |
| Poll    | [`assert_poll_ready`](https://docs.rs/assertables/latest/assertables/assert_poll/assert_poll_ready) <br>  [`assert_poll_ready_eq`](https://docs.rs/assertables/latest/assertables/assert_poll/assert_poll_ready_eq)  <br> `todo` <br> `todo` <br> [`assert_poll_pending`](https://docs.rs/assertables/latest/assertables/assert_poll/assert_poll_pending) | `assert_ready` <br> `assert_ready_eq` <br> `assert_ready_ok` <br> `assert_ready_err` <br>  `assert_pending` |
| Readers | [`assert_fs_read_to_string_*`](https://docs.rs/assertables/latest/assertables/assert_fs_read_to_string) <br> [`assert_io_read_to_string_*`](https://docs.rs/assertables/latest/assertables/assert_io_read_to_string) | - <br> - |
| Commands | [`assert_command_*`](https://docs.rs/assertables/latest/assertables/assert_command) <br> [`assert_program_args_*`](https://docs.rs/assertables/latest/assertables/assert_program_args) | - <br> - |
| Collections | [`assert_set_*`](https://docs.rs/assertables/latest/assertables/assert_set) <br> [`assert_bag_*`](https://docs.rs/assertables/latest/assertables/assert_bag) | - <br> - |
| Functions | [`assert_fn_*`](https://docs.rs/assertables/latest/assertables/assert_fn) <br> [`assert_fn_ok_*`](https://docs.rs/assertables/latest/assertables/assert_fn_ok) <br> [`assert_fn_err_*`](https://docs.rs/assertables/latest/assertables/assert_fn_err) | - <br> - |


## Assertables has more logic leverage

Assertables makes deliberate design decisions to implement each concept as three macros:

* The logic macro. This returns a Result and is the most important of the three macros.

* The panic macro. This is what a typical cargo test uses.

* The debug macro. This is what a typical runtime debug config uses.

Assertables puts all the logic in the logic macro, and developers can use the same logic anywhere they want, even for totally different purposes:

* Runtime production analysis using a Result. This works without triggering a panic, and without needing any debug config.

* Chaos engineering where logic macros can detect dirty input, or missing files, or bad data. This well for UI interactions with users, with fallback files, and with data sanitization.

* Custom macro wrapping where developers prefer to write their own syntax for their tests. This works well because the new syntax is just a surface-level addition, and can delegate to the logic macro.


## Compare a macro with various implementations

You can see the difference for yourself, such as in these two source code files:

* [`assert_gt`](https://github.com/SixArm/assertables-rust-crate/blob/main/src/assert_gt.rs) by Assertables.

* [`assert_gt`](https://crates.io/crates/rust-claim) by rust-claim.

You can see `assertables` provides three macros:

* The logic macro is `assert_gt_as_result`. It does the comparison and returns a Result and possible error message.

* The panic macro is `assert_gt`. It is a thin wrapper.

* The debug macro is `debug_assert_gt`. It is a thin wrapper.

You can see `claim` provides two macros:

* The panic macro is `assert_gt`. It contains the logic, which means the logic can't be reused independently.

* The debug macro is `debug_assert_gt`. It is a thin wrapper.
