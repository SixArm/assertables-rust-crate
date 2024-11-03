# Comparisons: assert_cmd and Assertables

https://crates.io/crates/assert_cmd

https://github.com/assert-rs/assert_cmd

The `assert_cmd` crate aims to simplify the process for doing integration testing of CLIs, including finding your crate's binary to test, and doing an assert on the result of your program's run.

For the examples below, suppose you have a command, such as:

```rust
let cmd = Command::new("foo");
```

## Test for success

Example of `assert_cmd`:

```rust
cmd.unwrap().assert().success();
```

Example of `asssertables`:

```rust
assert_status_success(cmd.unwrap());
```

## Test for code

Example of `assert_cmd`:

```rust
cmd.unwrap().assert().code(predicate::eq(42));
```

Example of `asssertables`:

```rust
assert_status_code_eq_x(cmd, 42);
```

## Extras

The crate has some macros for doing assertions on the result of a command:

```rust
append_context("main", "no args")
```

## Top comparison

The `assert_cmd` crate simplifies testing commands, because you can attach assertions to the command output, and also attach context to the assertion. The syntax is different than standard Rust assert macros.

The `assertables` crate simplifies testing commands with more macros. Each macro module is its own assertion.
The syntax is the same as standard Rust assert macros.
