# [Thread](https://www.programiz.com/rust/thread)

A thread is the smallest executable unit of a process.

Threads allow us to split the computation in our program into multiple threads. Running multiple tasks at the same time can improve performance of the code. However, it can add complexity.

____

## Creating a New Thread in Rust

In Rust, we can create a native operating system thread using the `thread::spawn()` function from the `std` module. The spawn method takes a closure as an argument.

Here is the syntax of `thread::spawn()`,

```rust
thread::spawn(|| {
    // code to execute in the thread 
})
```

Now, let's see an example.

```rust
use std::thread;
use std::time::Duration;

fn main() {
    // create a thread
    thread::spawn(|| {
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
}
```

```bash
cargo build
```

```bash
cargo run
```

### Output

```bash
0 from the main thread!
0 from the spawned thread!
1 from the main thread!
1 from the spawned thread!
2 from the main thread!
3 from the main thread!
2 from the spawned thread!
4 from the main thread!
3 from the spawned thread!
```
