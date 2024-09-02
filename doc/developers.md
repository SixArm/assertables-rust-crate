# Developers: how we write our test macros

We write each our test macros in three flavors:

* `assert_gt_as_result` returns a `Result` as `Ok` or `Err`. This macro contains all the logic, all the error formatting, etc. This macro is called by the other flavors below. This macros is also useful for runtime checks, such as when you want to know success or failure, yet you don't want to panic.

* `assert_gt` returns `()` or panics. This is the typical macro that most developers will use for testing. This macro wraps `assert_gt_as_result`. This macro provides two arms: one arm is for returning the error messsage as is, and one arm is for returning a developer's custom error message.

* `debug_assert_gt` return `()` or panics. This macro's statements are only enabled in non-optimized builds by default. An optimized build will not execute this macro's statements unless `-C debug-assertions` is passed to the compiler.

The sections below show each of the three flavors, using our simplest macro group: `assert_gt_as_result`, `assert_gt`, `debug_assert_gt`.


## assert_gt_as_result

The macro `assert_gt_as_result` returns a `Result` as `Ok` or `Err`.

The macro contains all the logic and all the error formatting.

The macro is called by the other flavors: directly by `assert_gt` and indirectly by `debug_assert_gt`.

The macros is useful for runtime checks, such as when you want to know success or failure, yet you don't want to panic.

Code:

```rust
#[macro_export]
macro_rules! assert_gt_as_result {
    ($a:expr, $b:expr $(,)?) => ({
        match (&$a, &$b) {
            (a_val, b_val) => {
                if a_val > b_val {
                    Ok(())
                } else {
                    Err(format!(
                        concat!(
                            "assertion failed: `assert_gt!(left, right)`\n",
                            "  left label: `{}`,\n",
                            "  left debug: `{:?}`,\n",
                            " right label: `{}`,\n",
                            " right debug: `{:?}`,\n",
                            "        left: `{:?}`,\n",
                            "       right: `{:?}`"
                        ),
                        stringify!($a), $a,
                        stringify!($b), $b,
                        a_val,
                        b_val
                    ))
                }
            }
        }
    });
}
```


## assert_gt

The macro `assert_gt` returns `()` or panics.

The macro is the typical macro that most developers will use for testing.

The macro wraps `assert_gt_as_result`.

The macro provides two arms: one arm is for returning the error messsage as is, and one arm is for returning a developer's custom error message.

Code:

```rust
#[macro_export]
macro_rules! assert_gt {
    ($a:expr, $b:expr $(,)?) => ({
        match assert_gt_result!($a, $b) {
            Ok(()) => (),
            Err(err) => panic!("{}", err),
        }
    });
    ($a:expr, $b:expr, $($message:tt)+) => ({
        match assert_gt!($a, $b) {
            Ok(()) => (),
            Err(_err) => panic!("{}", $($message)+),
        }
    });
}
```


## debug_assert_gt

The macro `debug_assert_gt` return `()` or panics.

The macro is the typical macro that most developers will use for runtime debugging during development, and possibly also for runtime debugging during production.

The macro's statements are only enabled in non-optimized builds by default. An optimized build will not execute this macro's statements unless `-C debug-assertions` is passed to the compiler.

The macro wraps `assert_gt`.

Code:

```rust
#[macro_export]
macro_rules! debug_assert_gt {
    ($($arg:tt)*) => {
        if $crate::cfg!(debug_assertions) {
            $crate::assert_gt!($($arg)*);
        }
    };
}
```