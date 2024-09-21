# Tutorial-Tuple-onwards

(Back to [27-Tutorial.md](/documentation/27-Tutorial.md) )

## [Rust Tuple](https://www.programiz.com/rust/tuple)

### Example: Tuple with Data Type

```rust 
fn main() {
    // initialization of tuple with data type
    let tuple: (&str, f32, u8) = ("Rust", 3.14, 100);
    
    println!("Tuple contents = {:?}", tuple);
}
```

#### Ouptut

```bash
Tuple contents = ("Rust", 3.14, 100)
```

#### Note

##### `:?` operator

We use the `:?` in the `println!` function to print an entire tuple.

____

### Tuple without Data Type in Rust

We can create a tuple **without mentioning the type of data** it is storing. 

The Rust compiler can automatically detect and set the data type. 

For example,

```rust
// create a tuple without data type
let student_info = ("Ricky", 21, 3.56);
```

Here,

`let student_info` - specifies the variable name of the tuple

`("Ricky", 21, 3.56)` - specifies the elements of the tuple

____

### Example: Tuple without Data Types

```rust
fn main() {
    // initialization of tuple without data type
    let tuple = ("Rust", "fun", 100);

    println!("Tuple contents = {:?}", tuple);
}
```

#### Output

```bash
Tuple contents = ("Rust", 3.14, 100)
```

____

### Accessing Elements in a Tuple

Each element in a tuple is associated with a unique sequence of numbers. 

This number is known as the tuple index or just index.

Suppose we have a tuple,

```rust
let random_tuple = ("Hello", 200, 3.14);
```

The tuple indexes of the `random_tuple` look like,

![TupleIndexImage.png](/static/images/TupleIndexImage.png)

#### Tuple Dot notation access

In Rust, we can access individual tuple elements using their corresponding tuple indexes and the dot . notation. 

For example,

- `random_tuple.0` - access the element at `index 0` (first element)

- `random_tuple.1` - access the element at `index 1` (second element)

- `random_tuple.2` - access the element at `index 2` (third element)

_____

### Example: Access Tuple Elements

```rust
fn main() {
    let random_tuple = ("Hello", 200, 3.14);

    // accessing tuple element at index 0
    println!("Value at Index 0 = {}", random_tuple.0);
    
    // accessing tuple element at index 1
    println!("Value at Index 1 = {}", random_tuple.1);
    
    // accessing tuple element at index 2
    println!("Value at Index 2 = {}", random_tuple.2);
}
```

```bash
Value at Index 0 = Hello
Value at Index 1 = 200
Value at Index 2 = 3.14
```

#### Note: 

The tuple index always starts at 0; hence the first element of the tuple is at position 0, not 1.

____

### Mutable Tuple

#### `mut` Keyword

In Rust, a tuple is immutable, which means we cannot change its elements once it is created.

However, we can create a mutable array by using the mut keyword before assigning it to a variable. For example,

```rust
// create a mutable tuple 
let mut mountains = ("Everest", 8848, "Fishtail", 6993);
```

Now, we can make changes to this tuple.

Let's take a look at an example,



```rust
fn main() {
    // initialize a mutable tuple
    let mut mountain_heights = ("Everest", 8848, "Fishtail", 6993);
    
    println!("Original tuple = {:?}", mountain_heights);
    
    // change 3rd and 4th element of a mutable tuple
    mountain_heights.2 = "Lhotse";
    mountain_heights.3 = 8516;
    
    println!("Changed tuple = {:?}", mountain_heights);
}
```

#### Output

```bash
Original tuple = ("Everest", 8848, "Fishtail", 6993)
Changed tuple = ("Everest", 8848, "Lhotse", 8516)
```

#### Note

Here, we create a mutable tuple named `mountain_heights`. 

We then change its 

`3rd` and `4th` element, the 

`2nd` and `3rd` tuple index.


