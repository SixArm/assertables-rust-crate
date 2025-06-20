// By https://github.com/pdh11

use assertables::*;
use std::collections::HashSet;

#[test]
fn test() {
    let haystack: HashSet<String> = [String::from("a")].into();
    let needle: &str = "a";
    assert!(haystack.contains(needle));
    assert_contains!(haystack, needle); // Skip & because needle type is &str which is already a reference
}
