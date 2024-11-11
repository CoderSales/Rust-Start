# [File Handling](https://www.programiz.com/rust/file-handling)

In computer programming, file handling is a way to deal with data in a file. File handling enables us to open, read, write, create and update files on the local system.

File handling is commonly performed by many applications including databases, web servers. It is an important example of how I/O (Input/Output) operations work.

File handling is also generally known as File I/O.

____

## File Struct in Rust

In Rust, the `std::fs::File` struct represents a file. It allows us to perform read/write operations on a file.

The file I/O is performed through the `std::fs` module which provides functions for working with the file system.

All methods in the `File` struct return a variant of the `std:io::Result` or simply the `Result` enum.

Let's look at the basics of file I/O in Rust with these operations:

- Opening a file

- Reading from a file

- Writing to a file

- Removing a file

- Appending to a file

____

### Opening a File in Rust

To open a file in Rust, we use the `File::open()` method. This method takes a file path as an argument and returns a `File` object. If the file does not exist, it returns an error (`Err`).

Let's look at an example.

```rust
use std::fs::File;

fn main() {
    // Open a file in read only mode in the local file system
    let data_result = File::open("data.txt");

    // Reading a file returns a Result enum
    // Result can be a file or an error
    let data_file = match data_result {
        Ok(file) => file,
        Err(error) => panic!("Problem opening the data file: {:?}", error),
    };

    println!("Data file: {:?}", data_file);
}
```

```bash
cargo build
```

```bash
cargo run
```

```bash
Data file: File { handle: 0xc4, path: "...data.txt" }
```
