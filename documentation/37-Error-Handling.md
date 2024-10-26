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
