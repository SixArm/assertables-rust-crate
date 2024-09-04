use assertables::*;
use regex::*;

#[test]
fn smoke_test_top() {
    assert_eq!(1, 1);
    assert_ge!(2, 1);
    assert_gt!(2, 1);
    assert_le!(1, 2);
    assert_lt!(1, 2);
    assert_ne!(2, 1);

    // Nearness
    assert_in_delta!(1, 1, 0);
    assert_in_epsilon!(1, 1, 0);

    // Inners
    assert_starts_with!("alfa", "al");
    assert_ends_with!("alfa", "fa");
    assert_contains!("alfa", "lf");
    assert_is_match!(Regex::new(r"lf").unwrap(), "alfa");
}

#[test]
fn smoke_test_bag() {
    assert_bag_eq!([1], [1]);
    assert_bag_ne!([1], [2]);
    assert_bag_subbag!([1], [1, 2]);
    assert_bag_superbag!([1, 2], [1]);
}

#[test]
fn smoke_test_set() {
    assert_set_eq!([1], [1]);
    assert_set_ne!([1], [2]);
    assert_set_subset!([1], [1, 2]);
    assert_set_superset!([1, 2], [1]);
    assert_set_joint!([1], [1]);
    assert_set_disjoint!([1], [2]);
}
