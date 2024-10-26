# [Error Handling](https://www.programiz.com/rust/error-handling)

[Back to 36 Tutorial Iterators onwards md](/documentation/36-Tutorial-Iterators-onwards.md)

____

## Rust Error Handling
An error is an unexpected behavior or event in a program that will produce an unwanted output.

In Rust, errors are of two categories:

Unrecoverable Errors
Recoverable Errors
Unrecoverable Errors in Rust
Unrecoverable errors are errors from which a program stops its execution. As the name suggests, we cannot recover from unrecoverable errors.

These errors are known as panic and can be triggered explicitly by calling the panic! macro.

Let's look at an example that uses the panic! macro.

Example 1: Rust Unrecoverable Errors with panic! Macro


____