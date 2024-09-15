fn main() {
    let mut i = 1;
    
    // start of outer loop
    while i <= 5 {
        let mut j = 1;
     
        // start of inner loop
        while j <= 5 { // hence 5 lines
            print!("*");
            
            // condition to exit the inner loop
            if j == 3 {
                // terminate the inner loop
                break; // if j is 3
                // hence 3 asterisks per line
            }
            
            j += 1;
        }
        
        println!("");
        
        i += 1;
    }
}

/*
url:
https://www.programiz.com/rust/break-continue

Page Title:
Rust break and continue


Section Title:
Working of break Keyword in Rust

Subsection Title:
Rust break with Nested Loops


Output:

***
***
***
***
***

Note: 
used the break keyword in the body of the inner while loop.
on line 15

*/
