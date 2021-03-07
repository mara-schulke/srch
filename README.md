# rtp - Readable Text Proccessor

`rtp` is a small, well tested cli (and language), to easily execute basic text operations
(filtering, ignore, replace) on the command line. There are many great tools that do this job.
But most other tools have one in common: They are hard to memorize if you dont use them regularly.
`rtp` tries to solve this issue by providing a super simple cli & expression language which can
be easily memorized and is well documented.

## Quickstart

```
rtp [operation] [mode] '<rtp-expression>' [flags] input output

rtp filter 'starts FOO or contains BLA' --json ./.env
rtp filter word 

rtp ignore word 'length 17'
rtp replace foo baz ./.env
```

## Common tasks where `rtp` excels `grep` in readability

| Task                                                    | `rtp`                                                              | `grep`                                                                                         |
|---------------------------------------------------------|--------------------------------------------------------------------|------------------------------------------------------------------------------------------------|
| Find all words containing a string                      | `rtp filter word 'contains "substr"'`                              | `grep -oh "\w*substr\w*"`                                                                      |
| Find all lines in a file with a specific length         | `rtp filter 'length 10'`                                           | `grep -x '.\{10\}'`                                                                            |                        |                              | `grep -oh "\w*substr\w*"` |
| Ignore all lines containing a string                    | `rtp ignore 'contains "hide me"'`                                  | `grep -v "hide me"`                                                                            |
| Replacing all words following a specific pattern        | `rtp replace word 'numeric and length 5' 12345`                      | `grep` itself cant replace, you need to use `sed` for that (which gets even more complicated). |
| Replacing all email addresses in a file with your email | `rtp replace word 'contains "@" and contains ".com"' your@email.com` | Same as above.                                                                                 |


## When to use other tools

As said earlier: `rtp` is no direct competitor to `grep`, `awk`, etc.! If you find yourself reaching the limits of the rtp expression language, you probably want to use more advanced tools. `rtp` 



# Documentation

All rtp commands support the (line/word) modifier - if none is specified line is used.

```
# both do the same thing
rtp [filter|ignore|replace]
rtp [filter|ignore|replace] line
```

```
rtp [filter|ignore|replace] [line/word] [starts|containis|ends|equals|email|digits|alpha|alphanum|length] [--last|--first|--nth|--odd|--even]
```

# The RTP Expression Language

This is a super simple format of writing readable and easy to memorize text processing expressions - there are many great and far more advanced languages and tools to process text on the commandline out there but all of them have one problem in common - they're unreadable and hard to memorize if not used often.

The RTP Expression Languages provides only 9 Attributes to query by. These attributes indicate the format of a string which gets tested against it.

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

Currently there are only two binary logical operations: `and` and `or`

| Operator | Boolean Algebra |
|----------|-----------------|
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

This Syntax might not cover all use cases. It's not meant to do that. If you find yourself reaching the limits of this language you might want to use more advanced tools (such as awk, grep, sed..)
