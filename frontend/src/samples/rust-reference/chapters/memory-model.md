# Memory model

[memory](#r-memory)

# Memory model

[memory.intro](#r-memory.intro)

> ![](../assets/inline/memory-model-1.svg)Warning
>
> The memory model of Rust is incomplete and not fully decided.

[memory.bytes](#r-memory.bytes)

## Bytes

[memory.bytes.intro](#r-memory.bytes.intro)

The most basic unit of memory in Rust is a byte.

> ![](../assets/inline/memory-model-2.svg)Note
>
> While bytes are typically lowered to hardware bytes, Rust uses an “abstract” notion of bytes that can make distinctions which are absent in hardware, such as being uninitialized, or storing part of a pointer. Those distinctions can affect whether your program has undefined behavior, so they still have tangible impact on how compiled Rust programs behave.

[memory.bytes.contents](#r-memory.bytes.contents)

Each byte may have one of the following values:

[memory.bytes.init](#r-memory.bytes.init)

- An initialized byte containing a `u8` value and optional [provenance](https://doc.rust-lang.org/1.100.0/core/ptr/index.html#provenance),

[memory.bytes.uninit](#r-memory.bytes.uninit)

- An uninitialized byte.

> ![](../assets/inline/memory-model-3.svg)Note
>
> The above list is not yet guaranteed to be exhaustive.
