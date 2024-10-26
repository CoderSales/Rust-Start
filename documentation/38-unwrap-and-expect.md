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

```rust
// function to find a user by their username which return an Option enum
fn get_user(username: &str) -> Option<&str> {
    if username.is_empty() {
        return None;
    }

    return Some(username);
}

fn main() {
    // use of unwrap method to get the result of Option enum from get_user function
    let result = get_user("Hari").unwrap();

    // print the result
    println!("user = {:?}", result);
}
```

#### Output

```bash
user = "Hari"
```

Both the match expression and unwrap() gives us the same output. The only difference being that unwrap() will panic if the return value is a None.

If we update the above program to send an empty username argument to the get_user() method. It will panic.

```rust
let result = get_user("").unwrap();
```

The output in this case will be,

##### Expected Output

```bash
thread 'main' panicked at 'called `Option::unwrap()` on a `None` value', src/main.rs:12:31ßß
```

##### Actual Output

```bash
thread 'main' panicked at main.rs:12:31:
called `Option::unwrap()` on a `None` value
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

____

### The expect() Method

expect() is very similar to unwrap() with the addition of a custom panic message as an argument.

The expect() method is defined on both Option and Result type.

Let's update the above example to use expect() instead of unwrap().

```rust
// function to find a user by their username which return an Option enum
fn get_user(username: &str) -> Option<&str> {
    if username.is_empty() {
        return None;
    }

    return Some(username);
}

fn main() {
    // use of expect method to get the result of Option enum from get_user function
    let result = get_user("").expect("fetch user");

    // print the result
    println!("user = {:?}", result);
}
```

#### Output

##### Expected

```bash
thread 'main' panicked at 'fetch user', src/main.rs:12:31
```

##### Actual

```bash
thread 'main' panicked at main.rs:12:31:
fetch user
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

____

###### Aside: Note

Essentially the same

____

Here, we use the expect() with a panic message as the argument.

expect() and unwrap() will produce the same result if there's no possibility of Option returning None and Result returning Err.

Note: unwrap() and expect() are utility methods to work with Option and Result types. It makes our program concise and prevents the need to write verbose match expressions to return a result.

____

### The Question Mark (?) Operator

The question mark (?) operator is a shorthand for returning the Result. It can only be applied to Result<T, E> and Option<T> type.

When we apply ? to Result<T, E> type:

If the value is Err(e), it returns an Err() immediately
If the value is Ok(x), it unwraps and returns x
Let's look at an example.

```rust
use std::num::ParseIntError;

// Function to parse an integer
fn parse_int() -> Result<i32, ParseIntError> {
    // Example of ? where value is unwrapped
    let x: i32 = "12".parse()?; // x = 12
    
    // Example of ? where error is returned
    let y: i32 = "12a".parse()?; // returns an Err() immediately
    
    Ok(x + y) // Doesn't reach this line
}

fn main() {
    let res = parse_int();

    println!("{:?}", res);
}
```

#### Output

```bash
Err(ParseIntError { kind: InvalidDigit })
```

This way, error handling in the function is reduced to a single line of code, making it cleaner and easier to read.

Similarly, when we apply ? to Option<T> type:

If the value is None, then it returns None
If the value is Some(x), then it unwraps the value and returns x

##### Note

Note: The question mark operator (?) can only be used in a function that returns Result or Option.
