[Learn Rust](https://www.programiz.com/rust)

[We use the println! macro](https://www.programiz.com/rust/hello-world)

[use print! macro](https://www.programiz.com/rust/print-output)

[{} is a placeholder which is replaced by the value of the variable after the comma](https://www.programiz.com/rust/print-output)

[add text with the placeholder to format our output](https://www.programiz.com/rust/print-output)

[Note: As per Rust's naming convention, we use uppercase for the name of constants.](https://www.programiz.com/rust/variables-mutability)

[Rust Data Types](https://www.programiz.com/rust/data-types)

i32 signed intgeger

u32 unsigned integer

____

## Categories of Integer Data Types in Rust

Depending on the size of data, we can further classify the signed and unsigned integer type into various categories:

| Size	    | Signed	| Unsigned  |
|-----------|-----------|-----------|
| 8-bit	    | i8	    | u8        |
| 16-bit	| i16	    | u16       |
| 32-bit	| i32	    | u32       |
| 64-bit	| i64	    | u64       |
| 128-bit	| i128	    | u128      |

Search: [table in markdown](https://www.google.com/search?q=table+in+markdown&oq=table+in+markdown&gs_lcrp=EgZjaHJvbWUyCQgAEEUYORiABDIMCAEQABgUGIcCGIAEMgcIAhAAGIAEMgcIAxAAGIAEMgcIBBAAGIAEMgcIBRAAGIAEMgcIBhAAGIAEMgcIBxAAGIAEMg0ICBAAGIYDGIAEGIoFMg0ICRAAGIYDGIAEGIoF0gEIMzE4MWowajeoAgCwAgA&sourceid=chrome&ie=UTF-8)

Result: [Extended Syntax | Markdown Guide](https://www.markdownguide.org/extended-syntax/)

___________________________________

____

## Type Inference

```rust
fn main() {
    let x = 51;

    println!("x = {}", x);
}
```

Output:

```bash
x = 51
```

Here, you can see that we haven't mentioned the data type of x variable. 

It is because Rust will automatically set i32 as the type (default type for integer variable) by looking at the value 51.

[Rust Data Types](https://www.programiz.com/rust/data-types)

____

## [Rust Pattern Matching](https://www.programiz.com/rust/pattern-matching)

### Matching a Variable in Rust

```rust
fn main() {
    let x = 2;

    // use of match expression to pattern match against variable x
    match x {
        1 => println!("x is 1"),
        2 => println!("x is 2"),
        _ => println!("x is something else"),
    }
}
```

#### Ouput

```bash
x is 2
```

### Matching an Enum In Rust

```rust
fn main() {
    enum Color {
        Red,
        Green,
        Blue,
    }

    let my_color = Color::Green;

    // use of match expression to match against an enum variant
    match my_color {
        Color::Red => println!("The color is red"),
        Color::Green => println!("The color is green"),
        Color::Blue => println!("The color is blue"),
    }
}
```

#### Output

```bash
The color is green
```

### Matching Option and Result Type in Rust

The most common case for 

pattern matching 

is with 

`Option` and `Result` enum types. 

Both the `Option` and `Result` type have two variants.

`Option` type has:

- `None` → to indicate failure with no value

- `Some(T)` → a value with type T


`Result` type has:

- `Ok(T)` → operation succeeded with value T

- `Err(E)` → operation failed with an error E

Let's look at examples of how we can use pattern 

matching on these types.

____

#### Example: Matching Option Type in Rust

```rust
fn main() {
    let my_option: Option<i32> = Some(222);
    
    // use of match expression to match Option type
    match my_option {
        Some(value) => println!("The option has a value of {}", value),
        None => println!("The option has no value"),
    }
}
```

```bash
The option has a value of 222
```

In this example, my_option is an Option type that contains either a Some variant with an i32 value or a None variant.

The match expression compares the value of my_option to the Some and None variants, and binds the value of Some variant to the value variable.

When a match is found, the corresponding code block is executed.

![match.jpg](/static/images/match.jpg)

End of Option Type Pattern matching.


#### Example: Matching Result Type in Rust

![ResultType.jpg](/static/images/ResultType.jpg)

![ResultTypeContinued.jpg](/static/images/ResultTypeContinued.jpg)


![IfLetShorthandMatch.png](/static/images/IfLetShorthandMatch.png)

____

## [Rust Array](https://www.programiz.com/rust/array)

```rust
fn main() {
    // initialization of array with data type
    let numbers: [i32; 5] = [1, 2, 3, 4, 5];
    
    println!("Array of numbers = {:?}", numbers);
}
```

```bash
Array of numbers = [1, 2, 3, 4, 5]
```

### Revision: Different Ways to Create Array in Rust

```rust
fn main() {
    // an array without data type
    let a = [5, 4, 3, 2, 1];
    
    // an array with data type and size
    let b: [i32; 5] = [1, 2, 3, 4, 5];
    
    // an array with default values
    let c = [3; 5];
    
    println!("a = {:?}", a);
    println!("b = {:?}", b);
    println!("c = {:?}", c);
}
```

#### Output

```bash
a = [5, 4, 3, 2, 1]
b = [1, 2, 3, 4, 5]
c = [3, 3, 3, 3, 3]
```

#### Note: 

We use `:?` in the `println!` function to print an entire array.

____

![RustArray.png](/static/images/RustArray.png)

____
### Array without Data Type in Rust:

```rust
fn main() {
    // initialization of array without data type

    let numbers = [1, 2, 3, 4, 5];
    println!("array of numbers = {:?}, numbers;
}
```

____

![RustArrayNotes.jpg](/static/images/RustArrayNotes.jpg)

____

### Example: Access Array Elements

```rust
fn main() {
    let colors = ["red", "green", "blue"];
    
    // accessing element at index 0
    println!("1st Color: {}", colors[0]);

    // accessing element at index 1
    println!("2nd Color: {}", colors[1]);

    // accessing element at index 2
    println!("3rd Color: {}", colors[2]);
}
```

#### Output

```bash
1st Color: red
2nd Color: green
3rd Color: blue
```

____

### Mutable array in Rust

```rust
fn main() {
    let mut numbers: [i32; 5] = [1, 2, 3, 4, 5];
    
    println!("original array = {:?}", numbers);
    
    // change the value of the 3rd element in the array
    numbers[2] = 0;
    
    println!("changed array = {:?}", numbers);
}
```

#### Output

```bash
original array = [1, 2, 3, 4, 5]
changed array = [1, 2, 0, 4, 5]
```

Reassignment using:

```rust
    numbers[2] = 0;
```

![RustArrayMutReassign.png](/static/images/RustArrayMutReassign.png)

____

### Looping Through an Array in Rust

```rust
fn main() {
    let colors = ["red", "green", "blue"];
    
    // loop through an array to print its index and value
    for index in 0..3 {
        println!("Index: {} -- Value: {}", index, colors[index]);
    }
}
```

#### Output

```bash
Index: 0 -- Value: red
Index: 1 -- Value: green
Index: 2 -- Value: blue
```

____
____

#### Rust Slice

```rust
fn main() {
    // an array of numbers
    let numbers = [1, 2, 3, 4, 5];
    
    // create a slice of 2nd and 3rd element
    let slice = &numbers[1..3];
    
    println!("array = {:?}", numbers);
    println!("slice = {:?}", slice);
}
```

```bash
array = [1, 2, 3, 4, 5]
slice = [2, 3]
```

#### Note

A slice is not the actual data like integers or floats but a reference/pointer to the data block. That's why we have used the & symbol before the variable name.

____

### Omit Indexes of a Rust Slice

```rust
fn main() {
    let numbers = [1, 2, 3, 4, 5];

    // omit the start index
    let slice = &numbers[..3];

    println!("array = {:?}", numbers);
    println!("slice = {:?}", slice);
}
```


```bash
array = [1, 2, 3, 4, 5]
slice = [1, 2, 3]
```

____

![RustSliceOmitStart.png](/static/images/RustSliceOmitStart.png)

____


### 2. Omitting the End Index of a Slice

```rust
fn main() {
    let numbers = [1, 2, 3, 4, 5];

    // omit the end index
    let slice = &numbers[2..];

    println!("array = {:?}", numbers);
    println!("slice = {:?}", slice);
}
```

#### Output

```bash
array = [1, 2, 3, 4, 5]
slice = [3, 4, 5]
```