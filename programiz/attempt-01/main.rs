use std::collections::HashSet;

fn main() {
    let hashset1 = HashSet::from([2, 7, 8]);
    let hashset2 = HashSet::from([1, 2, 7, 9]);
    
    // Symmetric difference of hashsets
    let result: HashSet<_> = hashset1.symmetric_difference(&hashset2).collect();
    
    println!("hashset1 = {:?}", hashset1);
    println!("hashset2 = {:?}", hashset2);
    println!("symmetric difference = {:?}", result);
}

/*
url:
https://www.programiz.com/rust/hashset

Page Title:
Rust HashSet

Section Title:
Set Operations

Subsection Title:
4. Symmetric Difference between two Sets

### Documentation

35-Tutorial-Hashset-continued-onwards.md

hashset1 = {7, 2, 8}
hashset2 = {1, 2, 7, 9}
symmetric difference = {1, 8, 9}

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
