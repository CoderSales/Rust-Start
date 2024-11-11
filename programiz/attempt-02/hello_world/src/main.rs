use std::fs::File;

fn main() {
    // Open a file in read only mode in the local file system
    let data_result = File::open("../../../static/data/data.txt");

    // Reading a file returns a Result enum
    // Result can be a file or an error
    let data_file = match data_result {
        Ok(file) => file,
        Err(error) => panic!("Problem opening the data file: {:?}", error),
    };

    println!("Data file: {:?}", data_file);
}