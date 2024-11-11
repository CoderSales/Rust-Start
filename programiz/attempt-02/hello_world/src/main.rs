// A macro named `print_message`
macro_rules! print_message {
    // Match rule that takes an argument expression
    ($message:expr) => {
        println!("{}", $message)
    };
}

fn main() {
    // Call the macro with an argument
    print_message!("I am learning Rust!");
}