# Tutorial Vector onwards

## [Rust Vector](https://www.programiz.com/rust/vector)

[31-Tutorial-Vector-onwards.md](/documentation/31-Tutorial-Vector-onwards.md)

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
