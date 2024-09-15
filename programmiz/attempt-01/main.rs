fn main() {
    let mut number = 0;
    
    // loop starts here
    loop {
        number += 1;

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
Rust break


Output:

1
2
3
4
5

Note: 
You can use the break keyword with while or for loops in a similar pattern.

*/
