# [HashSet](https://www.programiz.com/rust/hashset)

( Back to [33-Tutorial-HashMap-onwards.md](/documentation/33-Tutorial-HashMap-onwards.md) )

HashSet implements the set data structure in Rust. Just like a set, it allows us to store values without duplicates.

## Creating a HashSet in Rust

Hashset is part of the Rust standard collections library, so we must include the `HashSet` module in our program.

```rust
use std::collections::HashSet;
```

We have imported the `HashSet` module using the `use` declaration. It should be at the top of the program.

Now, we can create a hashset using the `new()` method of the `HashSet` module. For example,

```rust
let mut color: HashSet<String> = HashSet::new();
```

Here,

- `let mut color` - declares a mutable variable `color`
- `HashSet<String>` - type of the hashset where the values are of type `String`
- `HashSet::new()` - creates a new hashset

____

### Example: Creating a HashSet

```rust
// import HashSet from Rust standard collections library
use std::collections::HashSet;

fn main() {
    // create a new HashSet
    let mut color: HashSet<String> = HashSet::new();
    
    println!("HashSet = {:?}", color);
}
```

#### Output

```bash
HashSet = {}
```

Here, we create an empty `HashSet` and print it to the screen.

##### Note: 

We use `:?` in the `println!` macro to print a hashset.

____

### HashSet Operations in Rust

The `HashSet` module provides various methods to perform basic operations in a hashset.

- Add Values

- Check Values

- Remove Values

- Iterate over Values

____

#### 1. Add Values to a HashSet in Rust

We can use the `insert()` method to add an element to the hashset. For example,

```rust
let mut colors: HashSet<&str> = HashSet::new();

// insert elements to hashset
colors.insert("Red");
colors.insert("Yellow");
```

Here, we insert two values in the `HashSet` bound to the variable `colors`.

##### Note: 

Adding a new value to the hashset is only possible because of the `mut` variable declaration.

____

#### Example: Add Values to a HashSet

```rust
use std::collections::HashSet;

fn main() {
    let mut colors: HashSet<&str> = HashSet::new();
    
    // insert values in a HashSet
    colors.insert("Red");
    colors.insert("Yellow");
    colors.insert("Green");

    println!("colors = {:?}", colors);
}
```

##### Output

```bash
colors = {"Yellow", "Red", "Green"}
```

Here, the output has the elements in a different order. It's because a hashset doesn't preserve the insertion order of values.

____

### 2. Check Value is Present in a HashSet in Rust

We use the `contains()` method to check if a value is present in a hashset. The method returns true if the specified element is present in the hashset, otherwise returns false.

Let's see an example,

```rust
use std::collections::HashSet;

fn main() {
    let mut colors: HashSet<&str> = HashSet::new();

    colors.insert("Red");
    colors.insert("Yellow");

    println!("colors = {:?}", colors);

    // check for a value in a HashSet
    if colors.contains("Red") {
        println!("We have the color \"Red\" in the HashSet.")
    }
}
```

#### Output

```bash
colors = {"Red", "Yellow"}
We have the color "Red" in the HashSet.
```

In the above example, we have used the `colors.contains("Red")` as a condition to the `if` statement.

Here, the element `Red` is present inside the hashset, so the condition is true. Hence, we get the desired output.

____
