use std::thread;

fn main() {
    // main thread starts here
    let message = String::from("Hello, World!");

    // move the message value to a separate thread
    let handle = thread::spawn(move || {
        println!("{}", message);
    });

    // wait for the thread to finish
    handle.join().unwrap();
}
