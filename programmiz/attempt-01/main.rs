fn main() {
    // variable to print multiplication table for
    let i = 2;
    
    // counter variable that starts at 1
    let mut j = 1;
    
    // while loop that runs for 10 iterations
    while j <= 10 {
        // multiply i and j
        let mult = i * j;
        
        // print multiplication result on each iteration
        println!("{} * {} = {}", i, j, mult);

        // increase value of counter variable j
        j += 1;
    }
}

 /*
url:
https://www.programiz.com/rust/while-loop

Page Title:
Rust while Loop

Section Title:
Example: Multiplication Table Using while Loop

Output:

2 * 1 = 2
2 * 2 = 4
2 * 3 = 6
2 * 4 = 8
2 * 5 = 10
2 * 6 = 12
2 * 7 = 14
2 * 8 = 16
2 * 9 = 18
2 * 10 = 20

*/
