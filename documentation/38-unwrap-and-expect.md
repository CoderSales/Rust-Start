# [Rust unwrap() and expect()](https://www.programiz.com/rust/unwrap-and-expect)

[Back to 37  .... .md](/documentation/37-Error-Handling.md)

____

The unwrap() are expect() utility methods that work with Option and Result types in Rust.

____

## The unwrap() Method

Unwrap in Rust returns the result of the operation for Option and Result enums. If unwrap encounters an error Err or a None, it will panic and stop the program execution.

Unwrap method is defined on both Option and Result type.

An Option enum type can be handled by using the match expression as well as unwrap().

### Example: Using the match Expression

```rust
// function to find a user by their username which returns an Option type
fn get_user(username: &str) -> Option<&str> {
    if username.is_empty() {
        return None;
    }

    return Some(username);
}

fn main() {
    // returns an Option
    let user_option = get_user("Hari");

    // use of match expression to get the result out of Option
    let result = match user_option {
        Some(user) => user,
        None => "not found!",
    };

    // print the result
    println!("user = {:?}", result);
}
```

#### Output

```bash
user = "Hari"
```

Here, we have a get_user function that returns an Option type. It can either return Some(&str) or None.

Now, this program can use the unwrap() method to get rid of the match expression which is a little verbose.

Let's use unwrap() in the above example.

### Example: Using unwrap()

