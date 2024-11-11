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

Here, we import the module `std::fs::File` on the top of the program to use the file I/O functions.

To open a file, we call `File::open("data.txt")` which reads the `data.txt` file in the local file system.

The `open()` function returns a `Result` enum which will return the `File` object or an `Err`.

Then, we pattern match the `data_result` variable and `panic!` if there is an error with opening the file. If opening the file doesn't error, we output the `File` object.

____

#### Reading a File in Rust

To read a file in Rust, we use the `read_to_string()` method of the `std::io:Read` trait. This method reads all bytes until end of file (EOF) and copies it to a mutable string.

Here's an example.

```rust
use std::fs::File;
use std::io::Read;

fn main() {
    // Read a file in the local file system
    let mut data_file = File::open("data.txt").unwrap();

    // Create an empty mutable string
    let mut file_content = String::new();

    // Copy contents of file to a mutable string
    data_file.read_to_string(&mut file_content).unwrap();

    println!("File content: {:?}", file_content);
}
```

```bash
cargo build
```

```bash
cargo run
```

##### Output

```bash
File content: "The quick brown fox jumps over the lazy dog.\\n"
```

Here, we import two modules: `std::fs::File` and `std::io::Read` for reading a file. We first open the file `data.txt` with `File::open("data.txt") `method call and bind its result to a variable `data_file`.

Once we open the file, we use the `read_to_string()` method which takes an empty mutable string `file_content` as an argument and copies the content of the file `data.txt` to `file_content`.

____

###### Note:

We use `unwrap()` twice to get the result from the method calls. unwrap() is a utility method to work with `Option` and `Result` type. To learn more, visit [Rust unwrap() and expect()](https://www.programiz.com/rust/unwrap-and-expect).
`read_to_string()` comes from the `std::io::Read` trait. To learn more, visit [Rust Trait](https://www.programiz.com/rust/trait).

____

### Writing to a File in Rust

To write to a file in Rust, we can use the `write()` method from the `std::io:Write` trait. This method writes contents to a file.

Let's look at an example.

```rust
use std::fs::File;
use std::io::Write;

fn main() {
    // Create a file
    let mut data_file = File::create("data.txt").expect("creation failed");

    // Write contents to the file
    data_file.write("Hello, World!".as_bytes()).expect("write failed");

    println!("Created a file data.txt");
}
```

```bash
cargo build
```

```bash
cargo run
```

#### Output

```bash
Created a file data.txt
```

Here, we import `std::fs::File` and `std::io::Write` modules for writing to a file. We first create a file `data.txt` with the `File::create("data.txt")` method and bind it to a mutable variable `data_file`.

After we create a file, we write to the file using the `write()` method with the content `"Hello, World!"`.

____

### Removing a File in Rust

To remove or delete a file in Rust, we can use the remove_file() method from the std::fs module.

For example,

```rust
use std::fs;

fn main() {
    // Remove a file
    fs::remove_file("data.txt").expect("could not remove file");
    
    println!("Removed file data.txt");
}
```

```bash
cargo build
```

```bash
cargo run
```

#### Output

```bash
Removed file data.txt
```
