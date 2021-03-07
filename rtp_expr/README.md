# The RTP Expression Language

This is a crate contains a parser, execution engine and specification for a super simple language to write readable and memorizable text processing expressions.

This document should be used as specification for language to clarify the development of this package.

## Data Types

This Language holds only two data types; strings and integers. Both are used as literals in arguments of attribute statements.

### str

A `str` must use double quotes. So `"foo"` and `"!=$)(j0j0802"` are both valid strings. If a double quote occurs inside the string, it can be
escaped using `\`. So strings with escaped double quotes look like this: `"\"Quoted Text\""`. This is actually all there is to strings. They cant contain
any other escaped chars (such as new lines etc.)

### int

An `ìnt` is either a single `0` or `1-9` followed by `0-9` as often as wanted. There are no signed integers.

## Queries

Queries indicate the format of a string which gets tested against it. Currently there are 9 Attributes which are specified:

| Attribute        | Resolve to true if the tested string           |
|------------------|------------------------------------------------|
| `starts <str>`   | starts with the given string                   |
| `ends <str>`     | ends with the given string                     |
| `contains <str>` | contains a substring equal to the given string |
| `equals <str>`   | exactly equals the given string                |
| `length <int>`   | has the given length                           |
| `numeric`        | contains only numeric chars                    |
| `alpha`          | contains only alphabetic chars                 |
| `alphanumeric`   | contains only alphanumeric chars               |
| `special`        | contains only special chars                    |

## Logical Operators

Currently there are only two binary logical operations: `and` and `or`

| Operator | Precedence | Associativity |
|----------|------------|---------------|
| `and`    | 2          | Right         |
| `or`     | 1          | Right         |

### Examples

`1 or 2` parses as `(1 or 2)`

`1 and 2` parses as `(1 and 2)`

`1 and 2 or 3` parses as `((1 and 2) or 3)`

`1 or 2 and 3` parses as `(1 or (2 and 3))`

`1 and 2 and 3` parses as `(1 and (2 and 3))`

`1 or 2 or 3` parses as `(1 or (2 or 3))`

`1 or 2 or 3 and 4 or 5` parses as `(1 or (2 or ((3 and 4) or 5)))))`

## Groups

> Currently not implemented. May come in future versions to enable more complex text processing;
