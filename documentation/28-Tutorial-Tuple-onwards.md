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
