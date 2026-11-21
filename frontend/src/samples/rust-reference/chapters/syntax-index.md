# Syntax index

This appendix provides an index of tokens and common forms with links to where those elements are defined.

## Keywords

| Keyword | Use |
|----|----|
| `_` | [wildcard pattern](patterns.md#r-patterns.wildcard), [inferred const](items/generics.md#r-items.generics.const.inferred), [inferred type](types/inferred.md#r-type.inferred), [placeholder lifetime](lifetime-elision.md#r-lifetime-elision.function.explicit-placeholder), [constant items](items/constant-items.md#r-items.const), [extern crate](items/extern-crates.md#r-items.extern-crate), [use declarations](items/use-declarations.md#r-items.use), [destructuring assignment](expressions/underscore-expr.md#r-expr.placeholder) |
| `abstract` | [reserved keyword](keywords.md#r-lex.keywords.reserved) |
| `as` | [extern crate](items/extern-crates.md#r-items.extern-crate.as), [use declarations](items/use-declarations.md#r-items.use.forms-as), [type cast expressions](expressions/operator-expr.md#r-expr.as), [qualified paths](paths.md#r-paths.qualified) |
| `async` | [async functions](items/functions.md#r-items.fn.async), [async blocks](expressions/block-expr.md#r-expr.block.async), [async closures](expressions/closure-expr.md#r-expr.closure.async) |
| `await` | [await expressions](expressions/await-expr.md#r-expr.await) |
| `become` | [reserved keyword](keywords.md#r-lex.keywords.reserved) |
| `box` | [reserved keyword](keywords.md#r-lex.keywords.reserved) |
| `break` | [break expressions](expressions/loop-expr.md#r-expr.loop.break) |
| `const` | [const functions](const_eval.md#r-const-eval.const-fn), [const items](items/constant-items.md#r-items.const), [const generics](items/generics.md#r-items.generics.const), [const blocks](expressions/block-expr.md#r-expr.block.const), [raw borrow operator](expressions/operator-expr.md#r-expr.borrow.raw), [raw pointer type](types/pointer.md#r-type.pointer.raw), [const assembly operands](inline-assembly.md#r-asm.operand-type.supported-operands-const) |
| `continue` | [continue expressions](expressions/loop-expr.md#r-expr.loop.continue) |
| `crate` | [extern crate](items/extern-crates.md#r-items.extern-crate), [visibility](visibility-and-privacy.md#r-vis), [paths](paths.md#r-paths) |
| `do` | [reserved keyword](keywords.md#r-lex.keywords.reserved) |
| `dyn` | [trait objects](types/trait-object.md#r-type.trait-object) |
| `else` | [let statements](statements.md#r-statement.let), [if expressions](expressions/if-expr.md#r-expr.if) |
| `enum` | [enumerations](items/enumerations.md#r-items.enum) |
| `extern` | [extern crate](items/extern-crates.md#r-items.extern-crate), [extern function qualifier](items/functions.md#r-items.fn.extern), [external blocks](items/external-blocks.md#r-items.extern), [extern function pointer types](types/function-pointer.md#r-type.fn-pointer.qualifiers) |
| `false` | [boolean type](types/boolean.md#r-type.bool), [boolean expressions](expressions/literal-expr.md#r-expr.literal), [configuration predicates](conditional-compilation.md#r-cfg) |
| `final` | [reserved keyword](keywords.md#r-lex.keywords.reserved) |
| `fn` | [functions](items/functions.md#r-items.fn), [function pointer types](types/function-pointer.md#r-type.fn-pointer) |
| `for` | [trait implementations](items/implementations.md#r-items.impl.trait), [iterator loops](expressions/loop-expr.md#r-expr.loop.for), [higher-ranked trait bounds](trait-bounds.md#r-bound.higher-ranked) |
| `gen` | [reserved keyword](keywords.md#r-lex.keywords.reserved) |
| `if` | [if expressions](expressions/if-expr.md#r-expr.if), [match guards](expressions/match-expr.md#r-expr.match.guard) |
| `impl` | [inherent impls](items/implementations.md#r-items.impl.inherent), [trait impls](items/implementations.md#r-items.impl.trait), [impl trait types](types/impl-trait.md#r-type.impl-trait.return), [anonymous type parameters](types/impl-trait.md#r-type.impl-trait.param) |
| `in` | [visibility](visibility-and-privacy.md#r-vis), [iterator loops](expressions/loop-expr.md#r-expr.loop.for), [assembly operands](inline-assembly.md#r-asm.operand-type.supported-operands-in) |
| `let` | [let statements](statements.md#r-statement.let), [`if let` patterns](expressions/if-expr.md#r-expr.if.let) |
| `loop` | [infinite loops](expressions/loop-expr.md#r-expr.loop.infinite) |
| `macro_rules` | [macros by example](macros-by-example.md#r-macro.decl) |
| `macro` | [reserved keyword](keywords.md#r-lex.keywords.reserved) |
| `match` | [match expressions](expressions/match-expr.md#r-expr.match) |
| `mod` | [modules](items/modules.md#r-items.mod) |
| `move` | [closure expressions](expressions/closure-expr.md#r-expr.closure), [async blocks](expressions/block-expr.md#r-expr.block.async) |
| `mut` | [borrow expressions](expressions/operator-expr.md#r-expr.operator.borrow), [identifier patterns](patterns.md#r-patterns.ident), [reference patterns](patterns.md#r-patterns.ref), [struct patterns](patterns.md#r-patterns.struct), [reference types](types/pointer.md#r-type.pointer.reference), [raw pointer types](types/pointer.md#r-type.pointer.raw), [self parameters](items/functions.md#r-items.fn.params.self-pat), [static items](items/static-items.md#r-items.static) |
| `override` | [reserved keyword](keywords.md#r-lex.keywords.reserved) |
| `priv` | [reserved keyword](keywords.md#r-lex.keywords.reserved) |
| `pub` | [visibility](visibility-and-privacy.md#r-vis) |
| `raw` | [borrow expressions](expressions/operator-expr.md#r-expr.operator.borrow), [raw assembly](inline-assembly.md#r-asm.options.supported-options-raw) |
| `ref` | [identifier patterns](patterns.md#r-patterns.ident), [struct patterns](patterns.md#r-patterns.struct) |
| `return` | [return expressions](expressions/return-expr.md#r-expr.return) |
| `safe` | [external block functions](items/external-blocks.md#r-items.extern.fn), [external block statics](items/external-blocks.md#r-items.extern.static) |
| `self` | [extern crate](items/extern-crates.md#r-items.extern-crate.self), [self parameters](items/functions.md#r-items.fn.params.self-pat), [visibility](visibility-and-privacy.md#r-vis), [`self` paths](paths.md#r-paths.qualifiers.mod-self) |
| `Self` | [`Self` type paths](paths.md#r-paths.qualifiers.type-self), [use bounds](trait-bounds.md#r-bound.use) |
| `static` | [static items](items/static-items.md#r-items.static), [`'static` lifetimes](trait-bounds.md#r-bound) |
| `struct` | [structs](items/structs.md#r-items.struct) |
| `super` | [super paths](paths.md#r-paths.qualifiers.super), [visibility](visibility-and-privacy.md#r-vis) |
| `trait` | [trait items](items/traits.md#r-items.traits) |
| `true` | [boolean type](types/boolean.md#r-type.bool), [boolean expressions](expressions/literal-expr.md#r-expr.literal), [configuration predicates](conditional-compilation.md#r-cfg) |
| `try` | [reserved keyword](keywords.md#r-lex.keywords.reserved) |
| `type` | [type aliases](items/type-aliases.md#r-items.type) |
| `typeof` | [reserved keyword](keywords.md#r-lex.keywords.reserved) |
| `union` | [union items](items/unions.md#r-items.union) |
| `unsafe` | [unsafe blocks](expressions/block-expr.md#r-expr.block.unsafe), [unsafe attributes](attributes.md#r-attributes.safety), [unsafe modules](items/modules.md#r-items.mod.unsafe), [unsafe functions](unsafe-keyword.md#r-unsafe.fn), [unsafe external blocks](unsafe-keyword.md#r-unsafe.extern), [unsafe external functions](items/external-blocks.md#r-items.extern.fn.safety), [unsafe external statics](items/external-blocks.md#r-items.extern.static.safety), [unsafe traits](items/traits.md#r-items.traits.safety), [unsafe trait implementations](items/implementations.md#r-items.impl.trait.safety) |
| `unsized` | [reserved keyword](keywords.md#r-lex.keywords.reserved) |
| `use` | [use items](items/use-declarations.md#r-items.use), [use bounds](trait-bounds.md#r-bound.use) |
| `virtual` | [reserved keyword](keywords.md#r-lex.keywords.reserved) |
| `where` | [where clauses](items/generics.md#r-items.generics.where) |
| `while` | [predicate loops](expressions/loop-expr.md#r-expr.loop.while) |
| `yield` | [reserved keyword](keywords.md#r-lex.keywords.reserved) |

## Operators and punctuation

| Symbol | Name | Use |
|----|----|----|
| `+` | Plus | [addition](expressions/operator-expr.md#r-expr.arith-logic), [trait bounds](trait-bounds.md#r-bound), [macro Kleene matcher](macros-by-example.md#r-macro.decl.repetition) |
| `-` | Minus | [subtraction](expressions/operator-expr.md#r-expr.arith-logic), [negation](expressions/operator-expr.md#r-expr.negate) |
| `*` | Star | [multiplication](expressions/operator-expr.md#r-expr.arith-logic), [dereference](expressions/operator-expr.md#r-expr.deref), [raw pointers](types/pointer.md#r-type.pointer.raw), [macro Kleene matcher](macros-by-example.md#r-macro.decl.repetition), [glob imports](items/use-declarations.md#r-items.use.glob) |
| `/` | Slash | [division](expressions/operator-expr.md#r-expr.arith-logic) |
| `%` | Percent | [remainder](expressions/operator-expr.md#r-expr.arith-logic) |
| `^` | Caret | [bitwise and logical XOR](expressions/operator-expr.md#r-expr.arith-logic) |
| `!` | Not | [bitwise and logical NOT](expressions/operator-expr.md#r-expr.negate), [macro calls](macros.md#r-macro.invocation), [inner attributes](attributes.md#r-attributes), [never type](types/never.md#r-type.never), [negative impls](items/implementations.md#r-items.impl) |
| `&` | And | [bitwise and logical AND](expressions/operator-expr.md#r-expr.arith-logic), [borrow](expressions/operator-expr.md#r-expr.operator.borrow), [references](types/pointer.md#r-type.pointer.reference), [reference patterns](patterns.md#r-patterns.ref) |
| `|` | Or | [bitwise and logical OR](expressions/operator-expr.md#r-expr.arith-logic), [closures](expressions/closure-expr.md#r-expr.closure), [or patterns](patterns.md#r-patterns.or), [if let](expressions/if-expr.md#r-expr.if.let), [while let](expressions/loop-expr.md#r-expr.loop.while.let) |
| `&&` | AndAnd | [lazy AND](expressions/operator-expr.md#r-expr.bool-logic), [borrow](expressions/operator-expr.md#r-expr.operator.borrow), [references](types/pointer.md#r-type.pointer.reference), [reference patterns](patterns.md#r-patterns.ref) |
| `||` | OrOr | [lazy OR](expressions/operator-expr.md#r-expr.bool-logic), [closures](expressions/closure-expr.md#r-expr.closure) |
| `<<` | Shl | [shift left](expressions/operator-expr.md#r-expr.arith-logic), [nested generics](items/generics.md#r-items.generics) |
| `>>` | Shr | [shift right](expressions/operator-expr.md#r-expr.arith-logic), [nested generics](items/generics.md#r-items.generics) |
| `+=` | PlusEq | [addition assignment](expressions/operator-expr.md#r-expr.compound-assign) |
| `-=` | MinusEq | [subtraction assignment](expressions/operator-expr.md#r-expr.compound-assign) |
| `*=` | StarEq | [multiplication assignment](expressions/operator-expr.md#r-expr.compound-assign) |
| `/=` | SlashEq | [division assignment](expressions/operator-expr.md#r-expr.compound-assign) |
| `%=` | PercentEq | [remainder assignment](expressions/operator-expr.md#r-expr.compound-assign) |
| `^=` | CaretEq | [bitwise XOR assignment](expressions/operator-expr.md#r-expr.compound-assign) |
| `&=` | AndEq | [bitwise AND assignment](expressions/operator-expr.md#r-expr.compound-assign) |
| `|=` | OrEq | [bitwise OR assignment](expressions/operator-expr.md#r-expr.compound-assign) |
| `<<=` | ShlEq | [shift left assignment](expressions/operator-expr.md#r-expr.compound-assign) |
| `>>=` | ShrEq | [shift right assignment](expressions/operator-expr.md#r-expr.compound-assign), [nested generics](items/generics.md#r-items.generics) |
| `=` | Eq | [assignment](expressions/operator-expr.md#r-expr.assign), [let statements](statements.md#r-statement.let), [attributes](attributes.md#r-attributes), various type definitions |
| `==` | EqEq | [equal](expressions/operator-expr.md#r-expr.cmp) |
| `!=` | Ne | [not equal](expressions/operator-expr.md#r-expr.cmp) |
| `>` | Gt | [greater than](expressions/operator-expr.md#r-expr.cmp), [generics](items/generics.md#r-items.generics), [paths](paths.md#r-paths), [use bounds](trait-bounds.md#r-bound.use) |
| `<` | Lt | [less than](expressions/operator-expr.md#r-expr.cmp), [generics](items/generics.md#r-items.generics), [paths](paths.md#r-paths), [use bounds](trait-bounds.md#r-bound.use) |
| `>=` | Ge | [greater than or equal to](expressions/operator-expr.md#r-expr.cmp), [generics](items/generics.md#r-items.generics) |
| `<=` | Le | [less than or equal to](expressions/operator-expr.md#r-expr.cmp) |
| `@` | At | [subpattern binding](patterns.md#r-patterns.ident.scrutinized) |
| `.` | Dot | [field access](expressions/field-expr.md#r-expr.field), [tuple index](expressions/tuple-expr.md#r-expr.tuple-index), [method calls](expressions/method-call-expr.md#r-expr.method), [await expressions](expressions/await-expr.md#r-expr.await) |
| `..` | DotDot | [range expressions](expressions/range-expr.md#r-expr.range), [struct expressions](expressions/struct-expr.md#r-expr.struct), [rest pattern](patterns.md#r-patterns.rest), [range patterns](patterns.md#r-patterns.range), [struct patterns](patterns.md#r-patterns.struct) |
| `...` | DotDotDot | [variadic functions](items/external-blocks.md#r-items.extern.variadic), [range patterns](patterns.md#r-patterns.range) |
| `..=` | DotDotEq | [inclusive range expressions](expressions/range-expr.md#r-expr.range), [range patterns](patterns.md#r-patterns.range) |
| `,` | Comma | various separators |
| `;` | Semi | terminator for various items and statements, [array expressions](expressions/array-expr.md#r-expr.array), [array types](types/array.md#r-type.array) |
| `:` | Colon | various separators |
| `::` | PathSep | [path separator](paths.md#r-paths) |
| `->` | RArrow | [functions](items/functions.md#r-items.fn), [closures](expressions/closure-expr.md#r-expr.closure), [function pointer type](types/function-pointer.md#r-type.fn-pointer) |
| `=>` | FatArrow | [match arms](expressions/match-expr.md#r-expr.match), [macros](macros-by-example.md#r-macro.decl) |
| `<-` | LArrow | The left arrow symbol has been unused since before Rust 1.0, but it is still treated as a single token. |
| `#` | Pound | [attributes](attributes.md#r-attributes), [raw string literals](tokens.md#r-lex.token.literal.str-raw), [raw byte string literals](tokens.md#r-lex.token.str-byte-raw), [raw C string literals](tokens.md#r-lex.token.str-c-raw) |
| `$` | Dollar | [macros](macros-by-example.md#r-macro.decl) |
| `?` | Question | [try propagation expressions](expressions/operator-expr.md#r-expr.try), [relaxed trait bounds](trait-bounds.md#r-bound.sized), [macro Kleene matcher](macros-by-example.md#r-macro.decl.repetition) |
| `~` | Tilde | The tilde operator has been unused since before Rust 1.0, but its token may still be used. |

## Comments

| Comment  | Use                                               |
|----------|---------------------------------------------------|
| `//`     | [line comment](comments.md#r-comments)            |
| `//!`    | [inner line comment](comments.md#r-comments)      |
| `///`    | [outer line doc comment](comments.md#r-comments)  |
| `/*…*/`  | [block comment](comments.md#r-comments)           |
| `/*!…*/` | [inner block doc comment](comments.md#r-comments) |
| `/**…*/` | [outer block doc comment](comments.md#r-comments) |

## Other tokens

| Token | Use |
|----|----|
| `ident` | [identifiers](identifiers.md#r-ident) |
| `r#ident` | [raw identifiers](identifiers.md#r-ident.raw) |
| `'ident` | [lifetimes and loop labels](tokens.md#r-lex.token.life) |
| `'r#ident` | [raw lifetimes and loop labels](tokens.md#r-lex.token.life) |
| `…u8`, `…i32`, `…f64`, `…usize`, … | [number literals](tokens.md#r-lex.token.literal.num) |
| `"…"` | [string literals](tokens.md#r-lex.token.literal.str) |
| `r"…"`, `r#"…"#`, `r##"…"##`, … | [raw string literals](tokens.md#r-lex.token.literal.str-raw) |
| `b"…"` | [byte string literals](tokens.md#r-lex.token.str-byte) |
| `br"…"`, `br#"…"#`, `br##"…"##`, … | [raw byte string literals](tokens.md#r-lex.token.str-byte-raw) |
| `'…'` | [character literals](tokens.md#r-lex.token.literal.char) |
| `b'…'` | [byte literals](tokens.md#r-lex.token.byte) |
| `c"…"` | [C string literals](tokens.md#r-lex.token.str-c) |
| `cr"…"`, `cr#"…"#`, `cr##"…"##`, … | [raw C string literals](tokens.md#r-lex.token.str-c-raw) |

## Macros

 | Syntax | 
 | Use | 

 | ident!(…)

ident! {…}

ident![…] | 
 | [macro invocations](macros.md#r-macro.invocation) | 

 | $ident | 
 | [macro metavariable](macros-by-example.md#r-macro.decl.meta) | 

 | $ident:kind | 
 | [macro matcher fragment specifier](macros-by-example.md#r-macro.decl.meta.specifier) | 

 | $(…)… | 
 | [macro repetition](macros-by-example.md#r-macro.decl.repetition) | 

## Attributes

| Syntax     | Use                                                 |
|------------|-----------------------------------------------------|
| `#[meta]`  | [outer attribute](attributes.md#r-attributes.outer) |
| `#![meta]` | [inner attribute](attributes.md#r-attributes.inner) |

## Expressions

 | Expression | 
 | Use | 

 | |…| expr

|…| -&gt; Type { … } | 
 | [closures](expressions/closure-expr.md#r-expr.closure) | 

 | ident::… | 
 | [paths](paths.md#r-paths) | 

 | ::crate_name::… | 
 | [explicit crate paths](paths.md#r-paths.qualifiers.global-root) | 

 | crate::… | 
 | [crate-relative paths](paths.md#r-paths.qualifiers.crate) | 

 | self::… | 
 | [module-relative paths](paths.md#r-paths.qualifiers.mod-self) | 

 | super::… | 
 | [parent module paths](paths.md#r-paths.qualifiers.super) | 

 | Type::…

&lt;Type as Trait&gt;::ident | 
 | [associated items](items/associated-items.md#r-items.associated) | 

 | &lt;Type&gt;::… | 
 | [qualified paths](paths.md#r-paths.qualified) which can be used for types without names such as &lt;&amp;T&gt;::…, &lt;[T]&gt;::…, etc. | 

 | Trait::method(…)

Type::method(…)

&lt;Type as Trait&gt;::method(…) | 
 | [disambiguated method calls](expressions/call-expr.md#r-expr.call.desugar) | 

 | method::&lt;…&gt;(…)

path::&lt;…&gt; | 
 | [generic arguments](items/generics.md#r-items.generics), aka turbofish | 

 | () | 
 | [unit](types/tuple.md#r-type.tuple.unit) | 

 | (expr) | 
 | [parenthesized expressions](expressions/grouped-expr.md#r-expr.paren) | 

 | (expr,) | 
 | [single-element tuple expressions](expressions/tuple-expr.md#r-expr.tuple) | 

 | (expr, …) | 
 | [tuple expressions](expressions/tuple-expr.md#r-expr.tuple) | 

 | expr(expr, …) | 
 | [call expressions](expressions/call-expr.md#r-expr.call) | 

 | expr.0, expr.1, … | 
 | [tuple indexing expressions](expressions/tuple-expr.md#r-expr.tuple-index) | 

 | expr.ident | 
 | [field access expressions](expressions/field-expr.md#r-expr.field) | 

 | {…} | 
 | [block expressions](expressions/block-expr.md#r-expr.block) | 

 | Type {…} | 
 | [struct expressions](expressions/struct-expr.md#r-expr.struct) | 

 | Type(…) | 
 | [tuple struct constructors](items/structs.md#r-items.struct.tuple) | 

 | […] | 
 | [array expressions](expressions/array-expr.md#r-expr.array) | 

 | [expr; len] | 
 | [repeat array expressions](expressions/array-expr.md#r-expr.array) | 

 | expr[..], expr[a..], expr[..b], expr[a..b], expr[a..=b], expr[..=b] | 
 | [array and slice indexing expressions](expressions/array-expr.md#r-expr.array.index) | 

 | if expr {…} else {…} | 
 | [if expressions](expressions/if-expr.md#r-expr.if) | 

 | match expr { pattern =&gt; {…} } | 
 | [match expressions](expressions/match-expr.md#r-expr.match) | 

 | loop {…} | 
 | [infinite loop expressions](expressions/loop-expr.md#r-expr.loop.infinite) | 

 | while expr {…} | 
 | [predicate loop expressions](expressions/loop-expr.md#r-expr.loop.while) | 

 | for pattern in expr {…} | 
 | [iterator loops](expressions/loop-expr.md#r-expr.loop.for) | 

 | &amp;expr

&amp;mut expr | 
 | [borrow expressions](expressions/operator-expr.md#r-expr.operator.borrow) | 

 | &amp;raw const expr

&amp;raw mut expr | 
 | [raw borrow expressions](expressions/operator-expr.md#r-expr.borrow.raw) | 

 | *expr | 
 | [dereference expressions](expressions/operator-expr.md#r-expr.deref) | 

 | expr? | 
 | [try propagation expressions](expressions/operator-expr.md#r-expr.try) | 

 | -expr | 
 | [negation expressions](expressions/operator-expr.md#r-expr.negate) | 

 | !expr | 
 | [bitwise and logical NOT expressions](expressions/operator-expr.md#r-expr.negate) | 

 | expr as Type | 
 | [type cast expressions](expressions/operator-expr.md#r-expr.as) | 

## Items

[Items](items.md#r-items) are the components of a crate.

 | Item | 
 | Use | 

 | mod ident;

mod ident {…} | 
 | [modules](items/modules.md#r-items.mod) | 

 | use path; | 
 | [use declarations](items/use-declarations.md#r-items.use) | 

 | fn ident(…) {…} | 
 | [functions](items/functions.md#r-items.fn) | 

 | type Type = Type; | 
 | [type aliases](items/type-aliases.md#r-items.type) | 

 | struct ident {…} | 
 | [structs](items/structs.md#r-items.struct) | 

 | enum ident {…} | 
 | [enumerations](items/enumerations.md#r-items.enum) | 

 | union ident {…} | 
 | [unions](items/unions.md#r-items.union) | 

 | trait ident {…} | 
 | [traits](items/traits.md#r-items.traits) | 

 | impl Type {…}

impl Type for Trait {…} | 
 | [implementations](items/implementations.md#r-items.impl) | 

 | const ident = expr; | 
 | [constant items](items/constant-items.md#r-items.const) | 

 | static ident = expr; | 
 | [static items](items/static-items.md#r-items.static) | 

 | extern "C" {…} | 
 | [external blocks](items/external-blocks.md#r-items.extern) | 

 | fn ident&lt;…&gt;(…) …

struct ident&lt;…&gt; {…}

enum ident&lt;…&gt; {…}

impl&lt;…&gt; Type&lt;…&gt; {…} | 
 | [generic definitions](items/generics.md#r-items.generics) | 

## Type expressions

[Type expressions](types.md#r-type.name) are used to refer to types.

 | Type | 
 | Use | 

 | bool, u8, f64, str, … | 
 | [primitive types](types.md#r-type.kinds) | 

 | for&lt;…&gt; | 
 | [higher-ranked trait bounds](trait-bounds.md#r-bound.higher-ranked) | 

 | T: TraitA + TraitB | 
 | [trait bounds](trait-bounds.md#r-bound) | 

 | T: 'a + 'b | 
 | [lifetime bounds](trait-bounds.md#r-bound.lifetime) | 

 | T: TraitA + 'a | 
 | [trait and lifetime bounds](trait-bounds.md#r-bound) | 

 | T: ?Sized | 
 | [relaxed trait bounds](trait-bounds.md#r-bound.sized) | 

 | [Type; len] | 
 | [array types](types/array.md#r-type.array) | 

 | (Type, …) | 
 | [tuple types](types/tuple.md#r-type.tuple) | 

 | [Type] | 
 | [slice types](types/slice.md#r-type.slice) | 

 | (Type) | 
 | [parenthesized types](types.md#r-type.name.parenthesized) | 

 | impl Trait | 
 | [impl trait types](types/impl-trait.md#r-type.impl-trait.return), [anonymous type parameters](types/impl-trait.md#r-type.impl-trait.param) | 

 | dyn Trait | 
 | [trait object types](types/trait-object.md#r-type.trait-object) | 

 | ident

ident::… | 
 | [type paths](types.md#r-type.name.path) (can refer to [structs](items/structs.md#r-items.struct), [enumerations](items/enumerations.md#r-items.enum), [unions](items/unions.md#r-items.union), [type aliases](items/type-aliases.md#r-items.type), [traits](items/traits.md#r-items.traits), [generics](items/generics.md#r-items.generics), etc.) | 

 | Type&lt;…&gt;

Trait&lt;…&gt; | 
 | [generic arguments](items/generics.md#r-items.generics) (e.g. Vec&lt;u8&gt;) | 

 | Trait&lt;ident = Type&gt; | 
 | [associated type bindings](paths.md#r-paths.expr) (e.g. Iterator&lt;Item = T&gt;) | 

 | Trait&lt;ident: …&gt; | 
 | [associated type bounds](paths.md#r-paths.expr) (e.g. Iterator&lt;Item: Send&gt;) | 

 | &amp;Type

&amp;mut Type | 
 | [reference types](types/pointer.md#r-type.pointer.reference) | 

 | *mut Type

*const Type | 
 | [raw pointer types](types/pointer.md#r-type.pointer.raw) | 

 | fn(…) -&gt; Type | 
 | [function pointer types](types/function-pointer.md#r-type.fn-pointer) | 

 | _ | 
 | [inferred type](types/inferred.md#r-type.inferred), [inferred const](items/generics.md#r-items.generics.const.inferred) | 

 | '_ | 
 | [placeholder lifetime](lifetime-elision.md#r-lifetime-elision.function.explicit-placeholder) | 

 | ! | 
 | [never type](types/never.md#r-type.never) | 

## Patterns

[Patterns](patterns.md#r-patterns) are used to match values.

 | Pattern | 
 | Use | 

 | "foo", 'a', 123, 2.4, … | 
 | [literal patterns](patterns.md#r-patterns.literal) | 

 | ident | 
 | [identifier patterns](patterns.md#r-patterns.ident) | 

 | _ | 
 | [wildcard pattern](patterns.md#r-patterns.wildcard) | 

 | .. | 
 | [rest pattern](patterns.md#r-patterns.rest) | 

 | a.., ..b, a..b, a..=b, ..=b | 
 | [range patterns](patterns.md#r-patterns.range) | 

 | &amp;pattern

&amp;mut pattern | 
 | [reference patterns](patterns.md#r-patterns.ref) | 

 | path {…} | 
 | [struct patterns](patterns.md#r-patterns.struct) | 

 | path(…) | 
 | [tuple struct patterns](patterns.md#r-patterns.tuple-struct) | 

 | (pattern, …) | 
 | [tuple patterns](patterns.md#r-patterns.tuple) | 

 | (pattern) | 
 | [grouped patterns](patterns.md#r-patterns.paren) | 

 | [pattern, …] | 
 | [slice patterns](patterns.md#r-patterns.slice) | 

 | CONST, Enum::Variant, … | 
 | [path patterns](patterns.md#r-patterns.path) |
