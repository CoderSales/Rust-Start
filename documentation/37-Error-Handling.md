# [Error Handling](https://www.programiz.com/rust/error-handling)

[Back to 36 Tutorial Iterators onwards md](/documentation/36-Tutorial-Iterators-onwards.md)

____

## Rust Error Handling

An error is an unexpected behavior or event in a program that will produce an unwanted output.

In Rust, errors are of two categories:

- Unrecoverable Errors

- Recoverable Errors

____

### Unrecoverable Errors in Rust

Unrecoverable errors are errors from which a program stops its execution. As the name suggests, we cannot recover from unrecoverable errors.

These errors are known as **panic** and can be triggered explicitly by calling the `panic!` macro.

Let's look at an example that uses the `panic!` macro.

#### Example 1: Rust Unrecoverable Errors with panic! Macro

```rust
fn main() {
    println!("Hello, World!");

    // Explicitly exit the program with an unrecoverable error
    panic!("Crash");
}
```

##### Output

```bash
Hello, World!
thread 'main' panicked at main.rs:5:5:
Crash
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

Here, the call to the `panic!` macro causes an unrecoverable error.

```bash
thread 'main' panicked at 'Crash', src/main.rs:5:5
```
Notice that the program still runs the expressions above `panic!` macro. We can still see `Hello, World!` printed to the screen before the error message.

The `panic!` macro takes in an error message as an argument.

____

____
