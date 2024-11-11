use std::thread;

fn main() {
    let message = String::from("Hello, World!");

    // using the message variable without a move
    let handle = thread::spawn(|| {
        println!("{}", message);
    });

    handle.join().unwrap();
}