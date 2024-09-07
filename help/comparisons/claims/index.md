# Comparisons: claims and similar crates

There are various Rust crates that provide assert macros:

* [`claims`](https://crates.io/crates/claims)

* [`claim`](https://crates.io/crates/claim)

* [`rust-claim`](https://crates.io/crates/rust-claim) which was forked to create `claims`.

Each of these crates are doing the same surface-level thing as Assertables, by adding new assert macros.

Assertables has two major differences:

* More assertions

* More logic leverage


## Assertables has more assertions

If there's an assertion from any of those crates that you would like us to add to Assertables, then let us know, or create a merge request, and we'll add it.

| Category  | `claims` | `assertables` |
|---------|--------|-------------|
| Version |  0.7 | 8.2 |
| Updated | 2022 | 2024 |
| Compare  | assert_ge <br> assert_gt <br> assert_le <br> assert_lt<br><br> | assert_ge <br> assert_gt <br> assert_le <br> assert_lt <br> assert_in_delta <br> assert_in_epsilon |
| Match    | assert_matches <br> | assert_is_match <br> assert_not_match |
| Contains | x | assert_contains <br> assert_not_contains |
| Starts/Ends | x | assert_starts_with <br> assert_not_starts_with <br> assert_ends_with <br> assert_not_ends_with |
| Result  | assert_result_ok <br> assert_result_err <br> assert_result_ok_eq | assert_result_ok <br> assert_result_err <br> TODO |
| Option  | assert_option_some <br> assert_option_none <br> assert_option_some_eq | assert_option_some <br> assert_option_none <br> todo |
| Poll    | assert_ready <br> assert_pending <br> assert_ready_ok<br> assert_ready_err <br> assert_ready_eq | assert_pending <br> assert_ready <br> todo <br> todo <br> todo |
| FS Path  | x | assert_fs_read_to_string_* |
| IO Reader  | x | assert_io_read_to_string_* |
| Command | x | assert_command_* <br> assert_program_args_* |
| Set     | x | assert_set_eq <br> assert_set_ne <br> assert_set_subset <br> assert_set_superset <br> assert_set_joint <br> assert_set_disjoint |
| Bag     | x | assert_bag_eq <br> assert_bag_ne<br> <br> assert_bag_subbag <br> assert_bag_superbag |
| Function | x | assert_fn_* |


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

You can see that Assertables provides three macros:

* The logic macro is `assert_gt_as_result`. It does the comparison and returns a Result and possible error message.

* The panic macro is `assert_gt`. It is a thin wrapper.

* The debug macro is `debug_assert_gt`. It is a thin wrapper.

You can see that rust-claim provides two macros:

* The panic macro is `assert_gt`. It contains the logic, which means the logic can't be reused independently.

* The debug macro is `debug_assert_gt`. It is a thin wrapper.
