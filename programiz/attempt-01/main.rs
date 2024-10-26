use std::num::ParseIntError;

// Function to parse an integer
fn parse_int() -> Result<i32, ParseIntError> {
    // Example of ? where value is unwrapped
    let x: i32 = "12".parse()?; // x = 12
    
    // Example of ? where error is returned
    let y: i32 = "12a".parse()?; // returns an Err() immediately
    
    Ok(x + y) // Doesn't reach this line
}

fn main() {
    let res = parse_int();

    println!("{:?}", res);
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

Err(ParseIntError { kind: InvalidDigit })

### Notes:

*/
