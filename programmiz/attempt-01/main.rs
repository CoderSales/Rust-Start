fn main() {
    // declare a constant
    const PI:f32 = 3.14;
    println!("Initial Value of PI: {}", PI);

    // change value of PI
    PI = 535.23;
    println!("Update Value of PI: {}", PI);
}

/*
error[E0070]: invalid left-hand side of assignment
 --> main.rs:7:8
  |
7 |     PI = 535.23;
  |     -- ^
  |     |
  |     cannot assign to this expression

error: aborting due to 1 previous error

For more information about this error, try `rustc --explain E0070`.
*/