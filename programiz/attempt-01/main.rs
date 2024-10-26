// data_file is a Result<T, E>
match data_result {
    Ok(file) => file,
    Err(error) => panic!("Problem opening the data file: {:?}", error),
};

/*
url:
https://www.programiz.com/rust/error-handling

Page Title:
Rust Error Handling

Section Title:
The Result Enum

Subsection Title:


### Documentation

37-Error-Handling.md


### Compiler Messages

error: expected item, found keyword `match`
 --> main.rs:2:1
  |
2 | match data_result {
  | ^^^^^ expected item
  |
  = note: for a full list of items that can appear in modules, see <https://doc.rust-lang.org/reference/items.html>

error: aborting due to 1 previous error

### Output

### Notes:

*/
