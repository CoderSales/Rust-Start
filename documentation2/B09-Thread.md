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

In the example above, we create a thread using the `thread::spawn()` function. The thread loops over `0..5` and prints the current value.

Similarly, we have a main thread where we loop over `0..5` and print the current value.

We also call `thread::sleep` to force a thread to stop its execution for a short duration, allowing a different thread to run.

Notice that we sleep **2** milliseconds in the spawned thread and 1 millisecond in the main thread.

The output from this program might be a little different every time. The important thing to remember here is that if the main thread completes, all other threads are shut down whether or not they have finished running.

So, even though the spawned thread should print until `i` is 9, it only reaches to **2** because the main thread shut down.

____

### Join Handles in Rust

A spawned thread always returns a join handle. If we want the spawned thread to complete execution, we can save the return value of `thread::spawn` in a variable and then call the `join()` method on it.

The `join()` method on `JoinHandle` (return type of `thread::spawn`) waits for the spawned thread to finish.

Let's look at an example.

```rust
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
```

```bash
cargo build
```

```bash
cargo run
```

#### Output

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
4 from the spawned thread!
5 from the spawned thread!
6 from the spawned thread!
7 from the spawned thread!
8 from the spawned thread!
9 from the spawned thread!
```

Here, we save the return of the `thread::spawn()` function and bind it to a variable called `handle`.

In the final line of the code, we call the `join()` method of the `handle`. Calling `join()` on the `handle` blocks the thread until the thread terminates.

The two threads (main and spawned thread) continue alternating for some time, but the main thread waits because of `handle.join()` and does not end until the spawned thread is finished.

____

If we move the `handle.join()` before the final loop, the output will change and the print statements won't be interleaved.

____

```rust
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
    
    // wait for the separate thread to complete
    handle.join().unwrap();

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

##### Output of latest code

```bash
0 from the spawned thread!
1 from the spawned thread!
2 from the spawned thread!
3 from the spawned thread!
4 from the spawned thread!
5 from the spawned thread!
6 from the spawned thread!
7 from the spawned thread!
8 from the spawned thread!
9 from the spawned thread!
0 from the main thread!
1 from the main thread!
2 from the main thread!
3 from the main thread!
4 from the main thread!
```
