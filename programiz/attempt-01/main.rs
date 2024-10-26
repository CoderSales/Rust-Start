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

user = "Hari"


### Notes:

*/
