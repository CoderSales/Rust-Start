fn main() {
    let mut number = 0;

    while number < 5 {
        number += 1;

        // condition to skip the iteration
        if number == 3 {
            continue;
        }
        // continue if
        // number is 3

        println!("{}", number);
    }
}

/*
url:
https://www.programiz.com/rust/break-continue

Page Title:
Rust break and continue


Section Title:
Rust continue

Subsection Title:
Example: Rust continue


Output:

1
2
4
5



*/
