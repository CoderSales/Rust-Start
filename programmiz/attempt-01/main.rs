fn main() {
    let mut number = 0;

    loop {
        number += 1;

        // condition to skip the iteration
        if number == 3 {
            continue;
        }
        
        // condition to exit the loop
        if number > 5 {
            break;
        }
        
        println!("{}", number);
    }
}

/*
url:
https://www.programiz.com/rust/break-continue

Page Title:
Rust break and continue


Section Title:
Working of continue Keyword in Rust

Subsection Title:
break and continue with loop


Output:

1
2
4
5

*/
