fn main() {
    let mut str = String::from("hello");

    // mutable reference 1
    let ref1 = &mut str;

    // mutable reference 2
    let ref2 = &mut str;

    println!("{}, {}", ref1, ref2);
}

/*
url:
https://www.programiz.com/rust/references-and-borrowing

Page Title:
Rust References and Borrowing

Section Title:
Modifying a Reference in Rust Part 2


Subsection Title:


### Documentation

40-Tutorail-References-and-Borrowing.md


### Compiler Messages

#### Compiler Error [E0499]

error[E0499]: cannot borrow `str` as mutable more than once at a time
  --> main.rs:8:16
   |
5  |     let ref1 = &mut str;
   |                -------- first mutable borrow occurs here
...
8  |     let ref2 = &mut str;
   |                ^^^^^^^^ second mutable borrow occurs here
9  |
10 |     println!("{}, {}", ref1, ref2);
   |                        ---- first borrow later used here

error: aborting due to 1 previous error

For more information about this error, try `rustc --explain E0499`.



### Output


### Notes:


*/
