# Tutorial Functions onwards

( Back to [28-Tutorial-Tuple-onwards.md](/documentation/28-Tutorial-Tuple-onwards.md) )

## [Rust Functions](https://www.programiz.com/rust/function)

Functions are reusable blocks of code that perform a specific task. For example, if we want to create a program to add two numbers, then we can create a Rust function to add numbers. Now, we can reuse this same function whenever we add two numbers.

Creating a function in Rust helps divide our code into smaller blocks and makes our code look cleaner and easier to understand.

Not only in Rust, but functions are also one of the core building blocks of any programming language.

### Define a Function in Rust

In Rust, we use the `fn` keyword to define a function. The syntax of a function is,

```rust
fn function_name(arguments) {
    // code
}
```

Let's see an example.

```rust
fn greet() {
    // code
}
```

Here,

- `fn` - keyword used to create a function in Rust

- `greet()` - name of the function

- `// code` - function body

- `{ }` - start and end of the function body

Now let's complete the `greet()` function to print "Hello, World!".

```rust
// define a function
fn greet() {
    println!("Hello, World!");
}

fn main() {

}
```

When we run this code, we will not get any output. This is because here we are just defining a function. To execute a function, we need to call it.

