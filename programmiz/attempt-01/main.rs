fn main() {
    let x: u32 = -200;

    println!("x = {}", x);
}

/*
error[E0600]: cannot apply unary operator `-` to type `u32`
 --> main.rs:2:18
  |
2 |     let x: u32 = -200;
  |                  ^^^^ cannot apply unary operator `-`
  |
  = note: unsigned values cannot be negated

error: aborting due to 1 previous error

For more information about this error, try `rustc --explain E0600`.
*/