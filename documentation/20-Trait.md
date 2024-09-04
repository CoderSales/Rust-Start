____

Items 

- [Traits](https://github.com/CoderSales/Rust-Start/blob/main/documentation/20-Trait.md#trait)

- [Functions]()

____

### Trait

Trait [The Rust Reference | doc.rust-lang.org | Trait objects](https://doc.rust-lang.org/1.80.1/reference/types/trait-object.html)

A trait object is an opaque value of another type that implements a set of traits. The set of traits is made up of an object safe base trait plus any number of auto traits.

....

```rust
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

```bash
10
```

In this example, the trait Printable occurs as a trait object in both the type signature of print, and the cast expression in main.

____

[The Rust Reference | supertraits](https://doc.rust-lang.org/1.80.1/reference/items/traits.html#supertraits)

- [The Rust Reference | generic parameters](https://doc.rust-lang.org/1.80.1/reference/items/generics.html#generic-parameters)

    - [The Rust Reference | where clauses](https://doc.rust-lang.org/1.80.1/reference/items/generics.html#where-clauses)

[The Rust Reference | Parameter patterns](https://doc.rust-lang.org/1.80.1/reference/items/traits.html#parameter-patterns)

____

[The Rust Reference | 10.1.15 | Trait objects](https://doc.rust-lang.org/1.80.1/reference/types/trait-object.html)

- [The Rust Reference | Trait and lifetime bounds](https://doc.rust-lang.org/1.80.1/reference/trait-bounds.html)

____

[The Rust Reference | 10.1.7 | Slice types](https://doc.rust-lang.org/1.80.1/reference/types/slice.html)

____

[The Rust Reference | 2. Lexical structure | 2.6. Tokens |Lifetimes and loop labels](https://doc.rust-lang.org/1.80.1/reference/tokens.html#lifetimes-and-loop-labels)

#### Summary:

```text

` without another ` immediately is a LIFETIME_TOKEN
```

```text
Lexer
LIFETIME_TOKEN :
      ' IDENTIFIER_OR_KEYWORD (not immediately followed by ')
   | '_ (not immediately followed by ')

LIFETIME_OR_LABEL :
      ' NON_KEYWORD_IDENTIFIER (not immediately followed by ')
```

____

[2.6. Tokens | Punctuation](https://doc.rust-lang.org/1.80.1/reference/tokens.html#punctuation)

lists `Punctuation symbol tokens`

- [External blocks](https://doc.rust-lang.org/1.80.1/reference/items/external-blocks.html)

____

[the standard C ABI - Google Search](https://www.google.com/search?q=the+standard+C+ABI&sourceid=chrome&ie=UTF-8)

[The D Programming Language | Application Binary Interface | ABI](https://dlang.org/spec/abi.html#:~:text=The%20C%20ABI%20referred%20to,entire%20C%20ABI%20runtime%20library.)

- [C ABI](https://dlang.org/spec/abi.html#c_abi)

____

[The rustdoc book | How to write documentation | 4.2. The #[doc] attribute](https://doc.rust-lang.org/1.80.1/rustdoc/write-documentation/the-doc-attribute.html)

- [The rustdoc book | Re-exports](https://doc.rust-lang.org/1.80.1/rustdoc/write-documentation/re-exports.html)

____

[The Rust Reference | 7. Attributes](https://doc.rust-lang.org/1.80.1/reference/attributes.html#built-in-attributes-index)

- [The Unstable Book ](https://doc.rust-lang.org/1.80.1/unstable-book/index.html)

____

[The Rust Unstable Book | 2.61 coroutines](https://doc.rust-lang.org/1.80.1/unstable-book/language-features/coroutines.html)

```rust
#![feature(coroutines, coroutine_trait, stmt_expr_attributes)]

use std::ops::Coroutine;
use std::pin::Pin;

