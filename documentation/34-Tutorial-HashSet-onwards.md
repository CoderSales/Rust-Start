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

### 3. Remove Values from a HashSet in Rust

We can use the `remove()` method to remove the specified element from the hashset. For example,

```rust
use std::collections::HashSet;

fn main() {
    let mut colors: HashSet<&str> = HashSet::new();

    colors.insert("Red");
    colors.insert("Yellow");
    colors.insert("Green");

    println!("colors before remove operation = {:?}", colors);

    // remove value from a HashSet
    colors.remove("Yellow");
    
    println!("colors after remove operation = {:?}", colors);
}
```

#### Output

```bash
colors before remove operation = {"Yellow", "Red", "Green"}
colors after remove operation = {"Red", "Green"}
```

In the above example, we have used

```rust
colors.remove("Yellow");
```

to remove the element `Yellow` from the hashset.

____

### 4. Iterate over Values of a HashSet in Rust

We can use the [Rust for Loop](https://www.programiz.com/rust/for-loop) to iterate over values of a hashset. For example,

```rust
use std::collections::HashSet;

fn main() {
    let mut colors: HashSet<&str> = HashSet::new();
    
    colors.insert("Red");
    colors.insert("Yellow");
    colors.insert("Green");

    // iterate over a hashset
    for color in colors {
        // print each value in the hashset
        println!("{}", color);
    }
}
```

#### Output

```bash
Yellow
Green
Red
```

Here, we iterate over the hashset named `colors` and print each element.

##### Note from Running Program:

Order of colors in Output varies with each run.

____

### HashSet with Default Values in Rust

We can also create a hashset with default values using the `from()` method when creating it. For example,

```rust
use std::collections::HashSet;

fn main() {
    // Create HashSet with default set of values using from() method
    let numbers = HashSet::from([2, 7, 8, 10]);
    
    println!("numbers = {:?}", numbers);
}
```

#### Output

```bash
numbers = {8, 7, 10, 2}
```

Here, we create a hashset using the `HashSet::from()` method with default values and print it to the screen.

##### Note from Running Program:

Order of values in Output varies with each run.

____

### Other Methods of Rust HashSet

Besides the basic methods, here are some more commonly used HashSet methods.

| Method | Description |
|--------|-------------|
|`len()`|returns the length of a hashset|
|`is_empty()`|checks if the hashset is empty|
|`clear()`|removes all elements from the hashset|
|`drain()`|returns all the elements as an iterator and clears the hashset|

____

### Set Operations

The HashSet module also provides various methods used to perform different set operations.

#### 1. Union of two Sets

We can use the `union()` method to find the union of two sets. For example,

```rust
use std::collections::HashSet;

fn main() {
    let hashset1 = HashSet::from([2, 7, 8]);
    let hashset2 = HashSet::from([1, 2, 7]);
    
    // Union of hashsets
    let result: HashSet<_> = hashset1.union(&hashset2).collect();
    
    println!("hashset1 = {:?}", hashset1);
    println!("hashset2 = {:?}", hashset2);
    println!("union = {:?}", result);
}
```

##### Output

```bash
hashset1 = {7, 8, 2}
hashset2 = {1, 2, 7}
union = {8, 2, 1, 7}
```

Here, we have used the `union()` method to find the union between two sets: `hashset1` and `hashset2`.

```rust
hashset1.union(&hashset2).collect();
```

The `union()` method returns an iterator, so we have used the `collect()` method to get the actual result.

##### Note:

We have passed `&hashset2` as an argument to the `union()` method because it takes a reference as an argument.

____

### 2. Intersection of two Sets

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

#### Output

```bash
hashset1 = {7, 8, 2}
hashset2 = {2, 1, 7}
intersection = {7, 2}
```

##### Note from Running Program

Order of values in Output vary but the actual values themselves are the same.

____

Here, we have used the union() method to find the union between two sets: hashset1 and hashset2.

____

Aside:

References

____

Git

[git user.name and user.email are not set - Google Search](https://www.google.com/search?q=git+user.name+and+user.email+are+not+set&oq=git+user.name&gs_lcrp=EgZjaHJvbWUqBwgBEAAYgAQyCQgAEEUYORiABDIHCAEQABiABDIHCAIQABiABDIHCAMQABiABDIHCAQQABiABDIHCAUQABiABDINCAYQABiGAxiABBiKBTINCAcQABiGAxiABBiKBTIGCAgQRRhA0gEINDQ4N2owajeoAgCwAgA&sourceid=chrome&ie=UTF-8)


[gitlab - "Make sure you configure your 'user.email' and 'user.name' in git" when trying to push to git lab - Stack Overflow](https://stackoverflow.com/questions/54876421/make-sure-you-configure-your-user-email-and-user-name-in-git-when-trying-t)

Chrome Extension for Referencing

[add google chrome extension from local developer - Google Search](https://www.google.com/search?q=add+google+chrome+extension+from+local+developer&num=10&newwindow=1&sca_esv=9df5b358ca5e7839&sxsrf=ADLYWIK6rO3txntiWzUwm80UOiGFDg7sdA%3A1729947608811&ei=2OccZ72RMc2yhbIPyPGe6Aw&ved=0ahUKEwi93c-FjayJAxVNWUEAHci4B80Q4dUDCA8&uact=5&oq=add+google+chrome+extension+from+local+developer&gs_lp=Egxnd3Mtd2l6LXNlcnAiMGFkZCBnb29nbGUgY2hyb21lIGV4dGVuc2lvbiBmcm9tIGxvY2FsIGRldmVsb3BlcjIFECEYoAEyBRAhGKABMgUQIRigAUjpJlCyBFiKJnACeAGQAQCYAXOgAaMMqgEEMjAuMbgBA8gBAPgBAZgCFqAC5QvCAgoQABiwAxjWBBhHwgIKECEYoAEYwwQYCsICBBAhGArCAggQABiABBiiBMICBRAhGJIDwgIHECEYoAEYCpgDAIgGAZAGCJIHAjIyoAe4YQ&sclient=gws-wiz-serp)


[Hello World extension  |  Chrome Extensions  |  Chrome for Developers](https://developer.chrome.com/docs/extensions/get-started/tutorial/hello-world)

____



____

```rust
hashset1.union(&hashset2).collect();
```

____

The union() method returns an iterator, so we have used the collect() method to get the actual result.

____

Note: We have passed &hashset2 as an argument to the union() method because it takes a reference as an argument.

____
