fn main() {
    let mut str = String::from("Hello");
    
    // before modifying the string
    println!("Before: str = {}", str);

    // pass a mutable string when calling the function
    change(&mut str);
    
    // after modifying the string
    println!("After: str = {}", str);
}

fn change(s: &mut String) {
    // push a string to the mutable reference variable
    s.push_str(", World!");
}

/*
url:
https://www.programiz.com/rust/references-and-borrowing

Page Title:
Rust References and Borrowing

Section Title:


Subsection Title:


### Documentation

40-Tutorail-References-and-Borrowing.md


### Compiler Messages


### Output

Before: str = Hello
After: str = Hello, World!

### Notes:


*/
