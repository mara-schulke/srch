# rtp - Readable Text Proccessor

`rtp` is a small, well tested cli (and language), to easily execute basic text operations
(filtering, ignore, replace) on the command line. There are many great tools that do this job.
But most other tools have one in common: They are hard to memorize if you dont use them regularly.
`rtp` tries to solve this issue by providing a super simple cli & expression language which can
be easily memorized and is well documented.


## Common tasks where `rtp` excels `grep` in readability

| Task                                                    | `rtp`                                                              | `grep`                                                                                         |
|---------------------------------------------------------|--------------------------------------------------------------------|------------------------------------------------------------------------------------------------|
| Find all words containing a string                      | `rtp filter word 'contains "substr"'`                              | `grep -oh "\w*substr\w*"`                                                                      |
| Find all lines in a file with a specific length         | `rtp filter 'length 10'`                                           | `grep -x '.\{10\}'`                                                                            |                        |                              | `grep -oh "\w*substr\w*"` |
| Ignore all lines containing a string                    | `rtp ignore 'contains "hide me"'`                                  | `grep -v "hide me"`                                                                            |
| Replacing all words following a specific pattern        | `rtp replace word 'numeric & length 5' 12345`                      | `grep` itself cant replace, you need to use `sed` for that (which gets even more complicated). |
| Replacing all email addresses in a file with your email | `rtp replace word 'contains "@" & contains ".com"' your@email.com` | Same as above.                                                                                 |


## When to use other tools

As said earlier: `rtp` is no direct competitor to `grep`, `awk`, etc.! If you find yourself reaching the limits of the rtp expression language, you probably want to use more advanced tools. `rtp` 




```
rtp [operation] [mode] '<rtp-expression>' [flags] input output

rtp filter 'starts FOO or contains BLA' --json ./.env
rtp filter word 

rtp ignore word 'length 17'
rtp replace foo baz ./.env
```

All rtp commands support the (line/word) modifier - if none is specified line is used.

```
# both do the same thing
rtp [filter|ignore|replace]
rtp [filter|ignore|replace] line
```

```
rtp [filter|ignore|replace] [line/word] [starts|containis|ends|equals|email|digits|alpha|alphanum|length] [--last|--first|--nth|--odd|--even]
```

## The RTP Expression Language

This is a super simple format of writing readable and easy to memorize text processing expressions - there are many great and more advanced languages and tools to process text on the commandline out there but all of them have the same problem - they're unreadable and hard to memorize if not used often. The RTP Expression Language tries to solve this problem by providing only 2 Syntax Elements: Attributes and Logical concattenation.

Both are really easy to memorize, as for now, there are 8 Attributes to match:
- `starts`
- `ends`
- `contains`
- `equals`
- `numeric`
- `alpha`
- `alphanumeric`
- `length`

And 2 Logical Operators:
- `AND`
- `OR`

This Syntax might not cover all use cases. It's not meant to do that. If you find yourself reaching the limits of this language you might want to use more advanced tools (such as awk, grep, sed..)

So some examples are listed below:
```
starts 'FOO' AND ends 'BAR'
contains '@' AND contains '.com'
length 5 OR length 10
numeric AND length 8
```
