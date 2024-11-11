// A simple macro named `hello_world`
macro_rules! hello_world {
    // `()` indicates that the macro takes no argument
    () => {
        // The macro will expand into the contents of this block
        println!("Hello, World!")
    };
}

fn main() {
    // Call the hello_world macro
    // This call will expand into `println!("Hello, World!");`
    hello_world!()
}