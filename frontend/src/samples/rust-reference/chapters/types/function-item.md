# Function item types

[type.fn-item](#r-type.fn-item)

# Function item types

[type.fn-item.intro](#r-type.fn-item.intro)

When referred to, a function item, or the constructor of a tuple-like struct or enum variant, yields a [zero-sized](../glossary.md#r-glossary.zst) value of its *function item type*.

[type.fn-item.unique](#r-type.fn-item.unique)

That type explicitly identifies the function - its name, its type arguments, and its early-bound lifetime arguments (but not its late-bound lifetime arguments, which are only assigned when the function is called) - so the value does not need to contain an actual function pointer, and no indirection is needed when the function is called.

[type.fn-item.name](#r-type.fn-item.name)

There is no syntax that directly refers to a function item type, but the compiler will display the type as something like `fn(u32) -> i32 {fn_name}` in error messages.

Because the function item type explicitly identifies the function, the item types of different functions - different items, or the same item with different generics - are distinct, and mixing them will create a type error:

``` playground
#![allow(unused)]
fn main() {
fn foo<T>() { }
let x = &mut foo::<i32>;
*x = foo::<u32>; //~ ERROR mismatched types
}
```

[type.fn-item.coercion](#r-type.fn-item.coercion)

However, there is a [coercion](../type-coercions.md) from function items to [function pointers](function-pointer.md) with the same signature, which is triggered not only when a function item is used when a function pointer is directly expected, but also when different function item types with the same signature meet in different arms of the same `if` or `match`:

``` playground
#![allow(unused)]
fn main() {
let want_i32 = false;
fn foo<T>() { }

// `foo_ptr_1` has function pointer type `fn()` here
let foo_ptr_1: fn() = foo::<i32>;

// ... and so does `foo_ptr_2` - this type-checks.
let foo_ptr_2 = if want_i32 {
    foo::<i32>
} else {
    foo::<u32>
};
}
```

[type.fn-item.traits](#r-type.fn-item.traits)

All function items implement [`Copy`](../special-types-and-traits.md#copy), [`Clone`](../special-types-and-traits.md#clone), [`Send`](../special-types-and-traits.md#send), and [`Sync`](../special-types-and-traits.md#sync).

[`Fn`](../../core/ops/function/trait.Fn.html), [`FnMut`](../../core/ops/function/trait.FnMut.html), and [`FnOnce`](../../core/ops/function/trait.FnOnce.html) are implemented unless the function has any of the following:

- an [`unsafe`](../unsafe-keyword.md#r-unsafe.fn) qualifier
- a [`target_feature` attribute](../attributes/codegen.md#r-attributes.codegen.target_feature)
- an [ABI](../items/functions.md#r-items.fn.extern) other than `"Rust"`
