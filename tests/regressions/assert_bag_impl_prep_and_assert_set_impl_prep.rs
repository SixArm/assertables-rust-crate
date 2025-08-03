//! Discoverd by https://github.com/DenWav
//!
//! The macro `assert_bag_eq` and `assert_set_eq` and their siblings depend on
//! the macros `assert_bag_impl_prep` and `assert_set_impl_prep` which are
//! implementation preparation macros that convert inputs to bags and sets.
//!
//! This regression test checks each macro to ensure it runs even when it is
//! referred to by explicit path rather than being included by a star pattern.
//!
//! The correct code runs the implementation preparation macro via a full path:
//!
//! ```no_run
//! let a = $crate::assert_bag_impl_prep!(a_collection);
//! ```
//!
//! The incorrect code used a relative path:
//!
//! ```no_run
//! let a = assert_bag_impl_prep!(a_collection);
//! ```

#[test]
fn test_assert_bag_eq() {
    let a = vec![1, 2];
    let b = vec![2, 1];
    ::assertables::assert_bag_eq!(a, b);
}

#[test]
fn test_assert_bag_ne() {
    let a = vec![1, 1];
    let b = vec![2, 2];
    ::assertables::assert_bag_ne!(a, b);
}

#[test]
fn test_assert_bag_subbag() {
    let a = vec![1];
    let b = vec![1, 1];
    ::assertables::assert_bag_subbag!(a, b);
}

#[test]
fn test_assert_bag_superbag() {
    let a = vec![1, 1];
    let b = vec![1];
    ::assertables::assert_bag_superbag!(a, b);
}

#[test]
fn test_assert_set_eq() {
    let a = vec![1, 2];
    let b = vec![2, 1];
    ::assertables::assert_set_eq!(a, b);
}

#[test]
fn test_assert_set_ne() {
    let a = vec![1, 1];
    let b = vec![1, 2];
    ::assertables::assert_set_ne!(a, b);
}

#[test]
fn test_assert_set_subset() {
    let a = vec![2, 1];
    let b = vec![1, 2, 3];
    ::assertables::assert_set_subset!(a, b);
}

#[test]
fn test_assert_set_superset() {
    let a = vec![1, 2, 3];
    let b = vec![2, 1];
    ::assertables::assert_set_superset!(a, b);
}

#[test]
fn test_assert_set_joint() {
    let a = vec![1, 2];
    let b = vec![2, 3];
    ::assertables::assert_set_joint!(a, b);
}

#[test]
fn test_assert_set_disjoint() {
    let a = vec![1, 2];
    let b = vec![3, 4];
    ::assertables::assert_set_disjoint!(a, b);
}
