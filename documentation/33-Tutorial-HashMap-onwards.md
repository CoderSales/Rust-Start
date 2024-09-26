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

### HashMap Operations in Rust

The `HashMap` module provides various methods to perform basic operations in a hashmap.

- Add Elements

- Access Values

- Remove Elements

- Change Elements

____

### 1. Add Elements to a HashMap in Rust

We can use the `insert()` to add an element (key-value pairs) to a hashmap. For example,

```rust
let mut fruits: HashMap<i32, String> = HashMap::new();

// insert elements to hashmap
fruits.insert(1, String::from("Apple"));
fruits.insert(2, String::from("Banana"));
```

Here, we insert two key-value pairs in the `HashMap` bound to the variable `fruits`. The `String::from()` method here creates a value of `String` type.

#### Note:

Adding a new key-value to the HashMap is only possible because of the mut variable declaration.

____

#### Example: Add Elements to a HashMap

```rust
use std::collections::HashMap;

fn main() {
    let mut fruits: HashMap<i32, String> = HashMap::new();
    
    // add key-value in a hashmap
    fruits.insert(1, String::from("Apple"));
    fruits.insert(2, String::from("Banana"));
    
    println!("fruits = {:?}", fruits);
}
```

#### Output

```bash
fruits = {2: "Banana", 1: "Apple"}
```

____

### 2. Access Values in a HashMap in Rust

We can use the `get()` to access a value from the given hashmap. For example,

```rust
let mut fruits: HashMap<i32, String> = HashMap::new();

fruits.insert(1, String::from("Apple"));
fruits.insert(2, String::from("Banana"));

let first_fruit = fruits.get(&1);
```

Here, we get a value out of the hashmap using the key `&1` and the `get()` method.

We use the ampersand(`&`) and the key (`&1`) as the argument because `get()` returns us a reference of the value. It is not the actual value in the HashMap.

____

#### Example: Access Values in a HashMap

```rust
use std::collections::HashMap;

fn main() {
    let mut fruits: HashMap<i32, String> = HashMap::new();
    
    // insert elements in a hashmap
    fruits.insert(1, String::from("Apple"));
    fruits.insert(2, String::from("Banana"));
    
    // access values in a hashmap
    let first_fruit = fruits.get(&1);
    let second_fruit = fruits.get(&2);
    let third_fruit = fruits.get(&3);
    
    println!("first fruit = {:?}", first_fruit);
    println!("second fruit = {:?}", second_fruit);
    println!("third fruit = {:?}", third_fruit);
}
```

#### Output

```bash
first fruit = Some("Apple")
second fruit = Some("Banana")
third fruit = None
```

Notice that we use the ampersand(`&`) and the key (`&1`, `&2`) as an argument to the `get()` method.

```rust
let first_fruit = fruits.get(&1);
let second_fruit = fruits.get(&2); 
```

The output of the `get()` method is an `Option` enum which means that if the key passed as an argument matches, it returns `Some` value, and if it doesn't, it returns `None`.

In the above example, `let third_fruit = fruits.get(&3)` returns `None` because the key `&3` doesn't match anything that's in the hashmap.

____

### 3. Remove Elements from a HashMap in Rust

We can remove elements from a hashmap by providing a key to the `remove()` method. For example,

```rust
let mut fruits: HashMap<i32, String> = HashMap::new();

fruits.insert(1, String::from("Apple"));
fruits.insert(2, String::from("Banana"));

fruits.remove(&1);
```

Here, we remove a value from the hashmap using the key and the `remove()` method.

____

### Example: Remove Elements in a HashMap

```rust
use std::collections::HashMap;

fn main() {
    let mut fruits: HashMap<i32, String> = HashMap::new();
    
    // insert values in a hashmap
    fruits.insert(1, String::from("Apple"));
    fruits.insert(2, String::from("Banana"));
    
    println!("fruits before remove operation = {:?}", fruits);

    // remove value in a hashmap
    fruits.remove(&1);
    
    println!("fruits after remove operation = {:?}", fruits);
}
```

#### Output

```bash
fruits before remove operation = {2: "Banana", 1: "Apple"}
fruits after remove operation = {2: "Banana"}
```

Here, we remove an element in the hashmap with key `&1` using the `remove()` method.

____
