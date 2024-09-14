fn main() {
    // declare a variable with value 1
    let x = 1;
    println!("x = {}", x);

    // change the value of variable x
    x = 2;
    println!("x = {}", x);
}

/*
error[E0384]: cannot assign twice to immutable variable `x`
 --> main.rs:7:5
  |
3 |     let x = 1;
  |         -
  |         |
  |         first assignment to `x`
  |         help: consider making this binding mutable: `mut x`
...
7 |     x = 2;
  |     ^^^^^ cannot assign twice to immutable variable

error: aborting due to 1 previous error

For more information about this error, try `rustc --explain E0384`.
*/