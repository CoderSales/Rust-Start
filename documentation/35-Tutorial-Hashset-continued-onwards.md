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

##### Output

```bash
hashset1 = {2, 7, 8}
hashset2 = {1, 2, 7}
intersection = {2, 7}
```

____

##### Aside

###### Note

Key point above here seems to be 

the line

```rust
intersection = {2, 7}
```

in output, 

which seems to be

The intersection HashSet

____

#### 3. Difference between two Sets

We can use the `difference()` method to find the difference between two sets. For example,

```rust
use std::collections::HashSet;

fn main() {
    let hashset1 = HashSet::from([1, 2, 3, 4]);
    let hashset2 = HashSet::from([4, 3, 2]);
    
    // Difference between hashsets
    let result: HashSet<_> = hashset1.difference(&hashset2).collect();
    
    println!("hashset1 = {:?}", hashset1);
    println!("hashset2 = {:?}", hashset2);
    println!("difference = {:?}", result);
}
```

#### Output

```bash
hashset1 = {4, 1, 2, 3}
hashset2 = {4, 3, 2}
difference = {1}
```

____

#### 4. Symmetric Difference between two Sets

We can use the `symmetric_difference()` method to find the symmetric difference between two sets. The symmetric difference returns values from both sets except the ones in both.

```rust
use std::collections::HashSet;

fn main() {
    let hashset1 = HashSet::from([2, 7, 8]);
    let hashset2 = HashSet::from([1, 2, 7, 9]);
    
    // Symmetric difference of hashsets
    let result: HashSet<_> = hashset1.symmetric_difference(&hashset2).collect();
    
    println!("hashset1 = {:?}", hashset1);
    println!("hashset2 = {:?}", hashset2);
    println!("symmetric difference = {:?}", result);
}
```
