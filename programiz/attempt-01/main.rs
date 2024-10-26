// function to find a user by their username which return an Option enum
fn get_user(username: &str) -> Option<&str> {
    if username.is_empty() {
        return None;
    }

    return Some(username);
}

fn main() {
    // use of unwrap method to get the result of Option enum from get_user function
    let result = get_user("").unwrap();

    // print the result
    println!("user = {:?}", result);
}

/*
url:
https://www.programiz.com/rust/unwrap-and-expect

Page Title:
Rust unwrap() and expect()

Section Title:
The unwrap() Method

Subsection Title:
Example: Using the match Expression


### Documentation

38-unwrap-and-expect.md


### Compiler Messages


### Output

thread 'main' panicked at main.rs:12:31:
called `Option::unwrap()` on a `None` value
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace



### Notes:

*/
