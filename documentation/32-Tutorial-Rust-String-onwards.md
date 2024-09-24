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

____

### Mutable String in Rust

We can create a mutable string in Rust by using the `mut` keyword before assigning a string to a variable. For example,

```rust
// mutable string
let mut word = String::from("cat");
```

We can make changes to this string.

Let's look at an example,

```rust
fn main() {
    let mut word = String::from("cat");
    
    println!("original string = {}", word);
    
    // push a new string at the end of the initial string 
    word.push_str(" dog");
    
    println!("changed string = {}", word);
}
```

#### Output

```bash
original string = cat
changed string = cat dog
```

Here, we create a mutable variable `word` that holds the string `"cat"`. We then push a new string to the end of the original string using `word.push_str(" dog");`.

____