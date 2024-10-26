# [References and Borrowing](https://www.programiz.com/rust/references-and-borrowing)

References in Rust allow us to point to a resource (value) without owning it. This means that the original owner of the resource remains the same.

References are helpful when passing values to a function that we do not want to change the ownership of. Creating a reference is known as borrowing in Rust.

## Understanding References in Rust

Let's look at an example to learn about references in Rust.

```rust
fn main() {
    let str = String::from("Hello, World!");
    
    // Call function with reference String value
    let len = calculate_length(&str);

    println!("The length of '{}' is {}.", str, len);
}

// Function to calculate length of a string
// It takes a reference of a String as an argument
fn calculate_length(s: &String) -> usize {
    s.len()
}
```

#### Output

```bash
The length of 'Hello, World!' is 13.
```

In the above example, we define a function called calculate_length() which takes a &String type as an argument.

The important part here is that s is a reference to a String and it doesn't take ownership of the actual value of String.


