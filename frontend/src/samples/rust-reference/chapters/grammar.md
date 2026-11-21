# Grammar summary

# Grammar summary

The following is a summary of the grammar production rules. For details on the syntax of this grammar, see *[notation.grammar.syntax](notation.md#r-notation.grammar.syntax)*.

## Attributes summary

**Syntax**\
[InlineAttribute](#railroad-summary-InlineAttribute) →\
      inline ( always )\
    \| inline ( never )\
    \| inline

[CollapseDebuginfoAttribute](#railroad-summary-CollapseDebuginfoAttribute) → collapse_debuginfo ( [CollapseDebuginfoOption](#grammar-summary-CollapseDebuginfoOption) )

[CollapseDebuginfoOption](#railroad-summary-CollapseDebuginfoOption) →\
      yes\
    \| no\
    \| external

[InnerAttribute](#railroad-summary-InnerAttribute) → \# ! \[ [Attr](#grammar-summary-Attr) \]

[OuterAttribute](#railroad-summary-OuterAttribute) → \# \[ [Attr](#grammar-summary-Attr) \]

[Attr](#railroad-summary-Attr) →\
      [SimplePath](#grammar-summary-SimplePath) [AttrInput](#grammar-summary-AttrInput)?\
    \| unsafe ( [SimplePath](#grammar-summary-SimplePath) [AttrInput](#grammar-summary-AttrInput)? )

[AttrInput](#railroad-summary-AttrInput) →\
      [DelimTokenTree](#grammar-summary-DelimTokenTree)\
    \| = [Expression](#grammar-summary-Expression)

[MetaItem](#railroad-summary-MetaItem) →\
      [SimplePath](#grammar-summary-SimplePath)\
    \| [SimplePath](#grammar-summary-SimplePath) = [Expression](#grammar-summary-Expression)\
    \| [SimplePath](#grammar-summary-SimplePath) ( [MetaSeq](#grammar-summary-MetaSeq)? )

[MetaSeq](#railroad-summary-MetaSeq) →\
    [MetaItemInner](#grammar-summary-MetaItemInner) ( , [MetaItemInner](#grammar-summary-MetaItemInner) )\* ,?

[MetaItemInner](#railroad-summary-MetaItemInner) →\
      [MetaItem](#grammar-summary-MetaItem)\
    \| [Expression](#grammar-summary-Expression)

[MetaWord](#railroad-summary-MetaWord) →\
    [IDENTIFIER](#grammar-summary-IDENTIFIER)

[MetaNameValueStr](#railroad-summary-MetaNameValueStr) →\
    [IDENTIFIER](#grammar-summary-IDENTIFIER) = ( [STRING_LITERAL](#grammar-summary-STRING_LITERAL) \| [RAW_STRING_LITERAL](#grammar-summary-RAW_STRING_LITERAL) )

[MetaListPaths](#railroad-summary-MetaListPaths) →\
    [IDENTIFIER](#grammar-summary-IDENTIFIER) ( ( [SimplePath](#grammar-summary-SimplePath) ( , [SimplePath](#grammar-summary-SimplePath) )\* ,? )? )

[MetaListIdents](#railroad-summary-MetaListIdents) →\
    [IDENTIFIER](#grammar-summary-IDENTIFIER) ( ( [IDENTIFIER](#grammar-summary-IDENTIFIER) ( , [IDENTIFIER](#grammar-summary-IDENTIFIER) )\* ,? )? )

[MetaListNameValueStr](#railroad-summary-MetaListNameValueStr) →\
    [IDENTIFIER](#grammar-summary-IDENTIFIER) ( ( [MetaNameValueStr](#grammar-summary-MetaNameValueStr) ( , [MetaNameValueStr](#grammar-summary-MetaNameValueStr) )\* ,? )? )

[ProcMacroDeriveAttribute](#railroad-summary-ProcMacroDeriveAttribute) →\
    proc_macro_derive ( [DeriveMacroName](#grammar-summary-DeriveMacroName) ( , [DeriveMacroAttributes](#grammar-summary-DeriveMacroAttributes) )? ,? )

[DeriveMacroName](#railroad-summary-DeriveMacroName) → [IDENTIFIER](#grammar-summary-IDENTIFIER)

[DeriveMacroAttributes](#railroad-summary-DeriveMacroAttributes) →\
    attributes ( ( [IDENTIFIER](#grammar-summary-IDENTIFIER) ( , [IDENTIFIER](#grammar-summary-IDENTIFIER) )\* ,? )? )

Show Railroad

![](../assets/inline/grammar-1.svg) ![](../assets/inline/grammar-2.svg) ![](../assets/inline/grammar-3.svg) ![](../assets/inline/grammar-4.svg) ![](../assets/inline/grammar-5.svg) ![](../assets/inline/grammar-6.svg) ![](../assets/inline/grammar-7.svg) ![](../assets/inline/grammar-8.svg) ![](../assets/inline/grammar-9.svg) ![](../assets/inline/grammar-10.svg) ![](../assets/inline/grammar-11.svg) ![](../assets/inline/grammar-12.svg) ![](../assets/inline/grammar-13.svg) ![](../assets/inline/grammar-14.svg) ![](../assets/inline/grammar-15.svg) ![](../assets/inline/grammar-16.svg) ![](../assets/inline/grammar-17.svg) ![](../assets/inline/grammar-18.svg)

## Lexer summary

**Lexer**\
[COMMENT](#railroad-summary-COMMENT) →\
      [LINE_COMMENT](#grammar-summary-LINE_COMMENT)\
    \| [INNER_LINE_DOC](#grammar-summary-INNER_LINE_DOC)\
    \| [OUTER_LINE_DOC](#grammar-summary-OUTER_LINE_DOC)\
    \| [INNER_BLOCK_DOC](#grammar-summary-INNER_BLOCK_DOC)\
    \| [OUTER_BLOCK_DOC](#grammar-summary-OUTER_BLOCK_DOC)\
    \| [BLOCK_COMMENT](#grammar-summary-BLOCK_COMMENT)

[LINE_COMMENT](#railroad-summary-LINE_COMMENT) →\
      // ( ~\[/ ! [LF](#grammar-summary-LF)\] \| // ) ~[LF](#grammar-summary-LF)\*\
    \| // [EOF](#grammar-summary-EOF)\
    \| //immediately followed by LF

[BLOCK_COMMENT](#railroad-summary-BLOCK_COMMENT) →\
    /\* ^\
      ( [BLOCK_COMMENT](#grammar-summary-BLOCK_COMMENT) \| [BLOCK_CHAR](#grammar-summary-BLOCK_CHAR) )\*\
    \*/

[INNER_LINE_DOC](#railroad-summary-INNER_LINE_DOC) →\
    //! ^ [LINE_DOC_COMMENT_CONTENT](#grammar-summary-LINE_DOC_COMMENT_CONTENT) ( [LF](#grammar-summary-LF) \| [EOF](#grammar-summary-EOF) )

[LINE_DOC_COMMENT_CONTENT](#railroad-summary-LINE_DOC_COMMENT_CONTENT) → ( \![CR](#grammar-summary-CR) ~[LF](#grammar-summary-LF) )\*

[INNER_BLOCK_DOC](#railroad-summary-INNER_BLOCK_DOC) →\
    /\*! ^ ( [NESTED_BLOCK_DOC_COMMENT](#grammar-summary-NESTED_BLOCK_DOC_COMMENT) \| [DOC_BLOCK_CHAR](#grammar-summary-DOC_BLOCK_CHAR) )\* \*/

[OUTER_LINE_DOC](#railroad-summary-OUTER_LINE_DOC) →\
    /// ^ [LINE_DOC_COMMENT_CONTENT](#grammar-summary-LINE_DOC_COMMENT_CONTENT) ( [LF](#grammar-summary-LF) \| [EOF](#grammar-summary-EOF) )

[OUTER_BLOCK_DOC](#railroad-summary-OUTER_BLOCK_DOC) →\
    /\*\* \![\* /\]\
      ^\
      ~\[\* [CR](#grammar-summary-CR)\]\
      ( [NESTED_BLOCK_DOC_COMMENT](#grammar-summary-NESTED_BLOCK_DOC_COMMENT) \| [DOC_BLOCK_CHAR](#grammar-summary-DOC_BLOCK_CHAR) )\*\
    \*/

[BLOCK_CHAR](#railroad-summary-BLOCK_CHAR) → !\*/ [CHAR](#grammar-summary-CHAR)

[DOC_BLOCK_CHAR](#railroad-summary-DOC_BLOCK_CHAR) → ( !( \*/ \| [CR](#grammar-summary-CR) ) [CHAR](#grammar-summary-CHAR) )

[NESTED_BLOCK_DOC_COMMENT](#railroad-summary-NESTED_BLOCK_DOC_COMMENT) →\
    /\* ( [NESTED_BLOCK_DOC_COMMENT](#grammar-summary-NESTED_BLOCK_DOC_COMMENT) \| [DOC_BLOCK_CHAR](#grammar-summary-DOC_BLOCK_CHAR) )\* \*/

[IDENTIFIER_OR_KEYWORD](#railroad-summary-IDENTIFIER_OR_KEYWORD) → ( [XID_Start](#grammar-summary-XID_Start) \| \_ ) [XID_Continue](#grammar-summary-XID_Continue)\*

[XID_Start](#railroad-summary-XID_Start) → \<`XID_Start` defined by Unicode\>

[XID_Continue](#railroad-summary-XID_Continue) → \<`XID_Continue` defined by Unicode\>

[RAW_IDENTIFIER](#railroad-summary-RAW_IDENTIFIER) → r# [IDENTIFIER_OR_KEYWORD](#grammar-summary-IDENTIFIER_OR_KEYWORD)

[NON_KEYWORD_IDENTIFIER](#railroad-summary-NON_KEYWORD_IDENTIFIER) → [IDENTIFIER_OR_KEYWORD](#grammar-summary-IDENTIFIER_OR_KEYWORD)except a [strict](keywords.md#r-lex.keywords.strict) or [reserved](keywords.md#r-lex.keywords.reserved) keyword

[IDENTIFIER](#railroad-summary-IDENTIFIER) → [NON_KEYWORD_IDENTIFIER](#grammar-summary-NON_KEYWORD_IDENTIFIER) \| [RAW_IDENTIFIER](#grammar-summary-RAW_IDENTIFIER)

[RESERVED_RAW_IDENTIFIER](#railroad-summary-RESERVED_RAW_IDENTIFIER) →\
    r# ( \_ \| crate \| self \| Self \| super ) \![XID_Continue](#grammar-summary-XID_Continue)

[CHAR](#railroad-summary-CHAR) → \[U+0000-U+D7FF U+E000-U+10FFFF\] // a Unicode scalar value

[ASCII](#railroad-summary-ASCII) → \[U+0000-U+007F\]

[NUL](#railroad-summary-NUL) → U+0000

[EOF](#railroad-summary-EOF) → \![CHAR](#grammar-summary-CHAR) // End of file or input

[STRICT_KEYWORDS](#railroad-summary-STRICT_KEYWORDS) →\
      \_\
    \| as\
    \| async\
    \| await\
    \| break\
    \| const\
    \| continue\
    \| crate\
    \| dyn\
    \| else\
    \| enum\
    \| extern\
    \| false\
    \| fn\
    \| for\
    \| if\
    \| impl\
    \| in\
    \| let\
    \| loop\
    \| match\
    \| mod\
    \| move\
    \| mut\
    \| pub\
    \| ref\
    \| return\
    \| self\
    \| Self\
    \| static\
    \| struct\
    \| super\
    \| trait\
    \| true\
    \| type\
    \| unsafe\
    \| use\
    \| where\
    \| while

[RESERVED_KEYWORDS](#railroad-summary-RESERVED_KEYWORDS) →\
      abstract\
    \| become\
    \| box\
    \| do\
    \| final\
    \| gen\
    \| macro\
    \| override\
    \| priv\
    \| try\
    \| typeof\
    \| unsized\
    \| virtual\
    \| yield

[WEAK_KEYWORDS](#railroad-summary-WEAK_KEYWORDS) →\
      'static\
    \| macro_rules\
    \| raw\
    \| safe\
    \| union

[SHEBANG](#railroad-summary-SHEBANG) →\
    #! !( ( [WHITESPACE](#grammar-summary-WHITESPACE) \| [LINE_COMMENT](#grammar-summary-LINE_COMMENT) \| [SHEBANG_BLOCK_COMMENT](#grammar-summary-SHEBANG_BLOCK_COMMENT) )\* \[ )\
    ~[LF](#grammar-summary-LF)\* ( [LF](#grammar-summary-LF) \| [EOF](#grammar-summary-EOF) )

[SHEBANG_BLOCK_COMMENT](#railroad-summary-SHEBANG_BLOCK_COMMENT) →\
    /\* !( ! \| \* \![\* /\] )\
      ( [SHEBANG_NESTED_BLOCK_COMMENT](#grammar-summary-SHEBANG_NESTED_BLOCK_COMMENT) \| ( !( \*/ \| /\* ) [CHAR](#grammar-summary-CHAR) ) )\*\
    \*/

[SHEBANG_NESTED_BLOCK_COMMENT](#railroad-summary-SHEBANG_NESTED_BLOCK_COMMENT) →\
    /\*\
      ( [SHEBANG_NESTED_BLOCK_COMMENT](#grammar-summary-SHEBANG_NESTED_BLOCK_COMMENT) \| ( !( \*/ \| /\* ) [CHAR](#grammar-summary-CHAR) ) )\*\
    \*/

[Token](#railroad-summary-Token) →\
      [RESERVED_TOKEN](#grammar-summary-RESERVED_TOKEN)\
    \| [RAW_IDENTIFIER](#grammar-summary-RAW_IDENTIFIER)\
    \| [CHAR_LITERAL](#grammar-summary-CHAR_LITERAL)\
    \| [STRING_LITERAL](#grammar-summary-STRING_LITERAL)\
    \| [RAW_STRING_LITERAL](#grammar-summary-RAW_STRING_LITERAL)\
    \| [BYTE_LITERAL](#grammar-summary-BYTE_LITERAL)\
    \| [BYTE_STRING_LITERAL](#grammar-summary-BYTE_STRING_LITERAL)\
    \| [RAW_BYTE_STRING_LITERAL](#grammar-summary-RAW_BYTE_STRING_LITERAL)\
    \| [C_STRING_LITERAL](#grammar-summary-C_STRING_LITERAL)\
    \| [RAW_C_STRING_LITERAL](#grammar-summary-RAW_C_STRING_LITERAL)\
    \| [FLOAT_LITERAL](#grammar-summary-FLOAT_LITERAL)\
    \| [INTEGER_LITERAL](#grammar-summary-INTEGER_LITERAL)\
    \| [LIFETIME_TOKEN](#grammar-summary-LIFETIME_TOKEN)\
    \| [PUNCTUATION](#grammar-summary-PUNCTUATION)\
    \| [IDENTIFIER_OR_KEYWORD](#grammar-summary-IDENTIFIER_OR_KEYWORD)

[SUFFIX](#railroad-summary-SUFFIX) →\
      \_ ^ [XID_Continue](#grammar-summary-XID_Continue)+\
    \| [XID_Start](#grammar-summary-XID_Start) [XID_Continue](#grammar-summary-XID_Continue)\*

[CHAR_LITERAL](#railroad-summary-CHAR_LITERAL) →\
    '\
        ( ~\[' \\ [LF](#grammar-summary-LF) [CR](#grammar-summary-CR) [TAB](#grammar-summary-TAB)\] \| [QUOTE_ESCAPE](#grammar-summary-QUOTE_ESCAPE) \| [ASCII_ESCAPE](#grammar-summary-ASCII_ESCAPE) \| [UNICODE_ESCAPE](#grammar-summary-UNICODE_ESCAPE) )\
    ' [SUFFIX](#grammar-summary-SUFFIX)?

[QUOTE_ESCAPE](#railroad-summary-QUOTE_ESCAPE) → \\ \| \\

[ASCII_ESCAPE](#railroad-summary-ASCII_ESCAPE) →\
      \x [OCT_DIGIT](#grammar-summary-OCT_DIGIT) [HEX_DIGIT](#grammar-summary-HEX_DIGIT)\
    \| \n \| \r \| \t \| \\ \| \0

[UNICODE_ESCAPE](#railroad-summary-UNICODE_ESCAPE) →\
    \u{ ( [HEX_DIGIT](#grammar-summary-HEX_DIGIT) \_\* )1..=6valid hex char value }

[STRING_LITERAL](#railroad-summary-STRING_LITERAL) →\
    " (\
        ~\[" \\ [CR](#grammar-summary-CR)\]\
      \| [QUOTE_ESCAPE](#grammar-summary-QUOTE_ESCAPE)\
      \| [ASCII_ESCAPE](#grammar-summary-ASCII_ESCAPE)\
      \| [UNICODE_ESCAPE](#grammar-summary-UNICODE_ESCAPE)\
      \| [STRING_CONTINUE](#grammar-summary-STRING_CONTINUE)\
    )\* " [SUFFIX](#grammar-summary-SUFFIX)?

[STRING_CONTINUE](#railroad-summary-STRING_CONTINUE) → \\ [LF](#grammar-summary-LF) \[[TAB](#grammar-summary-TAB) [LF](#grammar-summary-LF) [CR](#grammar-summary-CR) [SP](#grammar-summary-SP)\]\*

[RAW_STRING_LITERAL](#railroad-summary-RAW_STRING_LITERAL) →\
      r " ^ [RAW_STRING_CONTENT](#grammar-summary-RAW_STRING_CONTENT) " [SUFFIX](#grammar-summary-SUFFIX)?\
    \| r \#n:1..=255 ^ " [RAW_STRING_CONTENT_HASHED](#grammar-summary-RAW_STRING_CONTENT_HASHED) " \#n [SUFFIX](#grammar-summary-SUFFIX)?

[RAW_STRING_CONTENT](#railroad-summary-RAW_STRING_CONTENT) → ( !" ~[CR](#grammar-summary-CR) )\*

[RAW_STRING_CONTENT_HASHED](#railroad-summary-RAW_STRING_CONTENT_HASHED) → ( !( " \#n ) ~[CR](#grammar-summary-CR) )\*

[BYTE_LITERAL](#railroad-summary-BYTE_LITERAL) →\
    b' ^ ( [ASCII_FOR_CHAR](#grammar-summary-ASCII_FOR_CHAR) \| [BYTE_ESCAPE](#grammar-summary-BYTE_ESCAPE) ) ' [SUFFIX](#grammar-summary-SUFFIX)?

[ASCII_FOR_CHAR](#railroad-summary-ASCII_FOR_CHAR) → \![' \\ [LF](#grammar-summary-LF) [CR](#grammar-summary-CR) [TAB](#grammar-summary-TAB)\] [ASCII](#grammar-summary-ASCII)

[BYTE_ESCAPE](#railroad-summary-BYTE_ESCAPE) →\
      \x [HEX_DIGIT](#grammar-summary-HEX_DIGIT) [HEX_DIGIT](#grammar-summary-HEX_DIGIT)\
    \| \n \| \r \| \t \| \\ \| \0 \| \\ \| \\

[BYTE_STRING_LITERAL](#railroad-summary-BYTE_STRING_LITERAL) →\
    b" ^ ( [ASCII_FOR_STRING](#grammar-summary-ASCII_FOR_STRING) \| [BYTE_ESCAPE](#grammar-summary-BYTE_ESCAPE) \| [STRING_CONTINUE](#grammar-summary-STRING_CONTINUE) )\* " [SUFFIX](#grammar-summary-SUFFIX)?

[ASCII_FOR_STRING](#railroad-summary-ASCII_FOR_STRING) → \![" \\ [CR](#grammar-summary-CR)\] [ASCII](#grammar-summary-ASCII)

[RAW_BYTE_STRING_LITERAL](#railroad-summary-RAW_BYTE_STRING_LITERAL) →\
      br " ^ [RAW_BYTE_STRING_CONTENT](#grammar-summary-RAW_BYTE_STRING_CONTENT) " [SUFFIX](#grammar-summary-SUFFIX)?\
    \| br \#n:1..=255 ^ " [RAW_BYTE_STRING_CONTENT_HASHED](#grammar-summary-RAW_BYTE_STRING_CONTENT_HASHED) " \#n [SUFFIX](#grammar-summary-SUFFIX)?

[RAW_BYTE_STRING_CONTENT](#railroad-summary-RAW_BYTE_STRING_CONTENT) → ( !" [ASCII_FOR_RAW](#grammar-summary-ASCII_FOR_RAW) )\*

[RAW_BYTE_STRING_CONTENT_HASHED](#railroad-summary-RAW_BYTE_STRING_CONTENT_HASHED) → ( !( " \#n ) [ASCII_FOR_RAW](#grammar-summary-ASCII_FOR_RAW) )\*

[ASCII_FOR_RAW](#railroad-summary-ASCII_FOR_RAW) → \![CR](#grammar-summary-CR) [ASCII](#grammar-summary-ASCII)

[C_STRING_LITERAL](#railroad-summary-C_STRING_LITERAL) →\
    c" ^ (\
        ~\[" \\ [CR](#grammar-summary-CR) [NUL](#grammar-summary-NUL)\]\
      \| !( \0 \| \x00 ) [BYTE_ESCAPE](#grammar-summary-BYTE_ESCAPE)\
      \| !( \u{ ( 0 \_\* )1..=6 } ) [UNICODE_ESCAPE](#grammar-summary-UNICODE_ESCAPE)\
      \| [STRING_CONTINUE](#grammar-summary-STRING_CONTINUE)\
    )\* " [SUFFIX](#grammar-summary-SUFFIX)?

[RAW_C_STRING_LITERAL](#railroad-summary-RAW_C_STRING_LITERAL) →\
      cr " ^ [RAW_C_STRING_CONTENT](#grammar-summary-RAW_C_STRING_CONTENT) " [SUFFIX](#grammar-summary-SUFFIX)?\
    \| cr \#n:1..=255 ^ " [RAW_C_STRING_CONTENT_HASHED](#grammar-summary-RAW_C_STRING_CONTENT_HASHED) " \#n [SUFFIX](#grammar-summary-SUFFIX)?

[RAW_C_STRING_CONTENT](#railroad-summary-RAW_C_STRING_CONTENT) → ( !" ~\[[CR](#grammar-summary-CR) [NUL](#grammar-summary-NUL)\] )\*

[RAW_C_STRING_CONTENT_HASHED](#railroad-summary-RAW_C_STRING_CONTENT_HASHED) → ( !( " \#n ) ~\[[CR](#grammar-summary-CR) [NUL](#grammar-summary-NUL)\] )\*

[INTEGER_LITERAL](#railroad-summary-INTEGER_LITERAL) →\
    ( [BIN_LITERAL](#grammar-summary-BIN_LITERAL) \| [OCT_LITERAL](#grammar-summary-OCT_LITERAL) \| [HEX_LITERAL](#grammar-summary-HEX_LITERAL) \| [DEC_LITERAL](#grammar-summary-DEC_LITERAL) )\
    ^ \![RESERVED_FLOAT](#grammar-summary-RESERVED_FLOAT) [SUFFIX](#grammar-summary-SUFFIX)?

[DEC_LITERAL](#railroad-summary-DEC_LITERAL) → [DEC_DIGIT](#grammar-summary-DEC_DIGIT) ( [DEC_DIGIT](#grammar-summary-DEC_DIGIT) \| \_ )\*

[BIN_LITERAL](#railroad-summary-BIN_LITERAL) → 0b ^ \_\* [BIN_DIGIT](#grammar-summary-BIN_DIGIT) ( [BIN_DIGIT](#grammar-summary-BIN_DIGIT) \| \_ )\* \![e E 2-9\]

[OCT_LITERAL](#railroad-summary-OCT_LITERAL) → 0o ^ \_\* [OCT_DIGIT](#grammar-summary-OCT_DIGIT) ( [OCT_DIGIT](#grammar-summary-OCT_DIGIT) \| \_ )\* \![e E 8-9\]

[HEX_LITERAL](#railroad-summary-HEX_LITERAL) → 0x ^ \_\* [HEX_DIGIT](#grammar-summary-HEX_DIGIT) ( [HEX_DIGIT](#grammar-summary-HEX_DIGIT) \| \_ )\*

[BIN_DIGIT](#railroad-summary-BIN_DIGIT) → \[0-1\]

[OCT_DIGIT](#railroad-summary-OCT_DIGIT) → \[0-7\]

[DEC_DIGIT](#railroad-summary-DEC_DIGIT) → \[0-9\]

[HEX_DIGIT](#railroad-summary-HEX_DIGIT) → \[0-9 a-f A-F\]

[RESERVED_FLOAT](#railroad-summary-RESERVED_FLOAT) → . !( . \| \_ \| [XID_Start](#grammar-summary-XID_Start) )

[TUPLE_INDEX](#railroad-summary-TUPLE_INDEX) → [DEC_LITERAL](#grammar-summary-DEC_LITERAL) \| [BIN_LITERAL](#grammar-summary-BIN_LITERAL) \| [OCT_LITERAL](#grammar-summary-OCT_LITERAL) \| [HEX_LITERAL](#grammar-summary-HEX_LITERAL)

[FLOAT_LITERAL](#railroad-summary-FLOAT_LITERAL) →\
      [DEC_LITERAL](#grammar-summary-DEC_LITERAL) ( . [DEC_LITERAL](#grammar-summary-DEC_LITERAL) )? [FLOAT_EXPONENT](#grammar-summary-FLOAT_EXPONENT) [SUFFIX](#grammar-summary-SUFFIX)?\
    \| [DEC_LITERAL](#grammar-summary-DEC_LITERAL) . [DEC_LITERAL](#grammar-summary-DEC_LITERAL) [SUFFIX](#grammar-summary-SUFFIX)?\
    \| [DEC_LITERAL](#grammar-summary-DEC_LITERAL) . !( . \| \_ \| [XID_Start](#grammar-summary-XID_Start) )

[FLOAT_EXPONENT](#railroad-summary-FLOAT_EXPONENT) →\
    ( e \| E ) ^ ( + \| - )? \_\* [DEC_DIGIT](#grammar-summary-DEC_DIGIT) ( [DEC_DIGIT](#grammar-summary-DEC_DIGIT) \| \_ )\*

[LIFETIME_TOKEN](#railroad-summary-LIFETIME_TOKEN) →\
      [RAW_LIFETIME](#grammar-summary-RAW_LIFETIME)\
    \| ' [IDENTIFIER_OR_KEYWORD](#grammar-summary-IDENTIFIER_OR_KEYWORD) !'

[LIFETIME_OR_LABEL](#railroad-summary-LIFETIME_OR_LABEL) →\
      [RAW_LIFETIME](#grammar-summary-RAW_LIFETIME)\
    \| ' [NON_KEYWORD_IDENTIFIER](#grammar-summary-NON_KEYWORD_IDENTIFIER) !'

[RAW_LIFETIME](#railroad-summary-RAW_LIFETIME) →\
    'r# ^ [IDENTIFIER_OR_KEYWORD](#grammar-summary-IDENTIFIER_OR_KEYWORD) !'

[RESERVED_RAW_LIFETIME](#railroad-summary-RESERVED_RAW_LIFETIME) → 'r# ( \_ \| crate \| self \| Self \| super ) !( ' \| [XID_Continue](#grammar-summary-XID_Continue) )

[PUNCTUATION](#railroad-summary-PUNCTUATION) →\
      ...\
    \| ..=\
    \| \<\<=\
    \| \>\>=\
    \| !=\
    \| %=\
    \| &&\
    \| &=\
    \| \*=\
    \| +=\
    \| -=\
    \| -\>\
    \| ..\
    \| /=\
    \| ::\
    \| \<-\
    \| \<\<\
    \| \<=\
    \| ==\
    \| =\>\
    \| \>=\
    \| \>\>\
    \| ^=\
    \| \|=\
    \| \|\|\
    \| !\
    \| \#\
    \| \$\
    \| %\
    \| &\
    \| (\
    \| )\
    \| \*\
    \| +\
    \| ,\
    \| -\
    \| .\
    \| /\
    \| :\
    \| ;\
    \| \<\
    \| =\
    \| \>\
    \| ?\
    \| @\
    \| \[\
    \| \]\
    \| ^\
    \| {\
    \| \|\
    \| }\
    \| ~

[RESERVED_TOKEN](#railroad-summary-RESERVED_TOKEN) →\
      [RESERVED_GUARDED_STRING_LITERAL](#grammar-summary-RESERVED_GUARDED_STRING_LITERAL)\
    \| [RESERVED_POUNDS](#grammar-summary-RESERVED_POUNDS)\
    \| [RESERVED_RAW_IDENTIFIER](#grammar-summary-RESERVED_RAW_IDENTIFIER)\
    \| [RESERVED_RAW_LIFETIME](#grammar-summary-RESERVED_RAW_LIFETIME)\
    \| [RESERVED_TOKEN_DOUBLE_QUOTE](#grammar-summary-RESERVED_TOKEN_DOUBLE_QUOTE)\
    \| [RESERVED_TOKEN_LIFETIME](#grammar-summary-RESERVED_TOKEN_LIFETIME)\
    \| [RESERVED_TOKEN_POUND](#grammar-summary-RESERVED_TOKEN_POUND)\
    \| [RESERVED_TOKEN_SINGLE_QUOTE](#grammar-summary-RESERVED_TOKEN_SINGLE_QUOTE)

[RESERVED_TOKEN_DOUBLE_QUOTE](#railroad-summary-RESERVED_TOKEN_DOUBLE_QUOTE) →\
    [IDENTIFIER_OR_KEYWORD](#grammar-summary-IDENTIFIER_OR_KEYWORD)except `b` or `c` or `r` or `br` or `cr` "

[RESERVED_TOKEN_SINGLE_QUOTE](#railroad-summary-RESERVED_TOKEN_SINGLE_QUOTE) →\
    [IDENTIFIER_OR_KEYWORD](#grammar-summary-IDENTIFIER_OR_KEYWORD)except `b` '

[RESERVED_TOKEN_POUND](#railroad-summary-RESERVED_TOKEN_POUND) →\
    [IDENTIFIER_OR_KEYWORD](#grammar-summary-IDENTIFIER_OR_KEYWORD)except `r` or `br` or `cr` \#

[RESERVED_TOKEN_LIFETIME](#railroad-summary-RESERVED_TOKEN_LIFETIME) →\
    ' [IDENTIFIER_OR_KEYWORD](#grammar-summary-IDENTIFIER_OR_KEYWORD)except `r` \#

[RESERVED_GUARDED_STRING_LITERAL](#railroad-summary-RESERVED_GUARDED_STRING_LITERAL) → \#+ [STRING_LITERAL](#grammar-summary-STRING_LITERAL)

[RESERVED_POUNDS](#railroad-summary-RESERVED_POUNDS) → \#2..

[WHITESPACE](#railroad-summary-WHITESPACE) →\
      U+0009 // Horizontal tab, `'\t'`\
    \| U+000A // Line feed, `'\n'`\
    \| U+000B // Vertical tab\
    \| U+000C // Form feed\
    \| U+000D // Carriage return, `'\r'`\
    \| U+0020 // Space, `' '`\
    \| U+0085 // Next line\
    \| U+200E // Left-to-right mark\
    \| U+200F // Right-to-left mark\
    \| U+2028 // Line separator\
    \| U+2029 // Paragraph separator

[TAB](#railroad-summary-TAB) → U+0009 // Horizontal tab, `'\t'`

[LF](#railroad-summary-LF) → U+000A // Line feed, `'\n'`

[CR](#railroad-summary-CR) → U+000D // Carriage return, `'\r'`

[SP](#railroad-summary-SP) → U+0020 // Space, `' '`

Show Railroad

![](../assets/inline/grammar-19.svg) ![](../assets/inline/grammar-20.svg) ![](../assets/inline/grammar-21.svg) ![](../assets/inline/grammar-22.svg) ![](../assets/inline/grammar-23.svg) ![](../assets/inline/grammar-24.svg) ![](../assets/inline/grammar-25.svg) ![](../assets/inline/grammar-26.svg) ![](../assets/inline/grammar-27.svg) ![](../assets/inline/grammar-28.svg) ![](../assets/inline/grammar-29.svg) ![](../assets/inline/grammar-30.svg) ![](../assets/inline/grammar-31.svg) ![](../assets/inline/grammar-32.svg) ![](../assets/inline/grammar-33.svg) ![](../assets/inline/grammar-34.svg) ![](../assets/inline/grammar-35.svg) ![](../assets/inline/grammar-36.svg) ![](../assets/inline/grammar-37.svg) ![](../assets/inline/grammar-38.svg) ![](../assets/inline/grammar-39.svg) ![](../assets/inline/grammar-40.svg) ![](../assets/inline/grammar-41.svg) ![](../assets/inline/grammar-42.svg) ![](../assets/inline/grammar-43.svg) ![](../assets/inline/grammar-44.svg) ![](../assets/inline/grammar-45.svg) ![](../assets/inline/grammar-46.svg) ![](../assets/inline/grammar-47.svg) ![](../assets/inline/grammar-48.svg) ![](../assets/inline/grammar-49.svg) ![](../assets/inline/grammar-50.svg) ![](../assets/inline/grammar-51.svg) ![](../assets/inline/grammar-52.svg) ![](../assets/inline/grammar-53.svg) ![](../assets/inline/grammar-54.svg) ![](../assets/inline/grammar-55.svg) ![](../assets/inline/grammar-56.svg) ![](../assets/inline/grammar-57.svg) ![](../assets/inline/grammar-58.svg) ![](../assets/inline/grammar-59.svg) ![](../assets/inline/grammar-60.svg) ![](../assets/inline/grammar-61.svg) ![](../assets/inline/grammar-62.svg) ![](../assets/inline/grammar-63.svg) ![](../assets/inline/grammar-64.svg) ![](../assets/inline/grammar-65.svg) ![](../assets/inline/grammar-66.svg) ![](../assets/inline/grammar-67.svg) ![](../assets/inline/grammar-68.svg) ![](../assets/inline/grammar-69.svg) ![](../assets/inline/grammar-70.svg) ![](../assets/inline/grammar-71.svg) ![](../assets/inline/grammar-72.svg) ![](../assets/inline/grammar-73.svg) ![](../assets/inline/grammar-74.svg) ![](../assets/inline/grammar-75.svg) ![](../assets/inline/grammar-76.svg) ![](../assets/inline/grammar-77.svg) ![](../assets/inline/grammar-78.svg) ![](../assets/inline/grammar-79.svg) ![](../assets/inline/grammar-80.svg) ![](../assets/inline/grammar-81.svg) ![](../assets/inline/grammar-82.svg) ![](../assets/inline/grammar-83.svg) ![](../assets/inline/grammar-84.svg) ![](../assets/inline/grammar-85.svg) ![](../assets/inline/grammar-86.svg) ![](../assets/inline/grammar-87.svg) ![](../assets/inline/grammar-88.svg) ![](../assets/inline/grammar-89.svg) ![](../assets/inline/grammar-90.svg) ![](../assets/inline/grammar-91.svg) ![](../assets/inline/grammar-92.svg) ![](../assets/inline/grammar-93.svg) ![](../assets/inline/grammar-94.svg) ![](../assets/inline/grammar-95.svg) ![](../assets/inline/grammar-96.svg) ![](../assets/inline/grammar-97.svg) ![](../assets/inline/grammar-98.svg) ![](../assets/inline/grammar-99.svg) ![](../assets/inline/grammar-100.svg)

## Configuration summary

**Syntax**\
[ConfigurationPredicate](#railroad-summary-ConfigurationPredicate) →\
      [ConfigurationOption](#grammar-summary-ConfigurationOption)\
    \| [ConfigurationAll](#grammar-summary-ConfigurationAll)\
    \| [ConfigurationAny](#grammar-summary-ConfigurationAny)\
    \| [ConfigurationNot](#grammar-summary-ConfigurationNot)\
    \| true\
    \| false

[ConfigurationOption](#railroad-summary-ConfigurationOption) →\
    [IDENTIFIER](#grammar-summary-IDENTIFIER) ( = ( [STRING_LITERAL](#grammar-summary-STRING_LITERAL) \| [RAW_STRING_LITERAL](#grammar-summary-RAW_STRING_LITERAL) ) )?

[ConfigurationAll](#railroad-summary-ConfigurationAll) →\
    all ( [ConfigurationPredicateList](#grammar-summary-ConfigurationPredicateList)? )

[ConfigurationAny](#railroad-summary-ConfigurationAny) →\
    any ( [ConfigurationPredicateList](#grammar-summary-ConfigurationPredicateList)? )

[ConfigurationNot](#railroad-summary-ConfigurationNot) →\
    not ( [ConfigurationPredicate](#grammar-summary-ConfigurationPredicate) )

[ConfigurationPredicateList](#railroad-summary-ConfigurationPredicateList) →\
    [ConfigurationPredicate](#grammar-summary-ConfigurationPredicate) ( , [ConfigurationPredicate](#grammar-summary-ConfigurationPredicate) )\* ,?

[CfgAttribute](#railroad-summary-CfgAttribute) → cfg ( [ConfigurationPredicate](#grammar-summary-ConfigurationPredicate) )

[CfgAttrAttribute](#railroad-summary-CfgAttrAttribute) → cfg_attr ( [ConfigurationPredicate](#grammar-summary-ConfigurationPredicate) , [CfgAttrs](#grammar-summary-CfgAttrs)? )

[CfgAttrs](#railroad-summary-CfgAttrs) → [Attr](#grammar-summary-Attr) ( , [Attr](#grammar-summary-Attr) )\* ,?

[CfgSelect](#railroad-summary-CfgSelect) → [CfgSelectArms](#grammar-summary-CfgSelectArms)?

[CfgSelectArms](#railroad-summary-CfgSelectArms) →\
    [CfgSelectConfigurationPredicate](#grammar-summary-CfgSelectConfigurationPredicate) =\>\
    (\
        { ^ [TokenTree](#grammar-summary-TokenTree) } ,? [CfgSelectArms](#grammar-summary-CfgSelectArms)?\
      \| [ExpressionWithBlockNoAttrs](#grammar-summary-ExpressionWithBlockNoAttrs) ,? [CfgSelectArms](#grammar-summary-CfgSelectArms)?\
      \| [ExpressionWithoutBlockNoAttrs](#grammar-summary-ExpressionWithoutBlockNoAttrs) ( , [CfgSelectArms](#grammar-summary-CfgSelectArms)? )?\
    )

[CfgSelectConfigurationPredicate](#railroad-summary-CfgSelectConfigurationPredicate) →\
    [ConfigurationPredicate](#grammar-summary-ConfigurationPredicate) \| \_

Show Railroad

![](../assets/inline/grammar-101.svg) ![](../assets/inline/grammar-102.svg) ![](../assets/inline/grammar-103.svg) ![](../assets/inline/grammar-104.svg) ![](../assets/inline/grammar-105.svg) ![](../assets/inline/grammar-106.svg) ![](../assets/inline/grammar-107.svg) ![](../assets/inline/grammar-108.svg) ![](../assets/inline/grammar-109.svg) ![](../assets/inline/grammar-110.svg) ![](../assets/inline/grammar-111.svg) ![](../assets/inline/grammar-112.svg)

## Items summary

**Syntax**\
[Crate](#railroad-summary-Crate) →\
    [InnerAttribute](#grammar-summary-InnerAttribute)\*\
    [Item](#grammar-summary-Item)\*

[AssociatedItem](#railroad-summary-AssociatedItem) →\
    [OuterAttribute](#grammar-summary-OuterAttribute)\* (\
        [MacroInvocationSemi](#grammar-summary-MacroInvocationSemi)\
      \| ( [Visibility](#grammar-summary-Visibility)? ( [TypeAlias](#grammar-summary-TypeAlias) \| [ConstantItem](#grammar-summary-ConstantItem) \| [Function](#grammar-summary-Function) ) )\
    )

[ConstantItem](#railroad-summary-ConstantItem) →\
    const ( [IDENTIFIER](#grammar-summary-IDENTIFIER) \| \_ ) : [Type](#grammar-summary-Type) ( = [Expression](#grammar-summary-Expression) )? ;

[Enumeration](#railroad-summary-Enumeration) →\
    enum [IDENTIFIER](#grammar-summary-IDENTIFIER) [GenericParams](#grammar-summary-GenericParams)? [WhereClause](#grammar-summary-WhereClause)? { [EnumVariants](#grammar-summary-EnumVariants)? }

[EnumVariants](#railroad-summary-EnumVariants) → [EnumVariant](#grammar-summary-EnumVariant) ( , [EnumVariant](#grammar-summary-EnumVariant) )\* ,?

[EnumVariant](#railroad-summary-EnumVariant) →\
    [OuterAttribute](#grammar-summary-OuterAttribute)\* [Visibility](#grammar-summary-Visibility)?\
    [IDENTIFIER](#grammar-summary-IDENTIFIER) ( [EnumVariantTuple](#grammar-summary-EnumVariantTuple) \| [EnumVariantStruct](#grammar-summary-EnumVariantStruct) )? [EnumVariantDiscriminant](#grammar-summary-EnumVariantDiscriminant)?

[EnumVariantTuple](#railroad-summary-EnumVariantTuple) → ( [TupleFields](#grammar-summary-TupleFields)? )

[EnumVariantStruct](#railroad-summary-EnumVariantStruct) → { [StructFields](#grammar-summary-StructFields)? }

[EnumVariantDiscriminant](#railroad-summary-EnumVariantDiscriminant) → = [Expression](#grammar-summary-Expression)

[ExternCrate](#railroad-summary-ExternCrate) → extern crate [CrateRef](#grammar-summary-CrateRef) [AsClause](#grammar-summary-AsClause)? ;

[CrateRef](#railroad-summary-CrateRef) → [IDENTIFIER](#grammar-summary-IDENTIFIER) \| self

[AsClause](#railroad-summary-AsClause) → as ( [IDENTIFIER](#grammar-summary-IDENTIFIER) \| \_ )

[ExternBlock](#railroad-summary-ExternBlock) →\
    unsafe? extern [Abi](#grammar-summary-Abi)? {\
        [InnerAttribute](#grammar-summary-InnerAttribute)\*\
        [ExternalItem](#grammar-summary-ExternalItem)\*\
    }

[ExternalItem](#railroad-summary-ExternalItem) →\
    [OuterAttribute](#grammar-summary-OuterAttribute)\* (\
        [MacroInvocationSemi](#grammar-summary-MacroInvocationSemi)\
      \| [Visibility](#grammar-summary-Visibility)? [StaticItem](#grammar-summary-StaticItem)\
      \| [Visibility](#grammar-summary-Visibility)? [Function](#grammar-summary-Function)\
    )

[Function](#railroad-summary-Function) →\
    [FunctionQualifiers](#grammar-summary-FunctionQualifiers) fn [IDENTIFIER](#grammar-summary-IDENTIFIER) [GenericParams](#grammar-summary-GenericParams)?\
        ( [FunctionParameters](#grammar-summary-FunctionParameters)? )\
        [FunctionReturnType](#grammar-summary-FunctionReturnType)? [WhereClause](#grammar-summary-WhereClause)?\
        ( [BlockExpression](#grammar-summary-BlockExpression) \| ; )

[FunctionQualifiers](#railroad-summary-FunctionQualifiers) → const? async? [ItemSafety](#grammar-summary-ItemSafety)? ( extern [Abi](#grammar-summary-Abi)? )?

[ItemSafety](#railroad-summary-ItemSafety) → safe \| unsafe

[Abi](#railroad-summary-Abi) → [STRING_LITERAL](#grammar-summary-STRING_LITERAL) \| [RAW_STRING_LITERAL](#grammar-summary-RAW_STRING_LITERAL)

[FunctionParameters](#railroad-summary-FunctionParameters) →\
      [SelfParam](#grammar-summary-SelfParam) ,?\
    \| ( [SelfParam](#grammar-summary-SelfParam) , )? [FunctionParam](#grammar-summary-FunctionParam) ( , [FunctionParam](#grammar-summary-FunctionParam) )\* ,?

[SelfParam](#railroad-summary-SelfParam) → [OuterAttribute](#grammar-summary-OuterAttribute)\* ( [ShorthandSelf](#grammar-summary-ShorthandSelf) \| [TypedSelf](#grammar-summary-TypedSelf) )

[ShorthandSelf](#railroad-summary-ShorthandSelf) → ( & \| & [Lifetime](#grammar-summary-Lifetime) )? mut? self

[TypedSelf](#railroad-summary-TypedSelf) → mut? self : [Type](#grammar-summary-Type)

[FunctionParam](#railroad-summary-FunctionParam) → [OuterAttribute](#grammar-summary-OuterAttribute)\* ( [FunctionParamPattern](#grammar-summary-FunctionParamPattern) \| ... \| [Type](#grammar-summary-Type) )

[FunctionParamPattern](#railroad-summary-FunctionParamPattern) → [PatternNoTopAlt](#grammar-summary-PatternNoTopAlt) : ( [Type](#grammar-summary-Type) \| ... )

[FunctionReturnType](#railroad-summary-FunctionReturnType) → -\> [Type](#grammar-summary-Type)

[GenericParams](#railroad-summary-GenericParams) → \< ( [GenericParam](#grammar-summary-GenericParam) ( , [GenericParam](#grammar-summary-GenericParam) )\* ,? )? \>

[GenericParam](#railroad-summary-GenericParam) → [OuterAttribute](#grammar-summary-OuterAttribute)\* ( [LifetimeParam](#grammar-summary-LifetimeParam) \| [TypeParam](#grammar-summary-TypeParam) \| [ConstParam](#grammar-summary-ConstParam) )

[LifetimeParam](#railroad-summary-LifetimeParam) → [Lifetime](#grammar-summary-Lifetime) ( : [LifetimeBounds](#grammar-summary-LifetimeBounds)? )?

[TypeParam](#railroad-summary-TypeParam) → [IDENTIFIER](#grammar-summary-IDENTIFIER) ( : [Bounds](#grammar-summary-Bounds)? )? ( = [Type](#grammar-summary-Type) )?

[ConstParam](#railroad-summary-ConstParam) →\
    const [IDENTIFIER](#grammar-summary-IDENTIFIER) : [Type](#grammar-summary-Type)\
    ( = ( [BlockExpression](#grammar-summary-BlockExpression) \| [IDENTIFIER](#grammar-summary-IDENTIFIER) \| -? [LiteralExpression](#grammar-summary-LiteralExpression) ) )?

[WhereClause](#railroad-summary-WhereClause) → where ( [WhereClauseItem](#grammar-summary-WhereClauseItem) , )\* [WhereClauseItem](#grammar-summary-WhereClauseItem)?

[WhereClauseItem](#railroad-summary-WhereClauseItem) →\
      [LifetimeWhereClauseItem](#grammar-summary-LifetimeWhereClauseItem)\
    \| [TypeBoundWhereClauseItem](#grammar-summary-TypeBoundWhereClauseItem)

[LifetimeWhereClauseItem](#railroad-summary-LifetimeWhereClauseItem) → [Lifetime](#grammar-summary-Lifetime) : [LifetimeBounds](#grammar-summary-LifetimeBounds)?

[TypeBoundWhereClauseItem](#railroad-summary-TypeBoundWhereClauseItem) → [ForLifetimes](#grammar-summary-ForLifetimes)? [Type](#grammar-summary-Type) : [Bounds](#grammar-summary-Bounds)?

[Implementation](#railroad-summary-Implementation) → [InherentImpl](#grammar-summary-InherentImpl) \| [TraitImpl](#grammar-summary-TraitImpl)

[InherentImpl](#railroad-summary-InherentImpl) →\
    impl [GenericParams](#grammar-summary-GenericParams)? [Type](#grammar-summary-Type) [WhereClause](#grammar-summary-WhereClause)? {\
        [InnerAttribute](#grammar-summary-InnerAttribute)\*\
        [AssociatedItem](#grammar-summary-AssociatedItem)\*\
    }

[TraitImpl](#railroad-summary-TraitImpl) →\
    unsafe? impl [GenericParams](#grammar-summary-GenericParams)? !? [TypePath](#grammar-summary-TypePath) for [Type](#grammar-summary-Type)\
    [WhereClause](#grammar-summary-WhereClause)?\
    {\
        [InnerAttribute](#grammar-summary-InnerAttribute)\*\
        [AssociatedItem](#grammar-summary-AssociatedItem)\*\
    }

[Module](#railroad-summary-Module) →\
      unsafe? mod [IDENTIFIER](#grammar-summary-IDENTIFIER) ;\
    \| unsafe? mod [IDENTIFIER](#grammar-summary-IDENTIFIER) {\
        [InnerAttribute](#grammar-summary-InnerAttribute)\*\
        [Item](#grammar-summary-Item)\*\
      }

[StaticItem](#railroad-summary-StaticItem) →\
    [ItemSafety](#grammar-summary-ItemSafety)? static mut? [IDENTIFIER](#grammar-summary-IDENTIFIER) : [Type](#grammar-summary-Type) ( = [Expression](#grammar-summary-Expression) )? ;

[Struct](#railroad-summary-Struct) →\
      [StructStruct](#grammar-summary-StructStruct)\
    \| [TupleStruct](#grammar-summary-TupleStruct)

[StructStruct](#railroad-summary-StructStruct) →\
    struct [IDENTIFIER](#grammar-summary-IDENTIFIER) [GenericParams](#grammar-summary-GenericParams)? [WhereClause](#grammar-summary-WhereClause)? ( { [StructFields](#grammar-summary-StructFields)? } \| ; )

[TupleStruct](#railroad-summary-TupleStruct) →\
    struct [IDENTIFIER](#grammar-summary-IDENTIFIER) [GenericParams](#grammar-summary-GenericParams)? ( [TupleFields](#grammar-summary-TupleFields)? ) [WhereClause](#grammar-summary-WhereClause)? ;

[StructFields](#railroad-summary-StructFields) → [StructField](#grammar-summary-StructField) ( , [StructField](#grammar-summary-StructField) )\* ,?

[StructField](#railroad-summary-StructField) → [OuterAttribute](#grammar-summary-OuterAttribute)\* [Visibility](#grammar-summary-Visibility)? [IDENTIFIER](#grammar-summary-IDENTIFIER) : [Type](#grammar-summary-Type)

[TupleFields](#railroad-summary-TupleFields) → [TupleField](#grammar-summary-TupleField) ( , [TupleField](#grammar-summary-TupleField) )\* ,?

[TupleField](#railroad-summary-TupleField) → [OuterAttribute](#grammar-summary-OuterAttribute)\* [Visibility](#grammar-summary-Visibility)? [Type](#grammar-summary-Type)

[Trait](#railroad-summary-Trait) →\
    unsafe? trait [IDENTIFIER](#grammar-summary-IDENTIFIER) [GenericParams](#grammar-summary-GenericParams)? ( : [Bounds](#grammar-summary-Bounds)? )? [WhereClause](#grammar-summary-WhereClause)?\
    {\
        [InnerAttribute](#grammar-summary-InnerAttribute)\*\
        [AssociatedItem](#grammar-summary-AssociatedItem)\*\
    }

[TypeAlias](#railroad-summary-TypeAlias) →\
    type [IDENTIFIER](#grammar-summary-IDENTIFIER) [GenericParams](#grammar-summary-GenericParams)? ( : [Bounds](#grammar-summary-Bounds)? )?\
        [WhereClause](#grammar-summary-WhereClause)?\
        ( = [Type](#grammar-summary-Type) [WhereClause](#grammar-summary-WhereClause)? )? ;

[Union](#railroad-summary-Union) →\
    union [IDENTIFIER](#grammar-summary-IDENTIFIER) [GenericParams](#grammar-summary-GenericParams)? [WhereClause](#grammar-summary-WhereClause)? { [StructFields](#grammar-summary-StructFields)? }

[UseDeclaration](#railroad-summary-UseDeclaration) → use [UseTree](#grammar-summary-UseTree) ;

[UseTree](#railroad-summary-UseTree) →\
      ( [SimplePath](#grammar-summary-SimplePath)? :: )? \*\
    \| ( [SimplePath](#grammar-summary-SimplePath)? :: )? { ( [UseTree](#grammar-summary-UseTree) ( , [UseTree](#grammar-summary-UseTree) )\* ,? )? }\
    \| [SimplePath](#grammar-summary-SimplePath) ( as ( [IDENTIFIER](#grammar-summary-IDENTIFIER) \| \_ ) )?

[Item](#railroad-summary-Item) →\
    [OuterAttribute](#grammar-summary-OuterAttribute)\* ( [VisItem](#grammar-summary-VisItem) \| [MacroItem](#grammar-summary-MacroItem) )

[VisItem](#railroad-summary-VisItem) →\
    [Visibility](#grammar-summary-Visibility)?\
    (\
        [Module](#grammar-summary-Module)\
      \| [ExternCrate](#grammar-summary-ExternCrate)\
      \| [UseDeclaration](#grammar-summary-UseDeclaration)\
      \| [Function](#grammar-summary-Function)\
      \| [TypeAlias](#grammar-summary-TypeAlias)\
      \| [Struct](#grammar-summary-Struct)\
      \| [Enumeration](#grammar-summary-Enumeration)\
      \| [Union](#grammar-summary-Union)\
      \| [ConstantItem](#grammar-summary-ConstantItem)\
      \| [StaticItem](#grammar-summary-StaticItem)\
      \| [Trait](#grammar-summary-Trait)\
      \| [Implementation](#grammar-summary-Implementation)\
      \| [ExternBlock](#grammar-summary-ExternBlock)\
    )

[MacroItem](#railroad-summary-MacroItem) →\
      [MacroInvocationSemi](#grammar-summary-MacroInvocationSemi)\
    \| [MacroRulesDefinition](#grammar-summary-MacroRulesDefinition)

[Visibility](#railroad-summary-Visibility) →\
      pub\
    \| pub ( crate )\
    \| pub ( self )\
    \| pub ( super )\
    \| pub ( in [SimplePath](#grammar-summary-SimplePath) )

Show Railroad

![](../assets/inline/grammar-113.svg) ![](../assets/inline/grammar-114.svg) ![](../assets/inline/grammar-115.svg) ![](../assets/inline/grammar-116.svg) ![](../assets/inline/grammar-117.svg) ![](../assets/inline/grammar-118.svg) ![](../assets/inline/grammar-119.svg) ![](../assets/inline/grammar-120.svg) ![](../assets/inline/grammar-121.svg) ![](../assets/inline/grammar-122.svg) ![](../assets/inline/grammar-123.svg) ![](../assets/inline/grammar-124.svg) ![](../assets/inline/grammar-125.svg) ![](../assets/inline/grammar-126.svg) ![](../assets/inline/grammar-127.svg) ![](../assets/inline/grammar-128.svg) ![](../assets/inline/grammar-129.svg) ![](../assets/inline/grammar-130.svg) ![](../assets/inline/grammar-131.svg) ![](../assets/inline/grammar-132.svg) ![](../assets/inline/grammar-133.svg) ![](../assets/inline/grammar-134.svg) ![](../assets/inline/grammar-135.svg) ![](../assets/inline/grammar-136.svg) ![](../assets/inline/grammar-137.svg) ![](../assets/inline/grammar-138.svg) ![](../assets/inline/grammar-139.svg) ![](../assets/inline/grammar-140.svg) ![](../assets/inline/grammar-141.svg) ![](../assets/inline/grammar-142.svg) ![](../assets/inline/grammar-143.svg) ![](../assets/inline/grammar-144.svg) ![](../assets/inline/grammar-145.svg) ![](../assets/inline/grammar-146.svg) ![](../assets/inline/grammar-147.svg) ![](../assets/inline/grammar-148.svg) ![](../assets/inline/grammar-149.svg) ![](../assets/inline/grammar-150.svg) ![](../assets/inline/grammar-151.svg) ![](../assets/inline/grammar-152.svg) ![](../assets/inline/grammar-153.svg) ![](../assets/inline/grammar-154.svg) ![](../assets/inline/grammar-155.svg) ![](../assets/inline/grammar-156.svg) ![](../assets/inline/grammar-157.svg) ![](../assets/inline/grammar-158.svg) ![](../assets/inline/grammar-159.svg) ![](../assets/inline/grammar-160.svg) ![](../assets/inline/grammar-161.svg) ![](../assets/inline/grammar-162.svg) ![](../assets/inline/grammar-163.svg) ![](../assets/inline/grammar-164.svg) ![](../assets/inline/grammar-165.svg) ![](../assets/inline/grammar-166.svg) ![](../assets/inline/grammar-167.svg)

## Expressions summary

**Syntax**\
[ArrayExpression](#railroad-summary-ArrayExpression) → \[ [ArrayElements](#grammar-summary-ArrayElements)? \]

[ArrayElements](#railroad-summary-ArrayElements) →\
      [Expression](#grammar-summary-Expression) ( , [Expression](#grammar-summary-Expression) )\* ,?\
    \| [Expression](#grammar-summary-Expression) ; [Expression](#grammar-summary-Expression)

[IndexExpression](#railroad-summary-IndexExpression) → [Expression](#grammar-summary-Expression) \[ [Expression](#grammar-summary-Expression) \]

[AwaitExpression](#railroad-summary-AwaitExpression) → [Expression](#grammar-summary-Expression) . await

[BlockExpression](#railroad-summary-BlockExpression) →\
    {\
        [InnerAttribute](#grammar-summary-InnerAttribute)\*\
        [Statements](#grammar-summary-Statements)?\
    }

[BlockExpressionNoInnerAttributes](#railroad-summary-BlockExpressionNoInnerAttributes) →\
    {\
        [Statements](#grammar-summary-Statements)?\
    }

[Statements](#railroad-summary-Statements) →\
      [Statement](#grammar-summary-Statement)+\
    \| [Statement](#grammar-summary-Statement)+ [ExpressionWithoutBlock](#grammar-summary-ExpressionWithoutBlock)\
    \| [ExpressionWithoutBlock](#grammar-summary-ExpressionWithoutBlock)

[AsyncBlockExpression](#railroad-summary-AsyncBlockExpression) → async move? [BlockExpression](#grammar-summary-BlockExpression)

[ConstBlockExpression](#railroad-summary-ConstBlockExpression) → const [BlockExpression](#grammar-summary-BlockExpression)

[UnsafeBlockExpression](#railroad-summary-UnsafeBlockExpression) → unsafe [BlockExpression](#grammar-summary-BlockExpression)

[CallExpression](#railroad-summary-CallExpression) → [Expression](#grammar-summary-Expression) ( [CallParams](#grammar-summary-CallParams)? )

[CallParams](#railroad-summary-CallParams) → [Expression](#grammar-summary-Expression) ( , [Expression](#grammar-summary-Expression) )\* ,?

[ClosureExpression](#railroad-summary-ClosureExpression) →\
    async?\
    move?\
    ( \|\| \| \| [ClosureParameters](#grammar-summary-ClosureParameters)? \| )\
    ( [Expression](#grammar-summary-Expression) \| -\> [TypeNoBounds](#grammar-summary-TypeNoBounds) [BlockExpression](#grammar-summary-BlockExpression) )

[ClosureParameters](#railroad-summary-ClosureParameters) → [ClosureParam](#grammar-summary-ClosureParam) ( , [ClosureParam](#grammar-summary-ClosureParam) )\* ,?

[ClosureParam](#railroad-summary-ClosureParam) → [OuterAttribute](#grammar-summary-OuterAttribute)\* [PatternNoTopAlt](#grammar-summary-PatternNoTopAlt) ( : [Type](#grammar-summary-Type) )?

[FieldExpression](#railroad-summary-FieldExpression) → [Expression](#grammar-summary-Expression) . [IDENTIFIER](#grammar-summary-IDENTIFIER)

[GroupedExpression](#railroad-summary-GroupedExpression) → ( [Expression](#grammar-summary-Expression) )

[IfExpression](#railroad-summary-IfExpression) →\
    if [Conditions](#grammar-summary-Conditions) [BlockExpressionNoInnerAttributes](#grammar-summary-BlockExpressionNoInnerAttributes)\
    ( else ( [BlockExpressionNoInnerAttributes](#grammar-summary-BlockExpressionNoInnerAttributes) \| [IfExpression](#grammar-summary-IfExpression) ) )?

[Conditions](#railroad-summary-Conditions) →\
      [Expression](#grammar-summary-Expression)except [StructExpression](#grammar-summary-StructExpression)\
    \| [LetChain](#grammar-summary-LetChain)

[LetChain](#railroad-summary-LetChain) → [LetChainCondition](#grammar-summary-LetChainCondition) ( && [LetChainCondition](#grammar-summary-LetChainCondition) )\*

[LetChainCondition](#railroad-summary-LetChainCondition) →\
      [Expression](#grammar-summary-Expression)except [ExcludedConditions](#grammar-summary-ExcludedConditions)\
    \| [OuterAttribute](#grammar-summary-OuterAttribute)\* let [Pattern](#grammar-summary-Pattern) = [Scrutinee](#grammar-summary-Scrutinee)except [ExcludedConditions](#grammar-summary-ExcludedConditions)

[ExcludedConditions](#railroad-summary-ExcludedConditions) →\
      [StructExpression](#grammar-summary-StructExpression)\
    \| [LazyBooleanExpression](#grammar-summary-LazyBooleanExpression)\
    \| [RangeExpr](#grammar-summary-RangeExpr)\
    \| [RangeFromExpr](#grammar-summary-RangeFromExpr)\
    \| [RangeInclusiveExpr](#grammar-summary-RangeInclusiveExpr)\
    \| [AssignmentExpression](#grammar-summary-AssignmentExpression)\
    \| [CompoundAssignmentExpression](#grammar-summary-CompoundAssignmentExpression)

[LiteralExpression](#railroad-summary-LiteralExpression) →\
      [CHAR_LITERAL](#grammar-summary-CHAR_LITERAL)\
    \| [STRING_LITERAL](#grammar-summary-STRING_LITERAL)\
    \| [RAW_STRING_LITERAL](#grammar-summary-RAW_STRING_LITERAL)\
    \| [BYTE_LITERAL](#grammar-summary-BYTE_LITERAL)\
    \| [BYTE_STRING_LITERAL](#grammar-summary-BYTE_STRING_LITERAL)\
    \| [RAW_BYTE_STRING_LITERAL](#grammar-summary-RAW_BYTE_STRING_LITERAL)\
    \| [C_STRING_LITERAL](#grammar-summary-C_STRING_LITERAL)\
    \| [RAW_C_STRING_LITERAL](#grammar-summary-RAW_C_STRING_LITERAL)\
    \| [INTEGER_LITERAL](#grammar-summary-INTEGER_LITERAL)\
    \| [FLOAT_LITERAL](#grammar-summary-FLOAT_LITERAL)\
    \| true\
    \| false

[LoopExpression](#railroad-summary-LoopExpression) →\
    [LoopLabel](#grammar-summary-LoopLabel)? (\
        [InfiniteLoopExpression](#grammar-summary-InfiniteLoopExpression)\
      \| [PredicateLoopExpression](#grammar-summary-PredicateLoopExpression)\
      \| [IteratorLoopExpression](#grammar-summary-IteratorLoopExpression)\
      \| [LabelBlockExpression](#grammar-summary-LabelBlockExpression)\
    )

[InfiniteLoopExpression](#railroad-summary-InfiniteLoopExpression) → loop [BlockExpression](#grammar-summary-BlockExpression)

[PredicateLoopExpression](#railroad-summary-PredicateLoopExpression) → while [Conditions](#grammar-summary-Conditions) [BlockExpression](#grammar-summary-BlockExpression)

[IteratorLoopExpression](#railroad-summary-IteratorLoopExpression) →\
    for [Pattern](#grammar-summary-Pattern) in [Expression](#grammar-summary-Expression)except [StructExpression](#grammar-summary-StructExpression) [BlockExpression](#grammar-summary-BlockExpression)

[LoopLabel](#railroad-summary-LoopLabel) → [LIFETIME_OR_LABEL](#grammar-summary-LIFETIME_OR_LABEL) :

[BreakExpression](#railroad-summary-BreakExpression) → break [LIFETIME_OR_LABEL](#grammar-summary-LIFETIME_OR_LABEL)? [Expression](#grammar-summary-Expression)?

[LabelBlockExpression](#railroad-summary-LabelBlockExpression) → [BlockExpression](#grammar-summary-BlockExpression)

[ContinueExpression](#railroad-summary-ContinueExpression) → continue [LIFETIME_OR_LABEL](#grammar-summary-LIFETIME_OR_LABEL)?

[MatchExpression](#railroad-summary-MatchExpression) →\
    match [Scrutinee](#grammar-summary-Scrutinee) {\
        [InnerAttribute](#grammar-summary-InnerAttribute)\*\
        [MatchArms](#grammar-summary-MatchArms)?\
    }

[Scrutinee](#railroad-summary-Scrutinee) → [Expression](#grammar-summary-Expression)except [StructExpression](#grammar-summary-StructExpression)

[MatchArms](#railroad-summary-MatchArms) →\
    ( [MatchArm](#grammar-summary-MatchArm) =\> ( [ExpressionWithoutBlock](#grammar-summary-ExpressionWithoutBlock) , \| [ExpressionWithBlock](#grammar-summary-ExpressionWithBlock) ,? ) )\*\
    [MatchArm](#grammar-summary-MatchArm) =\> [Expression](#grammar-summary-Expression) ,?

[MatchArm](#railroad-summary-MatchArm) → [OuterAttribute](#grammar-summary-OuterAttribute)\* [Pattern](#grammar-summary-Pattern) [MatchArmGuard](#grammar-summary-MatchArmGuard)?

[MatchArmGuard](#railroad-summary-MatchArmGuard) → if [MatchConditions](#grammar-summary-MatchConditions)

[MatchConditions](#railroad-summary-MatchConditions) →\
     [MatchGuardChain](#grammar-summary-MatchGuardChain)\
   \| [Expression](#grammar-summary-Expression)

[MatchGuardChain](#railroad-summary-MatchGuardChain) → [MatchGuardCondition](#grammar-summary-MatchGuardCondition) ( && [MatchGuardCondition](#grammar-summary-MatchGuardCondition) )\*

[MatchGuardCondition](#railroad-summary-MatchGuardCondition) →\
     [Expression](#grammar-summary-Expression)except [ExcludedMatchConditions](#grammar-summary-ExcludedMatchConditions)\
   \| [OuterAttribute](#grammar-summary-OuterAttribute)\* let [Pattern](#grammar-summary-Pattern) = [MatchGuardScrutinee](#grammar-summary-MatchGuardScrutinee)

[MatchGuardScrutinee](#railroad-summary-MatchGuardScrutinee) → [Expression](#grammar-summary-Expression)except [ExcludedMatchConditions](#grammar-summary-ExcludedMatchConditions)

[ExcludedMatchConditions](#railroad-summary-ExcludedMatchConditions) →\
      [LazyBooleanExpression](#grammar-summary-LazyBooleanExpression)\
    \| [RangeExpr](#grammar-summary-RangeExpr)\
    \| [RangeFromExpr](#grammar-summary-RangeFromExpr)\
    \| [RangeInclusiveExpr](#grammar-summary-RangeInclusiveExpr)\
    \| [AssignmentExpression](#grammar-summary-AssignmentExpression)\
    \| [CompoundAssignmentExpression](#grammar-summary-CompoundAssignmentExpression)

[MethodCallExpression](#railroad-summary-MethodCallExpression) → [Expression](#grammar-summary-Expression) . [PathExprSegment](#grammar-summary-PathExprSegment) ( [CallParams](#grammar-summary-CallParams)? )

[OperatorExpression](#railroad-summary-OperatorExpression) →\
      [BorrowExpression](#grammar-summary-BorrowExpression)\
    \| [DereferenceExpression](#grammar-summary-DereferenceExpression)\
    \| [TryPropagationExpression](#grammar-summary-TryPropagationExpression)\
    \| [NegationExpression](#grammar-summary-NegationExpression)\
    \| [ArithmeticOrLogicalExpression](#grammar-summary-ArithmeticOrLogicalExpression)\
    \| [ComparisonExpression](#grammar-summary-ComparisonExpression)\
    \| [LazyBooleanExpression](#grammar-summary-LazyBooleanExpression)\
    \| [TypeCastExpression](#grammar-summary-TypeCastExpression)\
    \| [AssignmentExpression](#grammar-summary-AssignmentExpression)\
    \| [CompoundAssignmentExpression](#grammar-summary-CompoundAssignmentExpression)

[BorrowExpression](#railroad-summary-BorrowExpression) →\
      ( & \| && ) [Expression](#grammar-summary-Expression)\
    \| ( & \| && ) mut [Expression](#grammar-summary-Expression)\
    \| ( & \| && ) raw const [Expression](#grammar-summary-Expression)\
    \| ( & \| && ) raw mut [Expression](#grammar-summary-Expression)

[DereferenceExpression](#railroad-summary-DereferenceExpression) → \* [Expression](#grammar-summary-Expression)

[TryPropagationExpression](#railroad-summary-TryPropagationExpression) → [Expression](#grammar-summary-Expression) ?

[NegationExpression](#railroad-summary-NegationExpression) →\
      - [Expression](#grammar-summary-Expression)\
    \| ! [Expression](#grammar-summary-Expression)

[ArithmeticOrLogicalExpression](#railroad-summary-ArithmeticOrLogicalExpression) →\
      [Expression](#grammar-summary-Expression) + [Expression](#grammar-summary-Expression)\
    \| [Expression](#grammar-summary-Expression) - [Expression](#grammar-summary-Expression)\
    \| [Expression](#grammar-summary-Expression) \* [Expression](#grammar-summary-Expression)\
    \| [Expression](#grammar-summary-Expression) / [Expression](#grammar-summary-Expression)\
    \| [Expression](#grammar-summary-Expression) % [Expression](#grammar-summary-Expression)\
    \| [Expression](#grammar-summary-Expression) & [Expression](#grammar-summary-Expression)\
    \| [Expression](#grammar-summary-Expression) \| [Expression](#grammar-summary-Expression)\
    \| [Expression](#grammar-summary-Expression) ^ [Expression](#grammar-summary-Expression)\
    \| [Expression](#grammar-summary-Expression) \<\< [Expression](#grammar-summary-Expression)\
    \| [Expression](#grammar-summary-Expression) \>\> [Expression](#grammar-summary-Expression)

[ComparisonExpression](#railroad-summary-ComparisonExpression) →\
      [Expression](#grammar-summary-Expression) == [Expression](#grammar-summary-Expression)\
    \| [Expression](#grammar-summary-Expression) != [Expression](#grammar-summary-Expression)\
    \| [Expression](#grammar-summary-Expression) \> [Expression](#grammar-summary-Expression)\
    \| [Expression](#grammar-summary-Expression) \< [Expression](#grammar-summary-Expression)\
    \| [Expression](#grammar-summary-Expression) \>= [Expression](#grammar-summary-Expression)\
    \| [Expression](#grammar-summary-Expression) \<= [Expression](#grammar-summary-Expression)

[LazyBooleanExpression](#railroad-summary-LazyBooleanExpression) →\
      [Expression](#grammar-summary-Expression) \|\| [Expression](#grammar-summary-Expression)\
    \| [Expression](#grammar-summary-Expression) && [Expression](#grammar-summary-Expression)

[TypeCastExpression](#railroad-summary-TypeCastExpression) → [Expression](#grammar-summary-Expression) as [TypeNoBounds](#grammar-summary-TypeNoBounds)

[AssignmentExpression](#railroad-summary-AssignmentExpression) → [Expression](#grammar-summary-Expression) = [Expression](#grammar-summary-Expression)

[CompoundAssignmentExpression](#railroad-summary-CompoundAssignmentExpression) →\
      [Expression](#grammar-summary-Expression) += [Expression](#grammar-summary-Expression)\
    \| [Expression](#grammar-summary-Expression) -= [Expression](#grammar-summary-Expression)\
    \| [Expression](#grammar-summary-Expression) \*= [Expression](#grammar-summary-Expression)\
    \| [Expression](#grammar-summary-Expression) /= [Expression](#grammar-summary-Expression)\
    \| [Expression](#grammar-summary-Expression) %= [Expression](#grammar-summary-Expression)\
    \| [Expression](#grammar-summary-Expression) &= [Expression](#grammar-summary-Expression)\
    \| [Expression](#grammar-summary-Expression) \|= [Expression](#grammar-summary-Expression)\
    \| [Expression](#grammar-summary-Expression) ^= [Expression](#grammar-summary-Expression)\
    \| [Expression](#grammar-summary-Expression) \<\<= [Expression](#grammar-summary-Expression)\
    \| [Expression](#grammar-summary-Expression) \>\>= [Expression](#grammar-summary-Expression)

[PathExpression](#railroad-summary-PathExpression) →\
      [PathInExpression](#grammar-summary-PathInExpression)\
    \| [QualifiedPathInExpression](#grammar-summary-QualifiedPathInExpression)

[RangeExpression](#railroad-summary-RangeExpression) →\
      [RangeExpr](#grammar-summary-RangeExpr)\
    \| [RangeFromExpr](#grammar-summary-RangeFromExpr)\
    \| [RangeToExpr](#grammar-summary-RangeToExpr)\
    \| [RangeFullExpr](#grammar-summary-RangeFullExpr)\
    \| [RangeInclusiveExpr](#grammar-summary-RangeInclusiveExpr)\
    \| [RangeToInclusiveExpr](#grammar-summary-RangeToInclusiveExpr)

[RangeExpr](#railroad-summary-RangeExpr) → [Expression](#grammar-summary-Expression) .. [Expression](#grammar-summary-Expression)

[RangeFromExpr](#railroad-summary-RangeFromExpr) → [Expression](#grammar-summary-Expression) ..

[RangeToExpr](#railroad-summary-RangeToExpr) → .. [Expression](#grammar-summary-Expression)

[RangeFullExpr](#railroad-summary-RangeFullExpr) → ..

[RangeInclusiveExpr](#railroad-summary-RangeInclusiveExpr) → [Expression](#grammar-summary-Expression) ..= [Expression](#grammar-summary-Expression)

[RangeToInclusiveExpr](#railroad-summary-RangeToInclusiveExpr) → ..= [Expression](#grammar-summary-Expression)

[ReturnExpression](#railroad-summary-ReturnExpression) → return [Expression](#grammar-summary-Expression)?

[StructExpression](#railroad-summary-StructExpression) →\
    [PathInExpression](#grammar-summary-PathInExpression) { ( [StructExprFields](#grammar-summary-StructExprFields) \| [StructBase](#grammar-summary-StructBase) )? }

[StructExprFields](#railroad-summary-StructExprFields) →\
    [StructExprField](#grammar-summary-StructExprField) ( , [StructExprField](#grammar-summary-StructExprField) )\* ( , [StructBase](#grammar-summary-StructBase) \| ,? )

[StructExprField](#railroad-summary-StructExprField) →\
    [OuterAttribute](#grammar-summary-OuterAttribute)\*\
    (\
        [IDENTIFIER](#grammar-summary-IDENTIFIER)\
      \| ( [IDENTIFIER](#grammar-summary-IDENTIFIER) \| [TUPLE_INDEX](#grammar-summary-TUPLE_INDEX) ) : [Expression](#grammar-summary-Expression)\
    )

[StructBase](#railroad-summary-StructBase) → .. [Expression](#grammar-summary-Expression)

[TupleExpression](#railroad-summary-TupleExpression) → ( [TupleElements](#grammar-summary-TupleElements)? )

[TupleElements](#railroad-summary-TupleElements) → ( [Expression](#grammar-summary-Expression) , )+ [Expression](#grammar-summary-Expression)?

[TupleIndexingExpression](#railroad-summary-TupleIndexingExpression) → [Expression](#grammar-summary-Expression) . [TUPLE_INDEX](#grammar-summary-TUPLE_INDEX)

[UnderscoreExpression](#railroad-summary-UnderscoreExpression) → \_

[Expression](#railroad-summary-Expression) →\
      [ExpressionWithoutBlock](#grammar-summary-ExpressionWithoutBlock)\
    \| [ExpressionWithBlock](#grammar-summary-ExpressionWithBlock)

[ExpressionWithoutBlock](#railroad-summary-ExpressionWithoutBlock) →\
    [OuterAttribute](#grammar-summary-OuterAttribute)\* [ExpressionWithoutBlockNoAttrs](#grammar-summary-ExpressionWithoutBlockNoAttrs)

[ExpressionWithoutBlockNoAttrs](#railroad-summary-ExpressionWithoutBlockNoAttrs) →\
      [LiteralExpression](#grammar-summary-LiteralExpression)\
    \| [PathExpression](#grammar-summary-PathExpression)\
    \| [OperatorExpression](#grammar-summary-OperatorExpression)\
    \| [GroupedExpression](#grammar-summary-GroupedExpression)\
    \| [ArrayExpression](#grammar-summary-ArrayExpression)\
    \| [AwaitExpression](#grammar-summary-AwaitExpression)\
    \| [IndexExpression](#grammar-summary-IndexExpression)\
    \| [TupleExpression](#grammar-summary-TupleExpression)\
    \| [TupleIndexingExpression](#grammar-summary-TupleIndexingExpression)\
    \| [StructExpression](#grammar-summary-StructExpression)\
    \| [CallExpression](#grammar-summary-CallExpression)\
    \| [MethodCallExpression](#grammar-summary-MethodCallExpression)\
    \| [FieldExpression](#grammar-summary-FieldExpression)\
    \| [ClosureExpression](#grammar-summary-ClosureExpression)\
    \| [AsyncBlockExpression](#grammar-summary-AsyncBlockExpression)\
    \| [ContinueExpression](#grammar-summary-ContinueExpression)\
    \| [BreakExpression](#grammar-summary-BreakExpression)\
    \| [RangeExpression](#grammar-summary-RangeExpression)\
    \| [ReturnExpression](#grammar-summary-ReturnExpression)\
    \| [UnderscoreExpression](#grammar-summary-UnderscoreExpression)\
    \| [MacroInvocation](#grammar-summary-MacroInvocation)

[ExpressionWithBlock](#railroad-summary-ExpressionWithBlock) →\
    [OuterAttribute](#grammar-summary-OuterAttribute)\* [ExpressionWithBlockNoAttrs](#grammar-summary-ExpressionWithBlockNoAttrs)

[ExpressionWithBlockNoAttrs](#railroad-summary-ExpressionWithBlockNoAttrs) →\
      [BlockExpression](#grammar-summary-BlockExpression)\
    \| [ConstBlockExpression](#grammar-summary-ConstBlockExpression)\
    \| [UnsafeBlockExpression](#grammar-summary-UnsafeBlockExpression)\
    \| [LoopExpression](#grammar-summary-LoopExpression)\
    \| [IfExpression](#grammar-summary-IfExpression)\
    \| [MatchExpression](#grammar-summary-MatchExpression)

Show Railroad

![](../assets/inline/grammar-168.svg) ![](../assets/inline/grammar-169.svg) ![](../assets/inline/grammar-170.svg) ![](../assets/inline/grammar-171.svg) ![](../assets/inline/grammar-172.svg) ![](../assets/inline/grammar-173.svg) ![](../assets/inline/grammar-174.svg) ![](../assets/inline/grammar-175.svg) ![](../assets/inline/grammar-176.svg) ![](../assets/inline/grammar-177.svg) ![](../assets/inline/grammar-178.svg) ![](../assets/inline/grammar-179.svg) ![](../assets/inline/grammar-180.svg) ![](../assets/inline/grammar-181.svg) ![](../assets/inline/grammar-182.svg) ![](../assets/inline/grammar-183.svg) ![](../assets/inline/grammar-184.svg) ![](../assets/inline/grammar-185.svg) ![](../assets/inline/grammar-186.svg) ![](../assets/inline/grammar-187.svg) ![](../assets/inline/grammar-188.svg) ![](../assets/inline/grammar-189.svg) ![](../assets/inline/grammar-190.svg) ![](../assets/inline/grammar-191.svg) ![](../assets/inline/grammar-192.svg) ![](../assets/inline/grammar-193.svg) ![](../assets/inline/grammar-194.svg) ![](../assets/inline/grammar-195.svg) ![](../assets/inline/grammar-196.svg) ![](../assets/inline/grammar-197.svg) ![](../assets/inline/grammar-198.svg) ![](../assets/inline/grammar-199.svg) ![](../assets/inline/grammar-200.svg) ![](../assets/inline/grammar-201.svg) ![](../assets/inline/grammar-202.svg) ![](../assets/inline/grammar-203.svg) ![](../assets/inline/grammar-204.svg) ![](../assets/inline/grammar-205.svg) ![](../assets/inline/grammar-206.svg) ![](../assets/inline/grammar-207.svg) ![](../assets/inline/grammar-208.svg) ![](../assets/inline/grammar-209.svg) ![](../assets/inline/grammar-210.svg) ![](../assets/inline/grammar-211.svg) ![](../assets/inline/grammar-212.svg) ![](../assets/inline/grammar-213.svg) ![](../assets/inline/grammar-214.svg) ![](../assets/inline/grammar-215.svg) ![](../assets/inline/grammar-216.svg) ![](../assets/inline/grammar-217.svg) ![](../assets/inline/grammar-218.svg) ![](../assets/inline/grammar-219.svg) ![](../assets/inline/grammar-220.svg) ![](../assets/inline/grammar-221.svg) ![](../assets/inline/grammar-222.svg) ![](../assets/inline/grammar-223.svg) ![](../assets/inline/grammar-224.svg) ![](../assets/inline/grammar-225.svg) ![](../assets/inline/grammar-226.svg) ![](../assets/inline/grammar-227.svg) ![](../assets/inline/grammar-228.svg) ![](../assets/inline/grammar-229.svg) ![](../assets/inline/grammar-230.svg) ![](../assets/inline/grammar-231.svg) ![](../assets/inline/grammar-232.svg) ![](../assets/inline/grammar-233.svg) ![](../assets/inline/grammar-234.svg) ![](../assets/inline/grammar-235.svg) ![](../assets/inline/grammar-236.svg) ![](../assets/inline/grammar-237.svg) ![](../assets/inline/grammar-238.svg) ![](../assets/inline/grammar-239.svg) ![](../assets/inline/grammar-240.svg) ![](../assets/inline/grammar-241.svg) ![](../assets/inline/grammar-242.svg)

## Assembly summary

**Syntax**\
[AsmArgs](#railroad-summary-AsmArgs) → [AsmAttrFormatString](#grammar-summary-AsmAttrFormatString) ( , [AsmAttrFormatString](#grammar-summary-AsmAttrFormatString) )\* ( , [AsmAttrOperand](#grammar-summary-AsmAttrOperand) )\* ,?

[FormatString](#railroad-summary-FormatString) → [STRING_LITERAL](#grammar-summary-STRING_LITERAL) \| [RAW_STRING_LITERAL](#grammar-summary-RAW_STRING_LITERAL) \| [MacroInvocation](#grammar-summary-MacroInvocation)

[AsmAttrFormatString](#railroad-summary-AsmAttrFormatString) → ( [OuterAttribute](#grammar-summary-OuterAttribute) )\* [FormatString](#grammar-summary-FormatString)

[AsmOperand](#railroad-summary-AsmOperand) →\
      [ClobberAbi](#grammar-summary-ClobberAbi)\
    \| [AsmOptions](#grammar-summary-AsmOptions)\
    \| [RegOperand](#grammar-summary-RegOperand)

[AsmAttrOperand](#railroad-summary-AsmAttrOperand) → ( [OuterAttribute](#grammar-summary-OuterAttribute) )\* [AsmOperand](#grammar-summary-AsmOperand)

[ClobberAbi](#railroad-summary-ClobberAbi) → clobber_abi ( [Abi](#grammar-summary-Abi) ( , [Abi](#grammar-summary-Abi) )\* ,? )

[AsmOptions](#railroad-summary-AsmOptions) →\
    options ( ( [AsmOption](#grammar-summary-AsmOption) ( , [AsmOption](#grammar-summary-AsmOption) )\* ,? )? )

[AsmOption](#railroad-summary-AsmOption) →\
      pure\
    \| nomem\
    \| readonly\
    \| preserves_flags\
    \| noreturn\
    \| nostack\
    \| att_syntax\
    \| raw

[RegOperand](#railroad-summary-RegOperand) → ( [ParamName](#grammar-summary-ParamName) = )?\
    (\
          [DirSpec](#grammar-summary-DirSpec) ( [RegSpec](#grammar-summary-RegSpec) ) [Expression](#grammar-summary-Expression)\
        \| [DualDirSpec](#grammar-summary-DualDirSpec) ( [RegSpec](#grammar-summary-RegSpec) ) [DualDirSpecExpression](#grammar-summary-DualDirSpecExpression)\
        \| sym [PathExpression](#grammar-summary-PathExpression)\
        \| const [Expression](#grammar-summary-Expression)\
        \| label { [Statements](#grammar-summary-Statements)? }\
    )

[ParamName](#railroad-summary-ParamName) → [IDENTIFIER_OR_KEYWORD](#grammar-summary-IDENTIFIER_OR_KEYWORD) \| [RAW_IDENTIFIER](#grammar-summary-RAW_IDENTIFIER)

[DualDirSpecExpression](#railroad-summary-DualDirSpecExpression) →\
      [Expression](#grammar-summary-Expression)\
    \| [Expression](#grammar-summary-Expression) =\> [Expression](#grammar-summary-Expression)

[RegSpec](#railroad-summary-RegSpec) → [RegisterClass](#grammar-summary-RegisterClass) \| [ExplicitRegister](#grammar-summary-ExplicitRegister)

[RegisterClass](#railroad-summary-RegisterClass) → [IDENTIFIER_OR_KEYWORD](#grammar-summary-IDENTIFIER_OR_KEYWORD)

[ExplicitRegister](#railroad-summary-ExplicitRegister) → [STRING_LITERAL](#grammar-summary-STRING_LITERAL)

[DirSpec](#railroad-summary-DirSpec) →\
      in\
    \| out\
    \| lateout

[DualDirSpec](#railroad-summary-DualDirSpec) →\
      inout\
    \| inlateout

Show Railroad

![](../assets/inline/grammar-243.svg) ![](../assets/inline/grammar-244.svg) ![](../assets/inline/grammar-245.svg) ![](../assets/inline/grammar-246.svg) ![](../assets/inline/grammar-247.svg) ![](../assets/inline/grammar-248.svg) ![](../assets/inline/grammar-249.svg) ![](../assets/inline/grammar-250.svg) ![](../assets/inline/grammar-251.svg) ![](../assets/inline/grammar-252.svg) ![](../assets/inline/grammar-253.svg) ![](../assets/inline/grammar-254.svg) ![](../assets/inline/grammar-255.svg) ![](../assets/inline/grammar-256.svg) ![](../assets/inline/grammar-257.svg) ![](../assets/inline/grammar-258.svg)

## Macros summary

**Syntax**\
[MacroRulesDefinition](#railroad-summary-MacroRulesDefinition) →\
    macro_rules ! [IDENTIFIER](#grammar-summary-IDENTIFIER) [MacroRulesDef](#grammar-summary-MacroRulesDef)

[MacroRulesDef](#railroad-summary-MacroRulesDef) →\
      ( [MacroRules](#grammar-summary-MacroRules) ) ;\
    \| \[ [MacroRules](#grammar-summary-MacroRules) \] ;\
    \| { [MacroRules](#grammar-summary-MacroRules) }

[MacroRules](#railroad-summary-MacroRules) →\
    [MacroRule](#grammar-summary-MacroRule) ( ; [MacroRule](#grammar-summary-MacroRule) )\* ;?

[MacroRule](#railroad-summary-MacroRule) →\
    [MacroMatcher](#grammar-summary-MacroMatcher) =\> [MacroTranscriber](#grammar-summary-MacroTranscriber)

[MacroMatcher](#railroad-summary-MacroMatcher) →\
      ( [MacroMatch](#grammar-summary-MacroMatch)\* )\
    \| \[ [MacroMatch](#grammar-summary-MacroMatch)\* \]\
    \| { [MacroMatch](#grammar-summary-MacroMatch)\* }

[MacroMatch](#railroad-summary-MacroMatch) →\
      [Token](#grammar-summary-Token)except `$` and [delimiters](tokens.md#r-lex.token.delim)\
    \| [MacroMatcher](#grammar-summary-MacroMatcher)\
    \| \$ ( [IDENTIFIER_OR_KEYWORD](#grammar-summary-IDENTIFIER_OR_KEYWORD)except `crate` \| [RAW_IDENTIFIER](#grammar-summary-RAW_IDENTIFIER) ) : [MacroFragSpec](#grammar-summary-MacroFragSpec)\
    \| \$ ( [MacroMatch](#grammar-summary-MacroMatch)+ ) [MacroRepSep](#grammar-summary-MacroRepSep)? [MacroRepOp](#grammar-summary-MacroRepOp)

[MacroFragSpec](#railroad-summary-MacroFragSpec) →\
      block \| expr \| expr_2021 \| ident \| item \| lifetime \| literal\
    \| meta \| pat \| pat_param \| path \| stmt \| tt \| ty \| vis

[MacroRepSep](#railroad-summary-MacroRepSep) → [Token](#grammar-summary-Token)except [delimiters](tokens.md#r-lex.token.delim) and [MacroRepOp](#grammar-summary-MacroRepOp)

[MacroRepOp](#railroad-summary-MacroRepOp) → \* \| + \| ?

[MacroTranscriber](#railroad-summary-MacroTranscriber) → [DelimTokenTree](#grammar-summary-DelimTokenTree)

[MacroInvocation](#railroad-summary-MacroInvocation) →\
    [SimplePath](#grammar-summary-SimplePath) ! [DelimTokenTree](#grammar-summary-DelimTokenTree)

[DelimTokenTree](#railroad-summary-DelimTokenTree) →\
      ( [TokenTree](#grammar-summary-TokenTree)\* )\
    \| \[ [TokenTree](#grammar-summary-TokenTree)\* \]\
    \| { [TokenTree](#grammar-summary-TokenTree)\* }

[TokenTree](#railroad-summary-TokenTree) →\
    [Token](#grammar-summary-Token)except [delimiters](tokens.md#r-lex.token.delim) \| [DelimTokenTree](#grammar-summary-DelimTokenTree)

[MacroInvocationSemi](#railroad-summary-MacroInvocationSemi) →\
      [SimplePath](#grammar-summary-SimplePath) ! ( [TokenTree](#grammar-summary-TokenTree)\* ) ;\
    \| [SimplePath](#grammar-summary-SimplePath) ! \[ [TokenTree](#grammar-summary-TokenTree)\* \] ;\
    \| [SimplePath](#grammar-summary-SimplePath) ! { [TokenTree](#grammar-summary-TokenTree)\* }

Show Railroad

![](../assets/inline/grammar-259.svg) ![](../assets/inline/grammar-260.svg) ![](../assets/inline/grammar-261.svg) ![](../assets/inline/grammar-262.svg) ![](../assets/inline/grammar-263.svg) ![](../assets/inline/grammar-264.svg) ![](../assets/inline/grammar-265.svg) ![](../assets/inline/grammar-266.svg) ![](../assets/inline/grammar-267.svg) ![](../assets/inline/grammar-268.svg) ![](../assets/inline/grammar-269.svg) ![](../assets/inline/grammar-270.svg) ![](../assets/inline/grammar-271.svg) ![](../assets/inline/grammar-272.svg)

## Paths summary

**Syntax**\
[SimplePath](#railroad-summary-SimplePath) →\
    ::? [SimplePathSegment](#grammar-summary-SimplePathSegment) ( :: [SimplePathSegment](#grammar-summary-SimplePathSegment) )\*

[SimplePathSegment](#railroad-summary-SimplePathSegment) →\
    [IDENTIFIER](#grammar-summary-IDENTIFIER) \| super \| self \| crate \| \$crate

[PathInExpression](#railroad-summary-PathInExpression) →\
    ::? [PathExprSegment](#grammar-summary-PathExprSegment) ( :: [PathExprSegment](#grammar-summary-PathExprSegment) )\*

[PathExprSegment](#railroad-summary-PathExprSegment) →\
    [PathIdentSegment](#grammar-summary-PathIdentSegment) ( :: [GenericArgs](#grammar-summary-GenericArgs) )?

[PathIdentSegment](#railroad-summary-PathIdentSegment) →\
    [IDENTIFIER](#grammar-summary-IDENTIFIER) \| super \| self \| Self \| crate \| \$crate

[GenericArgs](#railroad-summary-GenericArgs) →\
      \< [GenericArgList](#grammar-summary-GenericArgList)? \>\
    \| ( [TypeList](#grammar-summary-TypeList)? ) ( -\> [TypeNoBounds](#grammar-summary-TypeNoBounds) )?

[GenericArgList](#railroad-summary-GenericArgList) →\
    ( [GenericArg](#grammar-summary-GenericArg) , )\* [GenericArg](#grammar-summary-GenericArg) ,?

[TypeList](#railroad-summary-TypeList) →\
    ( [Type](#grammar-summary-Type) , )\* [Type](#grammar-summary-Type) ,?

[GenericArg](#railroad-summary-GenericArg) →\
    [Lifetime](#grammar-summary-Lifetime) \| [Type](#grammar-summary-Type) \| [GenericArgsConst](#grammar-summary-GenericArgsConst) \| [GenericArgsBinding](#grammar-summary-GenericArgsBinding) \| [GenericArgsBounds](#grammar-summary-GenericArgsBounds)

[GenericArgsConst](#railroad-summary-GenericArgsConst) →\
      [BlockExpression](#grammar-summary-BlockExpression)\
    \| [LiteralExpression](#grammar-summary-LiteralExpression)\
    \| - [LiteralExpression](#grammar-summary-LiteralExpression)\
    \| [SimplePathSegment](#grammar-summary-SimplePathSegment)

[GenericArgsBinding](#railroad-summary-GenericArgsBinding) →\
    [TypePathSegment](#grammar-summary-TypePathSegment) = [Type](#grammar-summary-Type)

[GenericArgsBounds](#railroad-summary-GenericArgsBounds) →\
    [TypePathSegment](#grammar-summary-TypePathSegment) : [Bounds](#grammar-summary-Bounds)?

[QualifiedPathInExpression](#railroad-summary-QualifiedPathInExpression) → [QualifiedPathType](#grammar-summary-QualifiedPathType) ( :: [PathExprSegment](#grammar-summary-PathExprSegment) )+

[QualifiedPathType](#railroad-summary-QualifiedPathType) → \< [Type](#grammar-summary-Type) ( as [TypePath](#grammar-summary-TypePath) )? \>

[QualifiedPathInType](#railroad-summary-QualifiedPathInType) → [QualifiedPathType](#grammar-summary-QualifiedPathType) ( :: [TypePathSegment](#grammar-summary-TypePathSegment) )+

[TypePath](#railroad-summary-TypePath) → ::? [TypePathSegment](#grammar-summary-TypePathSegment) ( :: [TypePathSegment](#grammar-summary-TypePathSegment) )\*

[TypePathSegment](#railroad-summary-TypePathSegment) → [PathIdentSegment](#grammar-summary-PathIdentSegment) ( ::? [GenericArgs](#grammar-summary-GenericArgs) )?

Show Railroad

![](../assets/inline/grammar-273.svg) ![](../assets/inline/grammar-274.svg) ![](../assets/inline/grammar-275.svg) ![](../assets/inline/grammar-276.svg) ![](../assets/inline/grammar-277.svg) ![](../assets/inline/grammar-278.svg) ![](../assets/inline/grammar-279.svg) ![](../assets/inline/grammar-280.svg) ![](../assets/inline/grammar-281.svg) ![](../assets/inline/grammar-282.svg) ![](../assets/inline/grammar-283.svg) ![](../assets/inline/grammar-284.svg) ![](../assets/inline/grammar-285.svg) ![](../assets/inline/grammar-286.svg) ![](../assets/inline/grammar-287.svg) ![](../assets/inline/grammar-288.svg) ![](../assets/inline/grammar-289.svg)

## Patterns summary

**Syntax**\
[Pattern](#railroad-summary-Pattern) → \|? [PatternNoTopAlt](#grammar-summary-PatternNoTopAlt) ( \| [PatternNoTopAlt](#grammar-summary-PatternNoTopAlt) )\*

[PatternNoTopAlt](#railroad-summary-PatternNoTopAlt) →\
      [PatternWithoutModernRange](#grammar-summary-PatternWithoutModernRange)\
    \| [ModernRangePattern](#grammar-summary-ModernRangePattern)

[PatternWithoutModernRange](#railroad-summary-PatternWithoutModernRange) →\
      [LiteralPattern](#grammar-summary-LiteralPattern)\
    \| [IdentifierPattern](#grammar-summary-IdentifierPattern)\
    \| [WildcardPattern](#grammar-summary-WildcardPattern)\
    \| [RestPattern](#grammar-summary-RestPattern)\
    \| [ReferencePattern](#grammar-summary-ReferencePattern)\
    \| [StructPattern](#grammar-summary-StructPattern)\
    \| [TupleStructPattern](#grammar-summary-TupleStructPattern)\
    \| [TuplePattern](#grammar-summary-TuplePattern)\
    \| [GroupedPattern](#grammar-summary-GroupedPattern)\
    \| [SlicePattern](#grammar-summary-SlicePattern)\
    \| [PathPattern](#grammar-summary-PathPattern)\
    \| [MacroInvocation](#grammar-summary-MacroInvocation)\
    \| [ObsoleteRangePattern](#grammar-summary-ObsoleteRangePattern)

[LiteralPattern](#railroad-summary-LiteralPattern) → -? [LiteralExpression](#grammar-summary-LiteralExpression)

[IdentifierPattern](#railroad-summary-IdentifierPattern) → ref? mut? [IDENTIFIER](#grammar-summary-IDENTIFIER) ( @ [PatternNoTopAlt](#grammar-summary-PatternNoTopAlt) )?

[WildcardPattern](#railroad-summary-WildcardPattern) → \_

[RestPattern](#railroad-summary-RestPattern) → ..

[ModernRangePattern](#railroad-summary-ModernRangePattern) →\
      [RangeExclusivePattern](#grammar-summary-RangeExclusivePattern)\
    \| [RangeInclusivePattern](#grammar-summary-RangeInclusivePattern)\
    \| [RangeFromPattern](#grammar-summary-RangeFromPattern)\
    \| [RangeToExclusivePattern](#grammar-summary-RangeToExclusivePattern)\
    \| [RangeToInclusivePattern](#grammar-summary-RangeToInclusivePattern)

[RangeExclusivePattern](#railroad-summary-RangeExclusivePattern) →\
      [RangePatternBound](#grammar-summary-RangePatternBound) .. [RangePatternBound](#grammar-summary-RangePatternBound)

[RangeInclusivePattern](#railroad-summary-RangeInclusivePattern) →\
      [RangePatternBound](#grammar-summary-RangePatternBound) ..= [RangePatternBound](#grammar-summary-RangePatternBound)

[RangeFromPattern](#railroad-summary-RangeFromPattern) →\
      [RangePatternBound](#grammar-summary-RangePatternBound) ..

[RangeToExclusivePattern](#railroad-summary-RangeToExclusivePattern) →\
      .. [RangePatternBound](#grammar-summary-RangePatternBound)

[RangeToInclusivePattern](#railroad-summary-RangeToInclusivePattern) →\
      ..= [RangePatternBound](#grammar-summary-RangePatternBound)

[ObsoleteRangePattern](#railroad-summary-ObsoleteRangePattern) →\
    [RangePatternBound](#grammar-summary-RangePatternBound) ... [RangePatternBound](#grammar-summary-RangePatternBound)

[RangePatternBound](#railroad-summary-RangePatternBound) →\
      [LiteralPattern](#grammar-summary-LiteralPattern)\
    \| [PathExpression](#grammar-summary-PathExpression)

[ReferencePattern](#railroad-summary-ReferencePattern) → ( & \| && ) mut? [PatternWithoutModernRange](#grammar-summary-PatternWithoutModernRange)

[StructPattern](#railroad-summary-StructPattern) →\
    [PathInExpression](#grammar-summary-PathInExpression) {\
        [StructPatternElements](#grammar-summary-StructPatternElements)?\
    }

[StructPatternElements](#railroad-summary-StructPatternElements) →\
      [StructPatternFields](#grammar-summary-StructPatternFields) ( , \| , [StructPatternEtCetera](#grammar-summary-StructPatternEtCetera) )?\
    \| [StructPatternEtCetera](#grammar-summary-StructPatternEtCetera)

[StructPatternFields](#railroad-summary-StructPatternFields) →\
    [StructPatternField](#grammar-summary-StructPatternField) ( , [StructPatternField](#grammar-summary-StructPatternField) )\*

[StructPatternField](#railroad-summary-StructPatternField) →\
    [OuterAttribute](#grammar-summary-OuterAttribute)\*\
    (\
        [TUPLE_INDEX](#grammar-summary-TUPLE_INDEX) : [Pattern](#grammar-summary-Pattern)\
      \| [IDENTIFIER](#grammar-summary-IDENTIFIER) : [Pattern](#grammar-summary-Pattern)\
      \| ref? mut? [IDENTIFIER](#grammar-summary-IDENTIFIER)\
    )

[StructPatternEtCetera](#railroad-summary-StructPatternEtCetera) → ..

[TupleStructPattern](#railroad-summary-TupleStructPattern) → [PathInExpression](#grammar-summary-PathInExpression) ( [TupleStructItems](#grammar-summary-TupleStructItems)? )

[TupleStructItems](#railroad-summary-TupleStructItems) → [Pattern](#grammar-summary-Pattern) ( , [Pattern](#grammar-summary-Pattern) )\* ,?

[TuplePattern](#railroad-summary-TuplePattern) → ( [TuplePatternItems](#grammar-summary-TuplePatternItems)? )

[TuplePatternItems](#railroad-summary-TuplePatternItems) →\
      [Pattern](#grammar-summary-Pattern) ,\
    \| [RestPattern](#grammar-summary-RestPattern)\
    \| [Pattern](#grammar-summary-Pattern) ( , [Pattern](#grammar-summary-Pattern) )+ ,?

[GroupedPattern](#railroad-summary-GroupedPattern) → ( [Pattern](#grammar-summary-Pattern) )

[SlicePattern](#railroad-summary-SlicePattern) → \[ [SlicePatternItems](#grammar-summary-SlicePatternItems)? \]

[SlicePatternItems](#railroad-summary-SlicePatternItems) → [Pattern](#grammar-summary-Pattern) ( , [Pattern](#grammar-summary-Pattern) )\* ,?

[PathPattern](#railroad-summary-PathPattern) → [PathExpression](#grammar-summary-PathExpression)

Show Railroad

![](../assets/inline/grammar-290.svg) ![](../assets/inline/grammar-291.svg) ![](../assets/inline/grammar-292.svg) ![](../assets/inline/grammar-293.svg) ![](../assets/inline/grammar-294.svg) ![](../assets/inline/grammar-295.svg) ![](../assets/inline/grammar-296.svg) ![](../assets/inline/grammar-297.svg) ![](../assets/inline/grammar-298.svg) ![](../assets/inline/grammar-299.svg) ![](../assets/inline/grammar-300.svg) ![](../assets/inline/grammar-301.svg) ![](../assets/inline/grammar-302.svg) ![](../assets/inline/grammar-303.svg) ![](../assets/inline/grammar-304.svg) ![](../assets/inline/grammar-305.svg) ![](../assets/inline/grammar-306.svg) ![](../assets/inline/grammar-307.svg) ![](../assets/inline/grammar-308.svg) ![](../assets/inline/grammar-309.svg) ![](../assets/inline/grammar-310.svg) ![](../assets/inline/grammar-311.svg) ![](../assets/inline/grammar-312.svg) ![](../assets/inline/grammar-313.svg) ![](../assets/inline/grammar-314.svg) ![](../assets/inline/grammar-315.svg) ![](../assets/inline/grammar-316.svg) ![](../assets/inline/grammar-317.svg) ![](../assets/inline/grammar-318.svg)

## Statements summary

**Syntax**\
[Statement](#railroad-summary-Statement) →\
      ;\
    \| [Item](#grammar-summary-Item)\
    \| [LetStatement](#grammar-summary-LetStatement)\
    \| [ExpressionStatement](#grammar-summary-ExpressionStatement)\
    \| [OuterAttribute](#grammar-summary-OuterAttribute)\* [MacroInvocationSemi](#grammar-summary-MacroInvocationSemi)

[LetStatement](#railroad-summary-LetStatement) →\
    [OuterAttribute](#grammar-summary-OuterAttribute)\* let [PatternNoTopAlt](#grammar-summary-PatternNoTopAlt) ( : [Type](#grammar-summary-Type) )?\
    (\
          = [Expression](#grammar-summary-Expression)\
        \| = [Expression](#grammar-summary-Expression)except [LazyBooleanExpression](#grammar-summary-LazyBooleanExpression) or end with a `}`\
              else [BlockExpressionNoInnerAttributes](#grammar-summary-BlockExpressionNoInnerAttributes)\
    )? ;

[ExpressionStatement](#railroad-summary-ExpressionStatement) →\
      [ExpressionWithoutBlock](#grammar-summary-ExpressionWithoutBlock) ;\
    \| [ExpressionWithBlock](#grammar-summary-ExpressionWithBlock) ;?

Show Railroad

![](../assets/inline/grammar-319.svg) ![](../assets/inline/grammar-320.svg) ![](../assets/inline/grammar-321.svg)

## Miscellaneous summary

**Syntax**\
[Bounds](#railroad-summary-Bounds) → [Bound](#grammar-summary-Bound) ( + [Bound](#grammar-summary-Bound) )\* +?

[Bound](#railroad-summary-Bound) → [Lifetime](#grammar-summary-Lifetime) \| [TraitBound](#grammar-summary-TraitBound) \| [UseBound](#grammar-summary-UseBound)

[TraitBound](#railroad-summary-TraitBound) →\
      ( ? \| [ForLifetimes](#grammar-summary-ForLifetimes) )? [TypePath](#grammar-summary-TypePath)\
    \| ( ( ? \| [ForLifetimes](#grammar-summary-ForLifetimes) )? [TypePath](#grammar-summary-TypePath) )

[LifetimeBounds](#railroad-summary-LifetimeBounds) → ( [Lifetime](#grammar-summary-Lifetime) + )\* [Lifetime](#grammar-summary-Lifetime)?

[Lifetime](#railroad-summary-Lifetime) →\
      [LIFETIME_OR_LABEL](#grammar-summary-LIFETIME_OR_LABEL)\
    \| 'static\
    \| '\_

[UseBound](#railroad-summary-UseBound) → use [UseBoundGenericArgs](#grammar-summary-UseBoundGenericArgs)

[UseBoundGenericArgs](#railroad-summary-UseBoundGenericArgs) →\
      \< \>\
    \| \< ( [UseBoundGenericArg](#grammar-summary-UseBoundGenericArg) , )\* [UseBoundGenericArg](#grammar-summary-UseBoundGenericArg) ,? \>

[UseBoundGenericArg](#railroad-summary-UseBoundGenericArg) →\
      [Lifetime](#grammar-summary-Lifetime)\
    \| [IDENTIFIER](#grammar-summary-IDENTIFIER)\
    \| Self

[ForLifetimes](#railroad-summary-ForLifetimes) → for [GenericParams](#grammar-summary-GenericParams)

Show Railroad

![](../assets/inline/grammar-322.svg) ![](../assets/inline/grammar-323.svg) ![](../assets/inline/grammar-324.svg) ![](../assets/inline/grammar-325.svg) ![](../assets/inline/grammar-326.svg) ![](../assets/inline/grammar-327.svg) ![](../assets/inline/grammar-328.svg) ![](../assets/inline/grammar-329.svg) ![](../assets/inline/grammar-330.svg)

## Types summary

**Syntax**\
[ArrayType](#railroad-summary-ArrayType) → \[ [Type](#grammar-summary-Type) ; [Expression](#grammar-summary-Expression) \]

[BareFunctionType](#railroad-summary-BareFunctionType) →\
    [ForLifetimes](#grammar-summary-ForLifetimes)? [FunctionTypeQualifiers](#grammar-summary-FunctionTypeQualifiers) fn\
       ( [FunctionParametersMaybeNamedVariadic](#grammar-summary-FunctionParametersMaybeNamedVariadic)? ) [BareFunctionReturnType](#grammar-summary-BareFunctionReturnType)?

[FunctionTypeQualifiers](#railroad-summary-FunctionTypeQualifiers) → unsafe? ( extern [Abi](#grammar-summary-Abi)? )?

[BareFunctionReturnType](#railroad-summary-BareFunctionReturnType) → -\> [TypeNoBounds](#grammar-summary-TypeNoBounds)

[FunctionParametersMaybeNamedVariadic](#railroad-summary-FunctionParametersMaybeNamedVariadic) →\
    [MaybeNamedFunctionParameters](#grammar-summary-MaybeNamedFunctionParameters) \| [MaybeNamedFunctionParametersVariadic](#grammar-summary-MaybeNamedFunctionParametersVariadic)

[MaybeNamedFunctionParameters](#railroad-summary-MaybeNamedFunctionParameters) →\
    [MaybeNamedParam](#grammar-summary-MaybeNamedParam) ( , [MaybeNamedParam](#grammar-summary-MaybeNamedParam) )\* ,?

[MaybeNamedParam](#railroad-summary-MaybeNamedParam) →\
    [OuterAttribute](#grammar-summary-OuterAttribute)\* ( ( [IDENTIFIER](#grammar-summary-IDENTIFIER) \| \_ ) : )? [Type](#grammar-summary-Type)

[MaybeNamedFunctionParametersVariadic](#railroad-summary-MaybeNamedFunctionParametersVariadic) →\
    ( [MaybeNamedParam](#grammar-summary-MaybeNamedParam) , )\* [MaybeNamedParam](#grammar-summary-MaybeNamedParam) , [OuterAttribute](#grammar-summary-OuterAttribute)\* ...

[ImplTraitType](#railroad-summary-ImplTraitType) → impl [Bounds](#grammar-summary-Bounds)?

[ImplTraitTypeOneBound](#railroad-summary-ImplTraitTypeOneBound) → impl [TraitBound](#grammar-summary-TraitBound)?

[InferredType](#railroad-summary-InferredType) → \_

[NeverType](#railroad-summary-NeverType) → !

[ReferenceType](#railroad-summary-ReferenceType) → & [Lifetime](#grammar-summary-Lifetime)? mut? [TypeNoBounds](#grammar-summary-TypeNoBounds)

[RawPointerType](#railroad-summary-RawPointerType) → \* ( mut \| const ) [TypeNoBounds](#grammar-summary-TypeNoBounds)

[SliceType](#railroad-summary-SliceType) → \[ [Type](#grammar-summary-Type) \]

[TraitObjectType](#railroad-summary-TraitObjectType) → [Bounds](#grammar-summary-Bounds) \| dyn [Bounds](#grammar-summary-Bounds)?

[TraitObjectTypeOneBound](#railroad-summary-TraitObjectTypeOneBound) → [TraitBound](#grammar-summary-TraitBound) \| dyn [TraitBound](#grammar-summary-TraitBound)?

[TupleType](#railroad-summary-TupleType) →\
      ( )\
    \| ( ( [Type](#grammar-summary-Type) , )+ [Type](#grammar-summary-Type)? )

[Type](#railroad-summary-Type) →\
      [TypeNoBounds](#grammar-summary-TypeNoBounds)\
    \| [ImplTraitType](#grammar-summary-ImplTraitType)\
    \| [TraitObjectType](#grammar-summary-TraitObjectType)

[TypeNoBounds](#railroad-summary-TypeNoBounds) →\
      [ParenthesizedType](#grammar-summary-ParenthesizedType)\
    \| [ImplTraitTypeOneBound](#grammar-summary-ImplTraitTypeOneBound)\
    \| [TraitObjectTypeOneBound](#grammar-summary-TraitObjectTypeOneBound)\
    \| [TypePath](#grammar-summary-TypePath)\
    \| [TupleType](#grammar-summary-TupleType)\
    \| [NeverType](#grammar-summary-NeverType)\
    \| [RawPointerType](#grammar-summary-RawPointerType)\
    \| [ReferenceType](#grammar-summary-ReferenceType)\
    \| [ArrayType](#grammar-summary-ArrayType)\
    \| [SliceType](#grammar-summary-SliceType)\
    \| [InferredType](#grammar-summary-InferredType)\
    \| [QualifiedPathInType](#grammar-summary-QualifiedPathInType)\
    \| [BareFunctionType](#grammar-summary-BareFunctionType)\
    \| [MacroInvocation](#grammar-summary-MacroInvocation)

[ParenthesizedType](#railroad-summary-ParenthesizedType) → ( [Type](#grammar-summary-Type) )

Show Railroad

![](../assets/inline/grammar-331.svg) ![](../assets/inline/grammar-332.svg) ![](../assets/inline/grammar-333.svg) ![](../assets/inline/grammar-334.svg) ![](../assets/inline/grammar-335.svg) ![](../assets/inline/grammar-336.svg) ![](../assets/inline/grammar-337.svg) ![](../assets/inline/grammar-338.svg) ![](../assets/inline/grammar-339.svg) ![](../assets/inline/grammar-340.svg) ![](../assets/inline/grammar-341.svg) ![](../assets/inline/grammar-342.svg) ![](../assets/inline/grammar-343.svg) ![](../assets/inline/grammar-344.svg) ![](../assets/inline/grammar-345.svg) ![](../assets/inline/grammar-346.svg) ![](../assets/inline/grammar-347.svg) ![](../assets/inline/grammar-348.svg) ![](../assets/inline/grammar-349.svg) ![](../assets/inline/grammar-350.svg) ![](../assets/inline/grammar-351.svg)
