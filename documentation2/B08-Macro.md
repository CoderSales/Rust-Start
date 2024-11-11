# [Macro](https://www.programiz.com/rust/macro)

A macro in Rust is a piece of code that generates another piece of code.

Macros generate code based on input, simplify repetitive patterns, and make code more concise.

Rust macro simply allows us to write code that writes more code which is also known as meta programming. Macros are used extensively in Rust.

Some of the popular Rust macros are `println!`, `vec!` and `panic!`.

____

## Creating a Macro in Rust

We can create a macro using the `macro_rules!` macro. It might be surprising but yes we use a macro to create a macro.

The `macro_rules!` macro has a special syntax.

```rust
macro_rules! macro_name {
    (...) => {...}
    // more match rules
}
```

Here, `() => {}` is the entry for a macro rule. We can have many rules to match for in a single macro.

Let's look at an example of a simple macro that defines a new function to print "Hello, World!".

```rust
// A simple macro named `hello_world`
macro_rules! hello_world {
    // `()` indicates that the macro takes no argument
    () => {
        // The macro will expand into the contents of this block
        println!("Hello, World!")
    };
}

fn main() {
    // Call the hello_world macro
    // This call will expand into `println!("Hello, World!");`
    hello_world!()
}
```

```bash
cargo build
```

```bash
cargo run
```

### Output

```bash
Hello, World!
```
