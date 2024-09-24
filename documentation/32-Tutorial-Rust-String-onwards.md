# [Rust String](https://www.programiz.com/rust/string)

( Back to: [31-Tutorial-Vector-onwards.md](/documentation/31-Tutorial-Vector-onwards.md) )

A string in Rust is a sequence of Unicode characters encoded in UTF-8. For example, `"Rust Programming"` is a string in which each character is a valid Unicode character. i.e. `"R"`, `"u"`, `"s"`, `"t"`, `" "`, and so on.

## Creating a String in Rust

We can create a string with a default value using the `String::from()` method. For example,

```rust
// create a string with a default value
let word = String::from("Hello, World!");
```

Here, we create a new string and assign it to the word variable. We also provide a default value of "Hello, World!".

### Note:

A string is allocated in heap memory and is dynamic (resizable) in size. Hence, the size of string is unknown at compile time.

### Example: Creating a String in Rust

```rust
fn main() {
    // string creation using String::from() method
    let word = String::from("Hello, World!");

    println!("word = {}", word);
}
```

#### Output

```bash
word = Hello, World!
```
