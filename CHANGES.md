# Changes


## Version 5.0 summary

Upgrades:

* Add many new macro names with `other` in order to compare two items of the same type.

* Add many new macro parameters with `expr` in order to enable arbitratry expressions.

* Improve messages for collection macros `bag` and `set` so they preserve insertion order.

* Refactor code composability via new `msg` macros.

* Refactor code readability via new `concat` formats.

* Refactor code maintainability via `assert` macros that call `assert…result` macros.

Breakers:

  * Rename result macros from `assertable` to `assert…result` in order to improve learnability.

  * Rename function macros `f` to `fn` in order to improve learnability.

  * Rename string macros from `contains_str` to `contains` in order to add new containee capabilties.

  * Rename matcher macros from `is_match` to `matches` in order to add new matcher capabilties.
