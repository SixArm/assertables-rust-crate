# Upgrade from version 8 to 9

A naming convention is changing, to greatly improve usability.

For macros that compare two items to each other:

* From `assert_foo_eq`  into `assert_foo_eq2`. 

For macros that compare one item to an expression:

* From `assert_foo_eq_expr` into `assert_foo_eq`

This is a breaking change, and you may need to change your code.


## How to automate the naming convention changes

To upgrade your code, one way is to do a regular expression search and replace.

Search:

```txt
\b(|debug_)(assert_\w*_)(eq|ne|lt|le|gt|ge)(|_as_result)\b
```

Replace:

```txt
$1$2$32$4
```

Search:

```txt
\b(|debug_)(assert_\w*_)(eq|ne|lt|le|gt|ge)_expr(|_as_result)\b
```

Replace:

```txt
$1$2$3$4
```

