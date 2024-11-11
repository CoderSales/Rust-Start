use std::thread;
use std::time::Duration;

fn main() {
    // create a thread and save the handle to a variable
    let handle = thread::spawn(|| {
        // everything in here runs in a separate thread
        for i in 0..10 {
            println!("{} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(2));
        }
    });

    // main thread
    for i in 0..5 {
        println!("{} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }
    
    // wait for the separate thread to complete
    handle.join().unwrap();
}