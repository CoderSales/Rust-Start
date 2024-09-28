use std::collections::HashSet;

fn main() {
    let hashset1 = HashSet::from([2, 7, 8]);
    let hashset2 = HashSet::from([1, 2, 7]);
    
    // Union of hashsets
    let result: HashSet<_> = hashset1.union(&hashset2).collect();
    
    println!("hashset1 = {:?}", hashset1);
    println!("hashset2 = {:?}", hashset2);
    println!("union = {:?}", result);
}

/*
url:
https://www.programiz.com/rust/hashset

Page Title:
Rust HashSet

Section Title:
Set Operations

Subsection Title:
1. Union of two Sets

### Documentation

34-Tutorial-HashSet-onwards.md

Output:
hashset1 = {7, 8, 2}
hashset2 = {1, 2, 7}
union = {8, 2, 1, 7}

Notes:

*/
