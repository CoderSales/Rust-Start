fn main() {
    let mut i = 1;
    
    // start of outer loop
    while i <= 5 { //  Note1: 5 lines
        let mut j = 1;
            
        // start of inner loop
        while j <= 5 { // Note2: would be 5 asterisks
            j += 1;
            
            // condition to skip iteration of the inner loop
            if j == 3 { // Note3: but skip 3 so 4 asterisks
                // move to the next iteration of the inner loop
                continue;
            }
            
            print!("*");
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
Working of continue Keyword in Rust

Subsection Title:
Rust continue with Nested Loops


Output:

****
****
****
****
****


*/
