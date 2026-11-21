# Constant items

[items.const](#r-items.const)

# Constant items

[items.const.syntax](#r-items.const.syntax)

**Syntax**\
[ConstantItem](#railroad-ConstantItem) →\
    const ( [IDENTIFIER](../identifiers.md#grammar-IDENTIFIER) \| \_ ) : [Type](../types.md#grammar-Type) ( = [Expression](../expressions.md#grammar-Expression) )? ;

Show Railroad

![](../../assets/inline/constant-items-1.svg) [items.const.intro](#r-items.const.intro)

A *constant item* is an optionally named *[constant value](../const_eval.md#constant-expressions)* which is not associated with a specific memory location in the program.

[items.const.behavior](#r-items.const.behavior)

Constants are essentially inlined wherever they are used, meaning that they are copied directly into the relevant context when used. This includes use of constants from external crates, and non-[`Copy`](../special-types-and-traits.md#copy) types. References to the same constant are not necessarily guaranteed to refer to the same memory address.

[items.const.namespace](#r-items.const.namespace)

The constant declaration defines the constant value in the [value namespace](../names/namespaces.md) of the module or block where it is located.

[items.const.static](#r-items.const.static)

Constants must be explicitly typed. The type must have a `'static` lifetime: any references in the initializer must have `'static` lifetimes. References in the type of a constant default to `'static` lifetime; see [static lifetime elision](../lifetime-elision.md#const-and-static-elision).

[items.const.static-temporary](#r-items.const.static-temporary)

A reference to a constant will have `'static` lifetime if the constant value is eligible for [promotion](../destructors.md#r-destructors.scope.const-promotion); otherwise, a temporary will be created.

``` playground
#![allow(unused)]
fn main() {
const BIT1: u32 = 1 << 0;
const BIT2: u32 = 1 << 1;

const BITS: [u32; 2] = [BIT1, BIT2];
const STRING: &'static str = "bitstring";

struct BitsNStrings<'a> {
    mybits: [u32; 2],
    mystring: &'a str,
}

const BITS_N_STRINGS: BitsNStrings<'static> = BitsNStrings {
    mybits: BITS,
    mystring: STRING,
};
}
```

[items.const.expr-omission](#r-items.const.expr-omission)

The constant expression may only be omitted in a [trait definition](traits.md).

[items.const.destructor](#r-items.const.destructor)

## Constants with destructors

Constants can contain destructors. Destructors are run when the value goes out of scope.

``` playground
#![allow(unused)]
fn main() {
struct TypeWithDestructor(i32);

impl Drop for TypeWithDestructor {
    fn drop(&mut self) {
        println!("Dropped. Held {}.", self.0);
    }
}

const ZERO_WITH_DESTRUCTOR: TypeWithDestructor = TypeWithDestructor(0);

fn create_and_drop_zero_with_destructor() {
    let x = ZERO_WITH_DESTRUCTOR;
    // x gets dropped at end of function, calling drop.
    // prints "Dropped. Held 0.".
}
}
```

[items.const.unnamed](#r-items.const.unnamed)

## Unnamed constant

[items.const.unnamed.intro](#r-items.const.unnamed.intro)

Unlike an [associated constant](associated-items.md#associated-constants), a [free](../glossary.md#free-item) constant may be unnamed by using an underscore instead of the name. For example:

``` playground
#![allow(unused)]
fn main() {
const _: () =  { struct _SameNameTwice; };

// OK although it is the same name as above:
const _: () =  { struct _SameNameTwice; };
}
```

[items.const.unnamed.repetition](#r-items.const.unnamed.repetition)

As with [underscore imports](use-declarations.md#underscore-imports), macros may safely emit the same unnamed constant in the same scope more than once. For example, the following should not produce an error:

``` playground
#![allow(unused)]
fn main() {
macro_rules! m {
    ($item: item) => { $item $item }
}

m!(const _: () = (););
// This expands to:
// const _: () = ();
// const _: () = ();
}
```

[items.const.eval](#r-items.const.eval)

## Evaluation

[Free](../glossary.md#free-item) constants are always [evaluated](../const_eval.md) at compile-time to surface panics. This happens even within an unused function:

``` playground
#![allow(unused)]
fn main() {
// Compile-time panic
const PANIC: () = std::unimplemented!();

fn unused_generic_function<T>() {
    // A failing compile-time assertion
    const _: () = assert!(usize::BITS == 0);
}
}
```
