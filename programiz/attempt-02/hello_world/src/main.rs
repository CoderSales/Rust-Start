use std::fs::OpenOptions;
use std::io::Write;

fn main() {
    // Open a file with append option
    let mut data_file = OpenOptions::new()
        .append(true)
        .open("data.txt")
        .expect("cannot open file");

    // Write to a file
    data_file
        .write("I am learning Rust!".as_bytes())
        .expect("write failed");

    println!("Appended content to a file");
}