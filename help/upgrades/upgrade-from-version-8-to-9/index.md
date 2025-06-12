# Upgrade from version 8 to 9

An Assertables macro naming convention is changing, to make the macro names slightly shorter.

For macros that compare one item to an expression:

* From `assert_*_expr` into `assert_*_x`

This is a breaking change, and you may need to change your code.

## How to automate the naming convention changes

To upgrade your code, one way is to do a regular expression search and replace.

Search:

```txt
(assert_\w*?)_expr
```

Replace:

```txt
$1
```
