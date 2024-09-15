fn main() {
    let decimal: f32 = 65.321;
  
    // convert float to char data type
    let character = decimal as char;

    println!("decimal = {}", decimal);
    println!("character = {}", character);
}
/*
Limitations of Type Casting

error[E0604]: only `u8` can be cast as `char`, not `f32`
*/