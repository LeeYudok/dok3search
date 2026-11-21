# Comments

[comments](#r-comments)

# Comments

[comments.syntax](#r-comments.syntax)

**Lexer**\
[COMMENT](#railroad-COMMENT) →\
      [LINE_COMMENT](#grammar-LINE_COMMENT)\
    \| [INNER_LINE_DOC](#grammar-INNER_LINE_DOC)\
    \| [OUTER_LINE_DOC](#grammar-OUTER_LINE_DOC)\
    \| [INNER_BLOCK_DOC](#grammar-INNER_BLOCK_DOC)\
    \| [OUTER_BLOCK_DOC](#grammar-OUTER_BLOCK_DOC)\
    \| [BLOCK_COMMENT](#grammar-BLOCK_COMMENT)

[LINE_COMMENT](#railroad-LINE_COMMENT) →\
      // ( ~\[/ ! [LF](whitespace.md#grammar-LF)\] \| // ) ~[LF](whitespace.md#grammar-LF)\*\
    \| // [EOF](input-format.md#grammar-EOF)\
    \| //immediately followed by LF

[BLOCK_COMMENT](#railroad-BLOCK_COMMENT) →\
    /\* ^\
      ( [BLOCK_COMMENT](#grammar-BLOCK_COMMENT) \| [BLOCK_CHAR](#grammar-BLOCK_CHAR) )\*\
    \*/

[INNER_LINE_DOC](#railroad-INNER_LINE_DOC) →\
    //! ^ [LINE_DOC_COMMENT_CONTENT](#grammar-LINE_DOC_COMMENT_CONTENT) ( [LF](whitespace.md#grammar-LF) \| [EOF](input-format.md#grammar-EOF) )

[LINE_DOC_COMMENT_CONTENT](#railroad-LINE_DOC_COMMENT_CONTENT) → ( \![CR](whitespace.md#grammar-CR) ~[LF](whitespace.md#grammar-LF) )\*

[INNER_BLOCK_DOC](#railroad-INNER_BLOCK_DOC) →\
    /\*! ^ ( [NESTED_BLOCK_DOC_COMMENT](#grammar-NESTED_BLOCK_DOC_COMMENT) \| [DOC_BLOCK_CHAR](#grammar-DOC_BLOCK_CHAR) )\* \*/

[OUTER_LINE_DOC](#railroad-OUTER_LINE_DOC) →\
    /// ^ [LINE_DOC_COMMENT_CONTENT](#grammar-LINE_DOC_COMMENT_CONTENT) ( [LF](whitespace.md#grammar-LF) \| [EOF](input-format.md#grammar-EOF) )

[OUTER_BLOCK_DOC](#railroad-OUTER_BLOCK_DOC) →\
    /\*\* \![\* /\]\
      ^\
      ~\[\* [CR](whitespace.md#grammar-CR)\]\
      ( [NESTED_BLOCK_DOC_COMMENT](#grammar-NESTED_BLOCK_DOC_COMMENT) \| [DOC_BLOCK_CHAR](#grammar-DOC_BLOCK_CHAR) )\*\
    \*/

[BLOCK_CHAR](#railroad-BLOCK_CHAR) → !\*/ [CHAR](input-format.md#grammar-CHAR)

[DOC_BLOCK_CHAR](#railroad-DOC_BLOCK_CHAR) → ( !( \*/ \| [CR](whitespace.md#grammar-CR) ) [CHAR](input-format.md#grammar-CHAR) )

[NESTED_BLOCK_DOC_COMMENT](#railroad-NESTED_BLOCK_DOC_COMMENT) →\
    /\* ( [NESTED_BLOCK_DOC_COMMENT](#grammar-NESTED_BLOCK_DOC_COMMENT) \| [DOC_BLOCK_CHAR](#grammar-DOC_BLOCK_CHAR) )\* \*/

Show Railroad

![](../assets/inline/comments-1.svg) ![](../assets/inline/comments-2.svg) ![](../assets/inline/comments-3.svg) ![](../assets/inline/comments-4.svg) ![](../assets/inline/comments-5.svg) ![](../assets/inline/comments-6.svg) ![](../assets/inline/comments-7.svg) ![](../assets/inline/comments-8.svg) ![](../assets/inline/comments-9.svg) ![](../assets/inline/comments-10.svg) ![](../assets/inline/comments-11.svg) [comments.normal](#r-comments.normal)

## Non-doc comments

[comments.normal.intro](#r-comments.normal.intro)

Comments follow the general C++ style of line (`//`) and block (`/* ... */`) comment forms. Nested block comments are supported.

[comments.normal.tokenization](#r-comments.normal.tokenization)\
Tests Tests with this rule:

- [tests/ui/tuple/index-float.rs](https://github.com/rust-lang/rust/blob/1.100.0/tests/ui/tuple/index-float.rs)

Non-doc comments are interpreted as a form of whitespace.

[comments.doc](#r-comments.doc)

## Doc comments

[comments.doc.syntax](#r-comments.doc.syntax)

Line doc comments beginning with exactly *three* slashes (`///`), and block doc comments (`/** ... */`), both outer doc comments, are interpreted as a special syntax for [`doc` attributes](https://doc.rust-lang.org/1.100.0/rustdoc/the-doc-attribute.html).

[comments.doc.attributes](#r-comments.doc.attributes)

That is, they are equivalent to writing `#[doc="..."]` around the body of the comment, i.e., `/// Foo` turns into `#[doc=" Foo"]` and `/** Bar */` turns into `#[doc=" Bar "]`. They must therefore appear before something that accepts an outer attribute.

[comments.doc.inner-syntax](#r-comments.doc.inner-syntax)

Line comments beginning with `//!` and block comments `/*! ... */` are doc comments that apply to the parent of the comment, rather than the item that follows.

[comments.doc.inner-attributes](#r-comments.doc.inner-attributes)

That is, they are equivalent to writing `#![doc="..."]` around the body of the comment. `//!` comments are usually used to document modules that occupy a source file.

[comments.doc.bare-crs](#r-comments.doc.bare-crs)

The character `U+000D` (CR) is not allowed in doc comments.

> ![](../assets/inline/comments-12.svg)Note
>
> It is conventional for doc comments to contain Markdown, as expected by `rustdoc`. However, the comment syntax does not respect any internal Markdown. `` /** `glob = "*/*.rs";` */ `` terminates the comment at the first `*/`, and the remaining code would cause a syntax error. This slightly limits the content of block doc comments compared to line doc comments.

> ![](../assets/inline/comments-13.svg)Note
>
> The sequence `U+000D` (CR) immediately followed by `U+000A` (LF) would have been previously transformed into a single `U+000A` (LF).

## Examples

``` playground
#![allow(unused)]
fn main() {
//! A doc comment that applies to the implicit anonymous module of this crate

pub mod outer_module {

    //!  - Inner line doc
    //!! - Still an inner line doc (but with a bang at the beginning)

    /*!  - Inner block doc */
    /*!! - Still an inner block doc (but with a bang at the beginning) */

    //   - Only a comment
    ///  - Outer line doc (exactly 3 slashes)
    //// - Only a comment

    /*   - Only a comment */
    /**  - Outer block doc (exactly) 2 asterisks */
    /*** - Only a comment */

    pub mod inner_module {}

    pub mod nested_comments {
        /* In Rust /* we can /* nest comments */ */ */

        // All three types of block comments can contain or be nested inside
        // any other type:

        /*   /* */  /** */  /*! */  */
        /*!  /* */  /** */  /*! */  */
        /**  /* */  /** */  /*! */  */
        pub mod dummy_item {}
    }

    pub mod degenerate_cases {
        // empty inner line doc
        //!

        // empty inner block doc
        /*!*/

        // empty line comment
        //

        // empty outer line doc
        ///

        // empty block comment
        /**/

        pub mod dummy_item {}

        // empty 2-asterisk block isn't a doc block, it is a block comment
        /***/

    }

    /* The next one isn't allowed because outer doc comments
       require an item that will receive the doc */

    /// Where is my item?
  mod boo {}
}
}
```
