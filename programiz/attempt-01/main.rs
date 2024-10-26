use std::fs::File;

fn main() {
    let data_result = File::open("data.txt");

    // using match for Result type
    let data_file = match data_result {
        Ok(file) => file,
        Err(error) => panic!("Problem opening the data file: {:?}", error),
    };

    println!("Data file", data_file);
}

/*
url:
https://www.programiz.com/rust/error-handling

Page Title:
Rust Error Handling

Section Title:
Recoverable Errors

Subsection Title:

### Documentation

36-Tutorial-Iterators-onwards.md



### at Compile time



error: argument never used
--> main.rs:12:27
|
12 |     println!("Data file", data_file);
|              -----------  ^^^^^^^^^ argument never used
|              |
|              formatting specifier missing

error: aborting due to 1 previous error



### Output


Hello, World!
thread 'main' panicked at main.rs:5:5:
Crash
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace




Notes:

*/
