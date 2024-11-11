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

In this example, we create a macro named `hello_world`. The macro definition has one rule to match which is:

```rust
() => {
    println!("Hello, World!");
};
```

To call the macro we use the `hello_world!()` call in the `main()` function.

The macro will replace the `hello_world!()` call with the code defined in the macro definition i.e. `println!("Hello, World!)`.

____

## Creating a Macro with Arguments in Rust

Macros can also take arguments, which allows us to customize the code that it generates based on different inputs.

For example, here's a macro that defines a function to print a custom message:

```rust
// A macro named `print_message`
macro_rules! print_message {
    // Match rule that takes an argument expression
    ($message:expr) => {
        println!("{}", $message)
    };
}

fn main() {
    // Call the macro with an argument
    print_message!("I am learning Rust!");
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
I am learning Rust!
```
