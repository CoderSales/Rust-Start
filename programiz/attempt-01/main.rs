fn main() {
    let numbers = [2, 1, 17, 99, 34, 56];
    
    // iterator
    let numbers_iterator = numbers.iter();
    
    for number in numbers_iterator {
        println!("{}", number);
    }
}

/*
url:
https://www.programiz.com/rust/iterator

Page Title:
Rust Iterator

Section Title:
Example: Iterator in Rust

Subsection Title:

### Documentation

36-Tutorial-Iterators-onwards.md

Output

2
1
17
99
34
56

Notes:
Order of values
in Output 
vary
but
the actual
values
themselves 
are the same

*/
