// A macro which uses repetitions
macro_rules! repeat_print {
    // match rule which matches multiple expressions in an argument
    ($($x:expr),*) => {
        $(
            println!("{}", $x);
        )*
    };
}

fn main() {
    // Call the macro with multiple arguments
    repeat_print!(1, 2, 3);
}