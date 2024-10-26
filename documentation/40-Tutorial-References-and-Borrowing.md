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

```rust
fn calculate_length(s: &String) -> usize { // s is a reference to a String
    s.len()
}
```

____

##### Aside Note

Output unchanged.

____


When, s goes out of scope, at the end of the function, it is not dropped because it does not have ownership of what it refers to.

____

The function call looks like:

```rust
let str = String::from("Hello, World!");

let len = calculate_length(&str);
```

The &str syntax while calling the function lets us create a reference that refers to the value of str but does not own it.

The action of creating a reference is known as borrowing. Borrowing is when we borrow something, and we are done with it, we give it back. It doesn't make us the owner of the data.

##### Note: 

Ampersand (&) represents references, and they allow us to refer to some value without taking ownership of it.

____

