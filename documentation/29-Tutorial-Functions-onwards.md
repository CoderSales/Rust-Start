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

____

### Calling a Function in Rust

We use the name of the function and parentheses `()` to call a function.

```rust
// call a function
greet();
```

Let's complete the above example now.

```rust
// define a function
fn greet() {
    println!("Hello, World!");
}

fn main() {
    // function call
    greet();
}
```

#### Output

```bash
Hello, World!
```

Here, we have created a `greet()` function that prints "Hello, World!" on the screen. Notice that we are calling the function from inside `main()`.

##### main() Function in Rust

If you look carefully, you can see the syntax of `main()` looks similar to a function.

```rust
fn main() {
    // function call
    greet();
}
```

In Rust, `main()` is also a function known as a built-in function that has a special meaning. It is **the entry point (start) of every Rust program**.

#### Note

Rust code uses a small case as the convention for defining a function name. An extended function name with multiple words will have underscores in between words.

____

### Example: Function to Add Two Numbers in Rust

```rust
// function to add two numbers
fn add() {
    let a = 5;
    let b = 10;

    let sum = a + b;

    println!("Sum of a and b = {}", sum);
}

fn main() {
    // function call
    add();
}
```

#### Output

```bash
Sum of a and b = 15
```

In the above example, we have created a function named `add()`. The function adds two numbers and prints the sum.

Here's how the program works,

![function.png](/static/images/function.png)