fn main() {
    let mut coroutine = #[coroutine] || {
        println!("2");
        yield;
        println!("4");
    };

    println!("1");
    Pin::new(&mut coroutine).resume(());
    println!("3");
    Pin::new(&mut coroutine).resume(());
    println!("5");
}
```

```rust
1
2
3
4
5
```

```text
At this time the main use case of coroutines is an implementation primitive for `async`/`await` and `gen` syntax, but coroutines will likely be extended to other primitives in the future. Feedback on the design and usage is always appreciated!
```

____

[The Rust Reference | 7. Attributes > 7.6 Type System | The non_exhaustive attribute](https://doc.rust-lang.org/1.80.1/reference/attributes/type_system.html#the-non_exhaustive-attribute)

ECMA

- [ECMA-334 | C# language specification](https://ecma-international.org/publications-and-standards/standards/ecma-334/)

    - [TC49 | Programming languages](https://ecma-international.org/technical-committees/tc49/)

        - Scope: .... a Common Language Infrastructure (CLI)

____

[The Rust Reference | 3. Macros | 3.2 Procedural Macros](https://doc.rust-lang.org/1.80.1/reference/procedural-macros.html#attribute-macros)

____

#### Clippy

[The Rust Reference | 7. Attributes | Tool attributes](https://doc.rust-lang.org/1.80.1/reference/attributes.html#tool-attributes)

Google Search: [clippy rust](https://www.google.com/search?q=clippy+rust&num=10&newwindow=1&sca_esv=c9a915b7e2353e00&sca_upv=1&sxsrf=ADLYWIIj54NTmH5FeJRmwOIlUG8Uq-mjOA%3A1725471050790&ei=SpnYZoX3L47BhbIPr4XD-AQ&ved=0ahUKEwiFxLXI6KmIAxWOYEEAHa_CEE8Q4dUDCBA&uact=5&oq=clippy+rust&gs_lp=Egxnd3Mtd2l6LXNlcnAiC2NsaXBweSBydXN0MgUQABiABDIFEAAYgAQyBRAAGIAEMgUQABiABDIFEAAYgAQyBRAAGIAEMgUQABiABDIFEAAYgAQyBRAAGIAEMgUQABiABEiVCFCxBVi-B3ABeAGQAQCYAcACoAGkA6oBBzAuMS4wLjG4AQPIAQD4AQGYAgOgAr4DwgIKEAAYsAMY1gQYR8ICChAAGIAEGEMYigWYAwCIBgGQBgiSBwcxLjEuMC4xoAe2CQ&sclient=gws-wiz-serp)

Result: [Clippy Documentation | doc.rust-lang.org/clippy](https://doc.rust-lang.org/clippy/)

Result: [rust-lang/rust-clippy](https://github.com/rust-lang/rust-clippy)

____

[The Rust Reference | 12. Names | 12.3. Preludes | Tool prelude](https://doc.rust-lang.org/1.80.1/reference/names/preludes.html#tool-prelude)

____

[The Rust Reference | 3. Macros | 3.2. Procedural Macros | Function-like procedural macros](https://doc.rust-lang.org/1.80.1/reference/procedural-macros.html#function-like-procedural-macros)

____

TokenStream [doc.rust-lang.org | Struct proc_macro::TokenStreamCopy item path](https://doc.rust-lang.org/1.80.1/proc_macro/struct.TokenStream.html)

____

#### Functions

[The Rust Reference | 6. Items | 6.4. Functions](https://doc.rust-lang.org/1.80.1/reference/items/functions.html)

A function consists of a block, along with a name, a set of parameters, and an output type.

____

[The rustc book | 2. Command-line Arguments | Linking modifiers: whole-archive](https://doc.rust-lang.org/1.80.1/rustc/command-line-arguments.html#linking-modifiers-whole-archive)

____

[learn.microsoft.com |  | Import Name Type](https://learn.microsoft.com/en-us/windows/win32/debug/pe-format#import-name-type)

____

[The Rust Reference | | Async functions](https://doc.rust-lang.org/1.80.1/reference/items/functions.html#async-functions)

____

6.4. Functions

....

An async function is roughly equivalent to a function that returns `impl Future` and with an `async move` `block` as its body:

```rust
// Source
async fn example(x: &str) -> usize {
    x.len()
}
```

....

[The Rust Reference | | 10.1.16 | | Impl trait](https://doc.rust-lang.org/1.80.1/reference/types/impl-trait.html)


`impl` Trait provides ways to specify unnamed but concrete types that implement a specific trait.

____

[8.2.3. | | async blocks](https://doc.rust-lang.org/1.80.1/reference/expressions/block-expr.html#async-blocks)

____
