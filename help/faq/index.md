# Frequently asked questions (FAQ)

Developers ask many questions about Assertables, Rust testing in general, macros in general, and more.


## What are the top benefits?

1. You can write better tests to improve reliability and maintainability.

2. You can handle more corner cases without needing to write custom code.

3. You can troubleshoot faster because error messages show specifics.


## What are the top features?
 
1. Easy to use: everything is well-documented with runnable examples.

2. More forms: for results, for solo/pair, for success return, etc.

3. Zero overhead: if you don't use a macro, then it's never compiled.


## Why use Assertables instead of just standard Rust macros?

Because Assertables gives you more kinds of tests, stronger semantics, clearer
naming, and better error messages.

Suppose you want to compare a file system text file to a string.

Assertables:

```rust
assert_fs_read_to_string_eq!("alfa.txt", "alfa\n");
```

Standard Rust:

```rust
assert!(fs::read_to_string("alfa.txt") == String::from("alfa\n));
```

The Assertables way is shorter and simpler to read, plus handles corner cases
such as the file not existing, plus provides better error messages.


## Why use Assertables instead of more_asserts, cool_asserts, assert2, claims, etc.?

1. Because Assertables provides all the macros (and more) in all those crates
   (and more).

2. Because Assertables has macro forms that return data upon success, which
   enables you to do subsequent tests. This is especially helpful for asserts
   that use Result or Option, as well as for asserts that make external calls
   such as reading files or running commands.

3. Because Assertables has macro forms that return a result, rather than panic.
   This enables production runtime uses such as for validations, verifications,
   live resilience monitoring, chaos engineering, and more.

## Are there license reasons to use Assertables instead of other assertion crates?

Yes, for some developers and organizations. 

Assertables provides more license choices than each of those crates. Assertables
provides Apache, MIT, GPL, BSD, and custom licenses for custom needs. See the
LICENSE file for specifics.


## Are there assertion crates that are good to use with Assertables?

Yes. There are assertion crates that provide even more functionality, especially
for more-sophisticated needs.

1. The crate `static_assertions` provides static code analysis, even before running `cargo test`.

2. The crate `assert_matches` provides matching macros that have even more functionality than Assertables macros `assert_matches` and `assert_not_matches`.
   
3. The crate `approx` provides floating point approximation macros that provide even more functionality than Assertables macros `assert_approx_eq`, `assert_in_delta`, `assert_in_epsilon`, etc.
