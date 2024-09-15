fn main() {
    let mut number = 0;
    
    // infinite loop starts here
    loop {
        number += 1;
        println!("{}", number);
        
        if number >= 10 {
            // exit the loop
            break;
        }
    }
}

 /*
url:
https://www.programiz.com/rust/loop

Page Title:
Rust loop

Section Title:
Example: Print First 10 Natural Numbers using Loop

Output:

1
2
3
4
5
6
7
8
9
10

*/
