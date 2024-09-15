fn main() {
    let integer: i32 = 65;
  
    // convert integer to char using the as keyword
    let character = integer as char;

    println!("integer = {}" , integer);
    println!("character = {}", character);
}

/*
error[E0604]: only `u8` can be cast as `char`, not `i32`

let character = integer as char;
                ^^^^^^^^^^^^^^^ invalid cast
help: try `char::from_u32` instead (via a `u32`)
For information about error try
rustc --explain E0604
*/