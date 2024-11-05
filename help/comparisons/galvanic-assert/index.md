# Galvanic-assert and Assertables

## Galvanic-assert

https://crates.io/crates/galvanic-assert

https://github.com/mindsbackyard/galvanic-assert

This crate provides a new assertion macros (assert_that!, expect_that!, get_expectation_for!) based on matching predicates (matchers).

The crate is part of galvanic---a complete test framework for Rust. The framework is shipped in three parts, so you can choose to use only the parts you need.

Each assertion has the form assert_that!(SOME_VALUE, MATCHES_SOMETHING);. To check if some value satisfies some matching predicate, e.g., less_than, contains_in_order, is_variant!, ...;

## Top Comparison

Galvanic-assert architecture is more complex than Assertables architecture:

* Galvanic-assert enables building your own kinds of structures for testing assertions.

* Assertables is deliberately simple, because each macro module stands on its own.
