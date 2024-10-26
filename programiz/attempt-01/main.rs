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

/*
url:
https://www.programiz.com/rust/unwrap-and-expect

Page Title:
Rust unwrap() and expect()

Section Title:
The expect() Method

Subsection Title:


### Documentation

38-unwrap-and-expect.md


### Compiler Messages


### Output

thread 'main' panicked at main.rs:12:31:
fetch user
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

### Notes:

*/
