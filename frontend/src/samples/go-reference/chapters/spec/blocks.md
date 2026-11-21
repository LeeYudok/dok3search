# Blocks

A *block* is a possibly empty sequence of declarations and statements within matching brace brackets.

``` ebnf
Block         = "{" StatementList "}" .
StatementList = { Statement ";" } .
```

In addition to explicit blocks in the source code, there are implicit blocks:

1.  The *universe block* encompasses all Go source text.
2.  Each [package](packages.md) has a *package block* containing all Go source text for that package.
3.  Each file has a *file block* containing all Go source text in that file.
4.  Each ["if"](statements.md#if-statements), ["for"](statements.md#for-statements), and ["switch"](statements.md#switch-statements) statement is considered to be in its own implicit block.
5.  Each clause in a ["switch"](statements.md#switch-statements) or ["select"](statements.md#select-statements) statement acts as an implicit block.

Blocks nest and influence [scoping](declarations-and-scope.md).
