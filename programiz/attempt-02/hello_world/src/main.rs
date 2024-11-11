use std::fs::File;
use std::io::Read;

fn main() {
    // Read a file in the local file system
    let mut data_file = File::open("../../../static/data/data.txt").unwrap();

    // Create an empty mutable string
    let mut file_content = String::new();

    // Copy contents of file to a mutable string
    data_file.read_to_string(&mut file_content).unwrap();

    println!("File content: {:?}", file_content);
}