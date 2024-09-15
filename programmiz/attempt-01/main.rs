fn main() {
    let a = true;
    let b = false;
    
    // logical AND operation
    let c = a && b;

    // logical OR operation
    let d = a || b;

    // logical NOT operation
    let e = !a;
    
    println!("{} && {} = {}", a, b, c);
    println!("{} || {} = {}", a, b, d);
    println!("!{} = {}", a, e);
}
/*
Example: Logical Operators

Output:

true && false = false
true || false = true
!true = false

*/
