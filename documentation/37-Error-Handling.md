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

#### Example 2: Rust Unrecoverable Errors

Unrecoverable errors are also triggered by taking an action that might cause our code to panic. For example, accessing an array past its index will cause a panic.

```rust
fn main() {
    let numbers = [1, 2 ,3];

    println!("unknown index value = {}", numbers[3]);
}
```

##### Output

```bash
error: this operation will panic at runtime
 --> main.rs:4:42
  |
4 |     println!("unknown index value = {}", numbers[3]);
  |                                          ^^^^^^^^^^ index out of bounds: the length is 3 but the index is 3  
  |
  = note: `#[deny(unconditional_panic)]` on by default

error: aborting due to 1 previous error
```

Here, Rust stops us from compiling the program because it knows the operation will panic at runtime.

The array `numbers` does not have a value at index **3** i.e. `numbers[3]`.

____

#### Recoverable Errors

Recoverable errors are errors that won't stop a program from executing. Most errors are recoverable, and we can easily take action based on the type of error.

For example, if you try to open a file that doesn't exist, you can create the file instead of stopping the execution of the program or exiting the program with a panic.

Let's look at an example.

```rust
use std::fs::File;

fn main() {
    let data_result = File::open("data.txt");

    // using match for Result type
    let data_file = match data_result {
        Ok(file) => file,
        Err(error) => panic!("Problem opening the data file: {:?}", error),
    };

    println!("Data file", data_file);
}
```

#### Compiler Warning / Output / Error 

```bash
error: argument never used
  --> main.rs:12:27
   |
12 |     println!("Data file", data_file);
   |              -----------  ^^^^^^^^^ argument never used
   |              |
   |              formatting specifier missing

error: aborting due to 1 previous error
```

#### Output

```bash
Hello, World!
thread 'main' panicked at main.rs:5:5:
Crash
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

____

##### Note output above is different to Tutorial

only compiler warning shown here and in `main.rs` file

###### Rerun with data.txt file present

copy in compiler warning and runtime error

____

If the data.txt file exists, the output is:

```bash
Data file: File { fd: 3, path: "/playground/data.txt", read: true, write: false }
If the data.txt file doesn't exist, the output is:
```

```bash
thread 'main' panicked at 'Problem opening the data file: Os { code: 2, kind: NotFound, message: "No such file or directory" }', src/main.rs:8:23
```

____

##### Debug difference with tutorial

###### Part 1

____

**References**

[| formatting specifier missing - Google Search](https://www.google.com/search?q=%7C+formatting+specifier+missing&oq=%7C++++++++++++++formatting+specifier+missing&gs_lcrp=EgZjaHJvbWUyBggAEEUYOTIHCAEQABiABDIHCAIQABiABDINCAMQABiGAxiABBiKBTINCAQQABiGAxiABBiKBTINCAUQABiGAxiABBiKBTIKCAYQABiABBiiBDIKCAcQABiABBiiBDIKCAgQABiABBiiBNIBBzk2N2owajeoAgCwAgA&sourceid=chrome&ie=UTF-8)


[python - Format specifier missing precision error with string formatting - Stack Overflow](https://stackoverflow.com/questions/60770948/format-specifier-missing-precision-error-with-string-formatting)

____

###### Part 2

Explain E277

(basically tried to print something that was not printable )

```bash
$ rustc --explain E0277 main.rs
You tried to use a type which doesn't implement some trait in a place which
expected that trait.

Erroneous code example:

```
// here we declare the Foo trait with a bar method
trait Foo {
    fn bar(&self);
}

// we now declare a function which takes an object implementing the Foo trait
fn some_func<T: Foo>(foo: T) {
    foo.bar();
}

fn main() {
    // we now call the method with the i32 type, which doesn't implement
    // the Foo trait
    some_func(5i32); // error: the trait bound `i32 : Foo` is not satisfied
}
```

In order to fix this error, verify that the type you're using does implement
the trait. Example:

```
trait Foo {
    fn bar(&self);
}

// we implement the trait on the i32 type
impl Foo for i32 {
    fn bar(&self) {}
}

fn some_func<T: Foo>(foo: T) {
    foo.bar(); // we can now use this method since i32 implements the
               // Foo trait
}

fn main() {
    some_func(5i32); // ok!
}
```

Or in a generic context, an erroneous code example would look like:

```
fn some_func<T>(foo: T) {
    println!("{:?}", foo); // error: the trait `core::fmt::Debug` is not
                           //        implemented for the type `T`
}

fn main() {
    // We now call the method with the i32 type,
    // which *does* implement the Debug trait.
    some_func(5i32);
}
```

Note that the error here is in the definition of the generic function. Although
we only call it with a parameter that does implement `Debug`, the compiler
still rejects the function. It must work with all possible input types. In
order to make this example compile, we need to restrict the generic type we're
accepting:

```
use std::fmt;

// Restrict the input type to types that implement Debug.
fn some_func<T: fmt::Debug>(foo: T) {
    println!("{:?}", foo);
}

fn main() {
    // Calling the method is still fine, as i32 implements Debug.
    some_func(5i32);

    // This would fail to compile now:
    // struct WithoutDebug;
    // some_func(WithoutDebug);
}
```

Rust only looks at the signature of the called function, as such it must
already specify all requirements that will be used for every type parameter.
```


____

### The Result Enum

In the above example, the return type of the `File::open('data.txt')` is a `Result<T, E>`.

The `Result<T, E>` type returns either a value or an error in Rust. It is an `enum` type with two possible variants.

- `Ok(T)` → operation succeeded with value `T`

- `Err(E)` → operation failed with an error `E`

Here, `T` and `E` are generic types. To know more about generics or generic types, visit [Rust Generics](https://www.programiz.com/rust/generics).

The most basic way to see whether a `Result` enum has a value or error is to use pattern matching with a `match` expression.

```rust
// data_file is a Result<T, E>
match data_result {
    Ok(file) => file,
    Err(error) => panic!("Problem opening the data file: {:?}", error),
 };
```

When the result is `Ok`, this code will return the `file`, and when the result is `Err`, it will return a `panic!`.

To learn more about pattern matching, visit [Rust Pattern Matching](https://www.programiz.com/rust/pattern-matching).

____

### The Option Enum

The Option type or Option<T> type is an enum type just like Result with two possible variants.

- None → to indicate failure with no value

- Some(T) → a value with type T

Let's look at an example,

```rust
fn main() {
    let text = "Hello, World!";
    
    let character_option = text.chars().nth(15);
    
    // using match for Option type
    let character = match character_option {
        None => "empty".to_string(),
        Some(c) => c.to_string()
    };
    
    println!("Character at index 15 is {}", character);
}
```

#### Output

```bash

```
