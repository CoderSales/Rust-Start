# Tutorial Vector onwards

## [Rust Vector](https://www.programiz.com/rust/vector)

( Back to: [30-Tutorial-Closure-onwards.md](/documentation/30-Tutorial-Closure-onwards.md) )

Vector is a dynamic (resizable) data structure that can store a list of elements of the same type. Being a resizable data structure, vectors can grow and shrink at runtime.

### Create a Vector in Rust

In Rust, we can create a vector using the vec! macro. For example,

```rust
let v = vec![1, 2, 3];
```

Here, we are creating a vector using the `vec!` macro with some initial values.

- `let v` - the name of the variable

- `vec![1, 2, 3]` - initialize a vector with integer values **1**, **2**, **3**

By looking at the type of the values provided to the macro, Rust will automatically set the vector type. For example, the vector type of the above vector is `Vec<i32>`.

We can also define the vector type ourselves using the `vec!` macro.

```rust
let v: Vec<u8> = vec![1, 2, 3];
```

Here, we are creating a vector with type u8, which has elements **1**, **2** and **3**.

### Example: Creating a Vector in Rust

```rust
fn main() {    
    // vector creation with vec! macro
    let v = vec![1, 2, 3];
    
    println!("v2= {:?}", v);
}
```

#### Output

```bash
v2= [1, 2, 3]
```

#### Note

Since Rust Vectors 

implement the Debug trait, 

We can use `:?` 

in the `println!` macro 

to print a vector.

____

### Accessing Elements of a Vector in Rust

Each element in a vector is associated with a unique sequence of numbers. This number is known as the **vector index**.

We can access elements of a vector using the vector index. Suppose we have a vector of colors.

```rust
let colors = vec!["blue", "red", "green"];
```

Here's what the indexes for this vector look like,

![vectorIndex.png](/static/images/vectorIndex.png)

We can access individual vector elements using their corresponding vector indexes. For example,

- `colors[0]` - access the element at **index 0** (first element)

- `colors[1]` - access the element at index 1 (second element)

- `colors[2]` - access the element at index 2 (third element)

#### Note

The vector index always starts at **0**; hence the first element of the array is at position **0**, not **1**.

____

### Example: Accessing Elements of a Vector using Vector Index

```rust
fn main() {
    let colors = vec!["blue", "red", "green"];
    
    // method 1: access vector elements using vector index
    println!("first color = {}", colors[0]);
    println!("second color = {}", colors[1]);
    println!("third color = {}", colors[2]);
}
```

#### Output

```bash
first color = blue
second color = red
third color = green
```

### Accessing Elements of a Vector using the get() method in Rust

We can also access the element of the vector with the `get()` method and the index of the element.

Suppose we have a vector of colors:

```rust
let colors = vec!["blue", "red", "green"];
```

We can access the elements of this vector using `get()`. The `get()` method does not directly return the vector element but an enum with type `Option<T>`. The result is either a `Some(T)` or `None`.


- `colors.get(0)` - returns Some value at **index 0**

- `colors.get(1)` - returns Some value at **index 1**

- `colors.get(2)` - returns Some value at **index 2**

The advantage of using the `get()` method over just using the vector index to access the element directly is that it will not error if the vector index is out of range.

Suppose we go out of the vector index range; then `get()` will return `None`. For example,

`colors.get(3)` will return `None`

____
