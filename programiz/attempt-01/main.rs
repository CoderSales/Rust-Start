use std::collections::HashSet;

fn main() {
    let hashset1 = HashSet::from([1, 2, 3, 4]);
    let hashset2 = HashSet::from([4, 3, 2]);
    
    // Difference between hashsets
    let result: HashSet<_> = hashset1.difference(&hashset2).collect();
    
    println!("hashset1 = {:?}", hashset1);
    println!("hashset2 = {:?}", hashset2);
    println!("difference = {:?}", result);
}

/*
url:
https://www.programiz.com/rust/hashset

Page Title:
Rust HashSet

Section Title:
Set Operations

Subsection Title:
3. Difference between two Sets

### Documentation

35-Tutorial-Hashset-continued-onwards.md

Output:
hashset1 = {2, 7, 8}
hashset2 = {1, 2, 7}
intersection = {2, 7}

Notes:
Order of values
in Output 
vary
but
the actual
values
themselves 
are the same

*/
