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
` before `
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
