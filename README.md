<!-- markdownlint-disable-next-line -->
<div align="center">

# `srch`

**Text Search For Humans**

[![Crate](https://img.shields.io/crates/v/srch.svg)](https://crates.io/crates/srch)
[![Docs](https://img.shields.io/badge/docs-latest-blue.svg)](https://docs.rs/srch)

</div>

`srch` is a cli to run text expressions and perform basic text operations such
as filtering, ignoring and replacing on the command line. There are many great
tools that do this job. But most other tools have one in common: They are hard
to memorize if you dont use them regularly. `srch` tries to solve this issue by
providing a super simple cli & expression language which can be easily
memorized and is well documented.

## Quickstart

```
$ srch for 'equals "foobar"' -m word                # matches all occurences `foobar` in the text
$ srch for 'length 20'                              # matches all lines with 20 chars
$ srch not 'numeric or special'                     # ignores all lines which contain only numbers and special chars
$ srch replace 'numeric and length 5' 12345 -m word # replaces all 5 digit numbers with `12345`
```

## Common tasks where `srch` excels `grep` in readability

| Task                                                    | `srch`                                                                   | `grep`                                                                                         |
| ------------------------------------------------------- | ------------------------------------------------------------------------ | ---------------------------------------------------------------------------------------------- |
| Find all words containing a string                      | `srch for 'contains "substr"' -m word`                                   | `grep -oh "\w*substr\w*"`                                                                      |
| Find all lines in a file with a specific length         | `srch for 'length 10'`                                                   | `grep -x '.\{10\}'`                                                                            |
| Ignore all lines containing a string                    | `srch not 'contains "hide me"'`                                          | `grep -v "hide me"`                                                                            |
| Replacing all words following a specific pattern        | `srch replace 'numeric and length 5' 12345 -m word`                      | `grep` itself cant replace, you need to use `sed` for that (which gets even more complicated). |
| Replacing all email addresses in a file with your email | `srch replace 'contains "@" and contains ".com"' your@email.com -m word` | Same as above.                                                                                 |

## When to use other tools

As said earlier: `srch` is no direct competitor to `grep`, `awk`, etc.! If you
find yourself reaching the limits of the text expression language, you probably
want to use more advanced tools.

# Installing

At the moment `srch` can be installed only via `cargo` using:

```
$ cargo install srch
```

# Documentation

There are the following global options:

- `-m` / `--mode`, sets the operation mode, can be either `line` or `word`,
  defaults to `line`

And there are the following global flags:

- `-f` / --first`, print only the first match if available
- `-l` / --last`, print only the last match if available
- `--skip n`, skip the first n matches
- `--limit n`, show at most n matches

```
srch for [FLAGS] [OPTIONS] <EXPRESSION> [FILE]
srch not [FLAGS] [OPTIONS] <EXPRESSION> [FILE]
srch replace [FLAGS] [OPTIONS] <EXPRESSION> <REPLACEMENT> [FILE]
```

If no file is provided `srch` tries to read from stdin.

## Examples

```
$ docker ps | srch for 'alphanumeric and length 12' -m word # prints all docker container ids
```

# The Text Expression Language

This is a super simple format of writing readable and easy to memorize text
processing expressions - there are many great and far more advanced languages
and tools to process text on the commandline out there but all of them have one
problem in common - they're unreadable and hard to memorize if not used often.

The Text Expression Languages provides only 9 Attributes to query by. These
attributes indicate the format of a string which gets tested against it.

| Attribute        | Resolve to true if the tested string           |
| ---------------- | ---------------------------------------------- |
| `starts <str>`   | starts with the given string                   |
| `ends <str>`     | ends with the given string                     |
| `contains <str>` | contains a substring equal to the given string |
| `equals <str>`   | exactly equals the given string                |
| `length <int>`   | has the given length                           |
| `numeric`        | contains only numeric chars                    |
| `alpha`          | contains only alphabetic chars                 |
| `alphanumeric`   | contains only alphanumeric chars               |
| `special`        | contains only special chars                    |

Currently there are only two binary logical operations: `and` and `or`

| Operator | Boolean Algebra |
| -------- | --------------- |
| `and`    | Conjunction     |
| `or`     | Disjunction     |

Attributes can be concattenated by logical operators.

## Examples

```
starts "FOO" and ends "BAR"
contains "@" and contains ".com"
length 5 and length 10
numeric and length 8
```

## Limitations

This Syntax might not cover all use cases. It's not meant to do that. If you
find yourself reaching the limits of this language you might want to use more
advanced tools (such as awk, grep, sed..)
