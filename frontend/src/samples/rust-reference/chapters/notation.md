# Notation

[notation](#r-notation)

# Notation

[notation.grammar](#r-notation.grammar)

## Grammar

[notation.grammar.syntax](#r-notation.grammar.syntax)

The following notations are used by the *Lexer* and *Syntax* grammar snippets:

| Notation | Examples | Meaning |
|----|----|----|
| CAPITAL | KW_IF, INTEGER_LITERAL | A token produced by the lexer |
| *ItalicCamelCase* | *LetStatement*, *Item* | A syntactical production |
| `string` | `x`, `while`, `*` | The exact character(s) |
| x? | `pub`? | An optional item |
| x\* | *OuterAttribute*\* | 0 or more of x |
| x+ | *MacroMatch*+ | 1 or more of x |
| xa..b | HEX_DIGIT1..6 | a to b repetitions of x, exclusive of b |
| xa..=b | HEX_DIGIT1..=5 | a to b repetitions of x, inclusive of b |
| xn:a..=b | `#`n:1..=255 | a to b repetitions of x (inclusive of b), with the count bound to the name n |
| xn | `#`n | x repeated the number of times bound to n by a previous labeled repetition |
| Rule1 Rule2 | `fn` *Name* *Parameters* | Sequence of rules in order |
| \| | `u8` \| `u16`, Block \| Item | Either one or another |
| ! | !COMMENT | Matches if the expression does not follow, without consuming any input |
| \[ \] | \[`b` `B`\] | Any of the characters listed |
| \[ - \] | \[`a`-`z`\] | Any of the characters in the range |
| ~\[ \] | ~\[`b` `B`\] | Any characters, except those listed |
| ~`string` | ~`\n`, ~`*/` | Any characters, except this sequence |
| ( ) | (`,` *Parameter*)? | Groups items |
| ^ | `b'` ^ ASCII_FOR_CHAR | The rest of the sequence must match or parsing fails unconditionally ([hard cut operator](#the-hard-cut-operator)) |
| U+xxxx..xxxxxx | U+0060 | A single Unicode character |
| \ | \ | An English description of what should be matched |
| Rule suffix | IDENTIFIER_OR_KEYWORD *except `crate`* | A modification to the previous rule |
| // Comment. | // Single line comment. | A comment extending to the end of the line. |

Sequences have a higher precedence than `|` alternation.

[notation.grammar.cut](#r-notation.grammar.cut)

### The hard cut operator

The grammar uses ordered alternation: the parser tries alternatives left to right and takes the first that matches. If an alternative fails partway through a sequence, the parser normally backtracks and tries the next alternative. The cut operator (`^`) prevents this. Once every expression to the left of `^` in a sequence has matched, the rest of the sequence must match or parsing fails unconditionally.

Mizushima et al. introduced [cut operators](https://kmizu.github.io/papers/paste513-mizushima.pdf) to parsing expression grammars. In the PEG literature, a *soft cut* prevents backtracking only within the immediately enclosing ordered choice — outer choices can still recover. A *hard cut* prevents all backtracking past the cut point; failure is definitive. The `^` used in this grammar is a hard cut.

The hard cut operator is necessary because some tokens in Rust begin with a prefix that is itself a valid token. For example, `c"` begins a C string literal, but `c` alone is a valid identifier. Without the cut, if `c"\0"` failed to lex as a C string literal (because null bytes are not allowed in C strings), the parser could backtrack and lex it as two tokens: the identifier `c` and the string literal `"\0"`. The [cut after `c"`](tokens.md#r-lex.token.str-c.syntax) prevents this — once the opening delimiter is recognized, the parser cannot go back. The same reasoning applies to [byte literals](tokens.md#r-lex.token.byte.syntax), [byte string literals](tokens.md#r-lex.token.str-byte.syntax), [raw string literals](tokens.md#r-lex.token.literal.str-raw.syntax), and other literals with prefixes that are themselves valid tokens.

[notation.grammar.string-tables](#r-notation.grammar.string-tables)

### String table productions

Some rules in the grammar — notably [unary operators](expressions/operator-expr.md#borrow-operators), [binary operators](expressions/operator-expr.md#arithmetic-and-logical-binary-operators), and [keywords](keywords.md) — are given in a simplified form: as a listing of printable strings. These cases form a subset of the rules regarding the [token](tokens.md) rule, and are assumed to be the result of a lexical-analysis phase feeding the parser, driven by a <span class="abbr" title="Deterministic Finite
Automaton">DFA, operating over the disjunction of all such string table entries.

When such a string in `monospace` font occurs inside the grammar, it is an implicit reference to a single member of such a string table production. See [tokens](tokens.md) for more information.

[notation.grammar.visualizations](#r-notation.grammar.visualizations)

### Grammar visualizations

Below each grammar block is a button to toggle the display of a [syntax diagram](https://en.wikipedia.org/wiki/Syntax_diagram). A square element is a non-terminal rule, and a rounded rectangle is a terminal.
