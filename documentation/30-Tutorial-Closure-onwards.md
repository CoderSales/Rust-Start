# Tutorial Closure onwards

(Back to [29-Tutorial-Functions-onwards.md](/documentation/29-Tutorial-Functions-onwards.md) )

## [Rust Closure](https://www.programiz.com/rust/closure)

In Rust, closures are **functions without names**. They are also known as **anonymous functions** or **lambdas**.


### Defining a Closure in Rust

Here's how we create a closure in Rust,

```rust
// define a closure to print a text
let print_text = || println!("Defining Closure");
```

In the above example, we have created a closure that prints the text "**Defining Closure**". Here,

- `print_text` - variable to store the closure

- `||` - start of a closure

- `println!("Defining Closure")` - body of the closure

### Calling Closure

Once a closure is defined, we need to call it just like calling a function. To call a closure, we use the variable name to which the closure is assigned. For example,

```rust
// define a closure to print a text
let print_text = || println!("Defining Closure");

// call the closure
print_text();
```

Here, `print_text()` calls the closure.

____

### Example: Closure in Rust

```rust
fn main() {
    // closure that prints a text
    let print_text = || println!("Hello, World!");
    
    print_text(); 
}
```

#### Output

```bash
Hello, World!
```

In the above example, we have defined a closure and stored it in the `print_text` variable. We then call the closure using `print_text()`.

____

### Rust Closure with Parameters

In Rust, we can also pass parameters to a closure. For example,

```rust
// define closure to add 1 to an integer
let add_one = |x: i32| x + 1;
```

Here,

- `let add_one` - is the name of the variable to store the the closure

- `|x: i32|` - is the parameter and its type that we pass to the closure

- `x + 1;` - is the body of the closure which returns `x + 1`

If we create a closure with parameters, we need to also pass the value while calling the closure.

```rust
// call the closure with value 2
add_one(2);
```

### Example: Rust Closure with Parameter

```rust
fn main() {
    // define a closure and store it in a variable
    let add_one = |x: i32| x + 1;
    
    // call closure and store the result in a variable
    let result = add_one(2);
    
    println!("Result = {}", result);
}
```

#### Output

```bash
Result = 3
```