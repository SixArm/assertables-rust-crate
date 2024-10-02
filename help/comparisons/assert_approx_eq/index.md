# Comparisons: assert_approx_eq

https://crates.io/crates/assert_approx_eq

The `assert_approx_eq` crate provides the `assert_approx_eq` macro. The `assertables` crate has a deliberately-similar macro because we want to make it easy to migrate.

Both crates have the same syntax for a comparison that uses the default delta:

```rust
assert_approx_eq!(a, b); // default delta is 1.0e-6
```

The `assert_approx_eq` crate uses an optional delta such as:

```rust
assert_approx_eq!(a, b, 1); // delta is 1
```

The `assertables` crate uses an explicit macro name with a delta:

```rust
assert_in_delta!(a, b, 1); // delta is 1
```

We prefer the explicit macro name, rather than a form with an optional delta
