fn main() {
    enum Color {
        Red,
        Green,
        Blue,
    }

    let my_color = Color::Green;

    // use of match expression to match against an enum variant
    match my_color {
        Color::Red => println!("The color is red"),
        Color::Green => println!("The color is green"),
        Color::Blue => println!("The color is blue"),
    }
}

/*
url:
https://www.programiz.com/rust/pattern-matching

Page Title:
Rust Pattern Matching


Section Title:
Matching Option and Result Type in Rust

Subsection Title:


Compiler Notes:

warning: 
variants `Red` and `Blue` 
are never constructed

Output:

The color is green

*/
