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
