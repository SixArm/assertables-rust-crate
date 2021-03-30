# SixArm.com assert macros for Rust tests

We use these assertions in our client projects.


## assert_set_eq!

Example:

```rust
let a = vec![1, 2, 3];
let b = vec![3, 2, 1];
assert_set_eq!(a, b);
```


## assert_set_ne!

Example:

```rust
let a = vec![1, 2, 3];
let b = vec![4, 5, 6];
assert_set_ne!(a, b;
```
