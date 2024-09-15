fn main() {
    // outer loop counter
    let mut i = 1;
    
    // outer loop
    while i <= 5 {
        // inner loop counter
        let mut j = 1;
        
        // inner loop
        while j <= 5 {
            print!("*");
            
            // increase inner loop counter
            j += 1;
        }
        
        println!("");
        
        // increase outer loop counter
        i += 1;
    }
}

/*
url:
https://www.programiz.com/rust/while-loop

Page Title:
Rust while Loop

Section Title:
Nested while Loop

Output:

*****
*****
*****
*****
*****

Notes:

j does the 5 *'s per line
i repeats the line of j asterisks over the 5 lines

i is outer
j is inner

*/
