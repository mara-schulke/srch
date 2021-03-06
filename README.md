# rtp

Readable Text Proccessor

rtp is no competitor to grep - it doesnt support advanced regex features. Instead of implementing advanced regex features, which grep already does great, rtp tries to make the basic text processing operations (filtering, ignore, replace) as easy as possible

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

This is a super simple format of writing readable and memorizable text processing expressions - there are many great and more advanced languages and tools to process text on the commandline out there but all of them have the same problem - they're unreadable and hard to memorize if not used often. The RTP Expression Language tries to solve this problem by providing only 2 Syntax Elements: Attributes and Logical concattenation.

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
