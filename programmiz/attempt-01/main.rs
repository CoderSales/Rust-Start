fn main() {
    let boolean1: bool = false;
    let boolean2: bool = true;
  
    // convert boolean type to integer
    let integer1 = boolean1 as i32;
    let integer2 = boolean2 as i32;

    println!("boolean1 = {}", boolean1);
    println!("boolean1 = {}", boolean2);
    println!("integer1 = {}", integer1);
    println!("integer2 = {}", integer2);
}
/*
Type Casting: Boolean to Integer in Rust

Output:

boolean1 = false
boolean1 = true
integer1 = 0
integer2 = 1
*/
