# [HashSet](https://www.programiz.com/rust/hashset)

( Back to [34-Tutorial-HashMap-onwards.md](/documentation/34-Tutorial-HashSet-onwards.md) )

## Creating a HashSet in Rust (continued)

### Set Operations (continued)

#### 2. Intersection of two Sets

We can use the `intersection()` method to find the intersection between two sets. For example,

```rust
use std::collections::HashSet;

fn main() {
    let hashset1 = HashSet::from([2, 7, 8]);
    let hashset2 = HashSet::from([1, 2, 7]);
    
    // Intersection of hashsets
    let result: HashSet<_> = hashset1.intersection(&hashset2).collect();
    
    println!("hashset1 = {:?}", hashset1);
    println!("hashset2 = {:?}", hashset2);
    println!("intersection = {:?}", result);
}
```

____

##### Aside

###### Note: 

This code was already added to the `main.rs` file in [programiz/attempt-01](../programiz/attempt-01)

____

##### Aside:

###### Note:

code to compile:

(from Rust-Start folder in git bash)

```bash
rustc programiz/attempt-01/main.rs
```

code to run

```bash
programiz/attempt-01/main.exe
```

____
