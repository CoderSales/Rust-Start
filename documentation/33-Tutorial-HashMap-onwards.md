# [HashMap](https://www.programiz.com/rust/hashmap)

( Back to: [32-Tutorial-Rust-String-onwards.md](/documentation/32-Tutorial-Rust-String-onwards.md) )

The Rust HashMap data structure allows us to store data in **key-value pairs**. Here are some of the features of hashmap:

- Each value is associated with a corresponding key.

- Keys are unique, whereas values can duplicate.

- Values can be accessed using their corresponding keys.

____

## Creating a HashMap in Rust

HashMap is part of the Rust standard collections library, so we must include the `HashMap` module in our program to use it.

```rust
use std::collections::HashMap;
```

We can import the `HashMap` module using the `use` declaration. It should be at the top of the program.

Now, we can create a hashmap using the `new()` method in the `HashMap` module. For example,

```rust
let mut info: HashMap<i32, String> = HashMap::new();
```

Here,

- `let mut info` - declares a mutable variable `info`

- `HashMap<i32, String>` - type of the HashMap where the key is a Integer and the value is a String

- `HashMap::new()` - creates a new HashMap

____

### Example: Creating a HashMap

```rust
// import HashMap from Rust standard collections library
use std::collections::HashMap;

fn main() {
    // create a new HashMap
    let mut info: HashMap<i32, String> = HashMap::new();
    
    println!("HashMap = {:?}", info);
}
```

#### Output

```bash
HashMap = {}
```

Here, we create an empty HashMap and print it to the screen.

##### Note:

We use :? in the println! macro to print a HashMap.

____
