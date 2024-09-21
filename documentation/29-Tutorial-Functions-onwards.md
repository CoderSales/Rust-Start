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

### Function Parameters in Rust

From the definition, we know that a function should be reusable. However, the `add()` function in our previous example can only be used to perform the addition of **5** and **10**.

```rust
// function to add two numbers
fn add() {
    let a = 5;
    let b = 10;

    let sum = a + b;

    println!("Sum of a and b = {}", sum);
}
```

This function is not dynamic to be reused.

To deal with this and make our functions more dynamic, we can create functions that accept external values. These external values are called function parameters.

Here's how we can create a function with parameters.

```rust
// function with parameters
fn add(a: i32, b: i32) {
    let sum = a + b;

    println!("Sum of a and b = {}", sum);
}
```

Here,

- `a` and `b` are function parameters

- `i32` is the data type of parameters

To call this function, we should provide some value during the function call.

```rust
add(2, 11);
```

Here, **2** and **11** are known as function arguments that are passed to the add function.

#### Output of adding mentioned modifications to previous example

```bash
Sum of a and b = 13
```

____

### Example: Function Parameters

```rust
// define an add function that takes in two parameters
fn add(a: i32, b: i32) {
    let sum = a + b;
    
    println!("Sum of a and b = {}", sum);
}

fn main() {
    // call add function with arguments
    add(2, 11);
}
```

### Output

```bash
Sum of a and b = 13
```

Here's how the program works,

