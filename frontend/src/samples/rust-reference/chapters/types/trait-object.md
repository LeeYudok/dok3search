# Trait objects

[type.trait-object](#r-type.trait-object)

# Trait objects

[type.trait-object.syntax](#r-type.trait-object.syntax)

**Syntax**\
[TraitObjectType](#railroad-TraitObjectType) → [Bounds](../trait-bounds.md#grammar-Bounds)​[1](#footnote-bare-2021) \| dyn​[2](#footnote-dyn-2018) [Bounds](../trait-bounds.md#grammar-Bounds)?

[TraitObjectTypeOneBound](#railroad-TraitObjectTypeOneBound) → [TraitBound](../trait-bounds.md#grammar-TraitBound)​[1](#footnote-bare-2021) \| dyn​[2](#footnote-dyn-2018) [TraitBound](../trait-bounds.md#grammar-TraitBound)?

Show Railroad

![](../../assets/inline/trait-object-1.svg) ![](../../assets/inline/trait-object-2.svg) [type.trait-object.intro](#r-type.trait-object.intro)

A *trait object* is an opaque value of another type that implements a set of traits. The set of traits is made up of a [dyn compatible](../items/traits.md#dyn-compatibility) *base trait* plus any number of [auto traits](../special-types-and-traits.md#auto-traits).

[type.trait-object.impls](#r-type.trait-object.impls)

Trait objects implement the base trait, its auto traits, and any [supertraits](../items/traits.md#supertraits) of the base trait.

[type.trait-object.bounds](#r-type.trait-object.bounds)

There must be at least one trait bound, there may not be more than one non-auto trait, no more than one lifetime, and opt-out bounds (e.g., `?Sized`) and `use<..>` bounds are not allowed.

For example, given a trait `Trait`, the following are all trait objects:

- `dyn Trait`
- `dyn Trait + Send`
- `dyn Trait + Send + Sync`
- `dyn Trait + 'static`
- `dyn Trait + Send + 'static`
- `dyn Trait +`
- `dyn 'static + Trait`.
- `dyn (Trait)`

[type.trait-object.syntax-edition2021](#r-type.trait-object.syntax-edition2021)

> 2021 Edition differences
>
> Before the 2021 edition, the `dyn` keyword may be omitted. In the 2021 edition and beyond, the `dyn` keyword is required semantically.

[type.trait-object.syntax-edition2018](#r-type.trait-object.syntax-edition2018)

> 2018 Edition differences
>
> In the 2015 edition, `dyn` must be followed by [PathIdentSegment](../paths.md#grammar-PathIdentSegment), [LIFETIME_TOKEN](../tokens.md#grammar-LIFETIME_TOKEN), `for`, `(` or `?` to be interpreted as a keyword instead of a regular identifier.
>
> Most notably, `dyn`, `dyn::T` and `dyn<T>` will all be treated as type paths. As such, if you want a trait object type with the trait `::module::Trait`, you need to put the path in parentheses and write it as `dyn (::module::Trait)`.
>
> Beginning in the 2018 edition, `dyn` is a true keyword and is not allowed in paths, so the parentheses are not necessary.

[type.trait-object.alias](#r-type.trait-object.alias)

Two trait object types alias each other if the base traits alias each other and if the sets of auto traits are the same and the lifetime bounds are the same. For example, `dyn Trait + Send + UnwindSafe` is the same as `dyn Trait + UnwindSafe + Send`.

[type.trait-object.unsized](#r-type.trait-object.unsized)

Due to the opaqueness of which concrete type the value is of, trait objects are [dynamically sized types](../dynamically-sized-types.md). Like all DSTs, trait objects are used behind some type of pointer; for example `&dyn SomeTrait` or `Box<dyn SomeTrait>`. Each instance of a pointer to a trait object includes:

- a pointer to an instance of a type `T` that implements `SomeTrait`
- a *virtual method table*, often just called a *vtable*, which contains, for each method of `SomeTrait` and its [supertraits](../items/traits.md#supertraits) that `T` implements, a pointer to `T`’s implementation (i.e. a function pointer).

The purpose of trait objects is to permit “late binding” of methods. Calling a method on a trait object results in virtual dispatch at runtime: that is, a function pointer is loaded from the trait object vtable and invoked indirectly. The actual implementation for each vtable entry can vary on an object-by-object basis.

An example of a trait object:

``` playground
trait Printable {
    fn stringify(&self) -> String;
}

impl Printable for i32 {
    fn stringify(&self) -> String { self.to_string() }
}

fn print(a: Box<dyn Printable>) {
    println!("{}", a.stringify());
}

fn main() {
    print(Box::new(10) as Box<dyn Printable>);
}
```

In this example, the trait `Printable` occurs as a trait object in both the type signature of `print`, and the cast expression in `main`.

[type.trait-object.lifetime-bounds](#r-type.trait-object.lifetime-bounds)

## Trait object lifetime bounds

Since a trait object can contain references, the lifetimes of those references need to be expressed as part of the trait object. This lifetime is written as `Trait + 'a`. There are [defaults](../lifetime-elision.md#default-trait-object-lifetimes) that allow this lifetime to usually be inferred with a sensible choice.

------------------------------------------------------------------------

1.  

    See [type.trait-object.syntax-edition2021](#r-type.trait-object.syntax-edition2021). [↩](#fr-bare-2021-1) [↩2](#fr-bare-2021-2)

    

2.  

    See [type.trait-object.syntax-edition2018](#r-type.trait-object.syntax-edition2018). [↩](#fr-dyn-2018-1) [↩2](#fr-dyn-2018-2)