![functionParameters.png](/static/images/functionParameters.png

The arguments are assigned to the function parameters when we call the function.

- **2** is assigned to `a`

- **11** is assigned to `b`

As a result, we see the sum of **2** and **11** equal to **13** printed on the screen.

____

### Function with Return Value in Rust

In the last example, we computed the sum of two numbers and printed the result inside the function. However, we can also return the result from the function and use it anywhere in our program.

Here's how we can create a function in Rust that returns a value.

```rust
// define an add function that takes in two parameters with a return type
fn add(a: i32, b: i32) -> i32 {
    let sum = a + b;

    // return a value from the function
    return sum;
}
```

Here, `-> i32` before the opening curly bracket `{` indicates the function's return type. In this case, the function will return an `i32` value.

We have then used the `return` keyword to return the `sum` variable from the function.

The function returns the value to the place from where it is called, so the returned value needs to be stored somewhere.

```rust
// store the returned value in a variable
let sum = add(3, 5);
```

____

### Example: Function with Return Value

```rust
// define an add function that takes in two parameters with a return type
fn add(a: i32, b: i32) -> i32 {
    let sum = a + b;

    // return a value from the function
    return sum;
}

fn main() {
    // function call
    let sum = add(3, 5);

    println!("Sum of a and b = {}", sum);
}
```

#### Output

```bash
Sum of a and b = 8
```

Here's how the program works,

![functionWorkingsWithReturn.png](/static/images/functionWorkingsWithReturn.png)

#### Note

```bash
In the above example, when we reach the `return` statement in the add function, it returns the `sum` variable. The returned value is stored in the `sum` variable inside `main()`.
```

## [Variable Scope](https://www.programiz.com/rust/variable-scope)

In computer programming, a variable's scope defines the region in which the variable is available for use. For example,

```rust
fn main() {
    // this variable has scope inside the main function block
    let age = 31;
    …
}
```

Here, the `age` variable has scope inside the body `{...}` of the `main()` function,

#### Note

Each variable in Rust has a scope that is valid inside a block. A block is a collection of statements enclosed by curly braces `{}`.

_____

### Working of Variable Scope in Rust

Let's look at how variable scope works with an example,

```rust
fn main() {
    // scope of outer_var variable is inside the main function code block
    let outer_var = 100;
    
    // start of the inner code block
    {
        // scope of inner_var variable is only inside this new code block
        let inner_var = 200;
        println!("inner_var = {}", inner_var);
    }
    // end of the inner code block
    
    println!("inner_var = {}", inner_var);
    println!("outer_var = {}", outer_var);
}
```

Here, if we try to print the `inner_var` outside of the inner code block, the program fails to compile, and we encounter an error.

#### Error

```bash
error[E0425]: cannot find value `inner_var` in this scope
  --> main.rs:13:32
   |
13 |     println!("inner_var = {}", inner_var);
   |                                ^^^^^^^^^ help: a local variable with a similar name exists: `outer_var`      

error: aborting due to 1 previous error

For more information about this error, try `rustc --explain E0425`.
```

The Rust compiler could not find inner_var in scope as we tried to print the variable outside the inner code block.

To fix this, we can do the following,

```rust
fn main() {
    // scope of outer_var variable is inside the main function code block
    let outer_var = 100;
    
    // start of the inner code block
    {
        // scope of inner_var variable is only inside this new code block
        let inner_var = 200;
        println!("inner_var = {}", inner_var);
        println!("outer_var inside inner block = {}", outer_var);
    }
    // end of the inner code block
    
    println!("outer_var = {}", outer_var);
}
```

#### Output

```bash
inner_var = 200
outer_var inside inner block = 100
outer_var = 100
```

We removed the `println!("inner_var = {}", inner_var);` from the outer code block and the program now works as expected.

Additionally, we can access the `outer_var` inside the inner code block because its scope is in the `main()` function.

Here is how variable scope works in the above program,

![scope.png](/static/images/scope.png)

### Variable Shadowing in Rust

In Rust, when a variable declared within a particular scope has the same name as a variable declared in the outer scope, it is known as **variable shadowing**.

We can use the same variable name in different scope blocks in the same program.

Let's take a look at an example,

```rust
fn main() {
    let random = 100;

    // start of the inner block
    {
        println!("random variable before shadowing in inner block = {}", random);

        // this declaration shadows the outer random variable
        let random = "abc";

        println!("random after shadowing in inner block = {}", random);
    }
    // end of the inner block

    println!("random variable in outer block = {}", random);
}
```

#### Output

```bash
random variable before shadowing in inner block = 100
random after shadowing in inner block = abc
random variable in outer block = 100
```

Here, the `random` variable declared in the outer block is shadowed in the inner block. Let's look at what that means,

```rust
let random = "abc";
```

The `random` variable value inside the inner block will shadow the value of the outer block so that the inner block will have the `"abc"` value. However, the value of the random variable remains the same outside of the inner block.

____

### Variable Freezing in Rust

We can freeze a variable in Rust by using shadowing and immutability. Once a variable is frozen, we cannot change the variable value in the inner scope.

Let's see an example.

```rust
fn main() {
    let mut age = 1;

    // start of the inner block
    {
        // shadowing by immutable age variable
        let age = age;

        // error, age variable is frozen in this scope
        age = 2;

        println!("age variable inner block = {}", age);
        // age variable goes out of scope
    }
    // end of the inner block

    // age variable is not frozen in outer block
    age = 3;

    println!("integer variable outer block = {}", age);
}
```

#### Error

```bash
warning: value assigned to `age` is never read
 --> main.rs:7:13
  |
7 |         let age = age;
  |             ^^^
  |
  = help: maybe it is overwritten before being read?
  = note: `#[warn(unused_assignments)]` on by default

error[E0384]: cannot assign twice to immutable variable `age`
  --> main.rs:10:9
   |
7  |         let age = age;
   |             ---
   |             |
   |             first assignment to `age`
   |             help: consider making this binding mutable: `mut age`
...
10 |         age = 2;
   |         ^^^^^^^ cannot assign twice to immutable variable

error: aborting due to 1 previous error; 1 warning emitted

For more information about this error, try `rustc --explain E0384`.
```

In the above example, we have assigned the mutable variable of the outer block named age to the same immutable variable in the inner scope.

```rust
fn main() {
    let mut age = 100;

    {
        let age = age;
        …
    }
    …
}
```

In doing this, 

we are shadowing the mutable `age` variable 

with an immutable variable named `age`.

Now the `age` variable freezes inside the inner block because the inner `age` variable is pointing to the same value as the `age` variable in the outer block.

Thus, we cannot change the value of `age` inside the inner block and encounter an error.

Once we get out of the inner block, the value of `age` can be changed.

Let's look at the working version of the variable freezing example.

```rust
fn main() {
    let mut age = 100;

    {
        // shadowing by immutable age variable
        let age = age;

        println!("age variable inner block = {}", age);
        // age goes out of scope
    }

    // age variable is not frozen in this scope
    age = 3;

    println!("age variable outer block = {}", age);
}
```

#### Output

```bash

```
