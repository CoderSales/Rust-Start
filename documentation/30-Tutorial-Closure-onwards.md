# Tutorial Closure onwards

(Back to [29-Tutorial-Functions-onwards.md](/documentation/29-Tutorial-Functions-onwards.md) )

## [Rust Closure](https://www.programiz.com/rust/closure)

In Rust, closures are **functions without names**. They are also known as **anonymous functions** or **lambdas**.


### Defining a Closure in Rust

Here's how we create a closure in Rust,

```rust
// define a closure to print a text
let print_text = || println!("Defining Closure");
```

In the above example, we have created a closure that prints the text "**Defining Closure**". Here,

- `print_text` - variable to store the closure

- `||` - start of a closure

- `println!("Defining Closure")` - body of the closure

### Calling Closure

Once a closure is defined, we need to call it just like calling a function. To call a closure, we use the variable name to which the closure is assigned. For example,

```rust
// define a closure to print a text
let print_text = || println!("Defining Closure");

// call the closure
print_text();
```

Here, `print_text()` calls the closure.

____

### Example: Closure in Rust

```rust
fn main() {
    // closure that prints a text
    let print_text = || println!("Hello, World!");
    
    print_text(); 
}
```

#### Output

```bash
Hello, World!
```

In the above example, we have defined a closure and stored it in the `print_text` variable. We then call the closure using `print_text()`.

____

### Rust Closure with Parameters

In Rust, we can also pass parameters to a closure. For example,

```rust
// define closure to add 1 to an integer
let add_one = |x: i32| x + 1;
```

Here,

- `let add_one` - is the name of the variable to store the closure

- `|x: i32|` - is the parameter and its type that we pass to the closure

- `x + 1;` - is the body of the closure which returns `x + 1`

If we create a closure with parameters, we need to also pass the value while calling the closure.

```rust
// call the closure with value 2
add_one(2);
```

### Example: Rust Closure with Parameter

```rust
fn main() {
    // define a closure and store it in a variable
    let add_one = |x: i32| x + 1;
    
    // call closure and store the result in a variable
    let result = add_one(2);
    
    println!("Result = {}", result);
}
```

#### Output

```bash
Result = 3
```

In the above example, we have defined a closure and binded it to the `add_one` variable. We then call the closure with `add_one(2)` and bind the return value to the `result` variable.

Here's how the program works,

____

![closureWithParameter.png](/static/images/closureWithParameter.png)

Working of closure with parameter in Rust

____

### Multi-line Closure in Rust

We can also include multiple statements inside a closure. In this case, we enclose those statements using curly braces `{}`.

Let's look at an example.

```rust
fn main() {
    // define a multi-line closure
    let squared_sum = |x: i32, y: i32| {
    
        // find the sum of two parameters
        let mut sum: i32 = x + y;
        
        // find the squared value of the sum
        let mut result: i32 = sum * sum;
        
        return result;
    };
    
    // call the closure
    let result = squared_sum(5, 3);
    
    println!("Result = {}", result);
}
```

```bash
Result = 64
```

In the above example, we have created a closure that takes two parameters: `x` and `y`. Inside the closure, we add `x` and `y` and assign the result to the `sum` variable.

Finally, we have computed the square of sum and returned the `result`.

Here, code inside the opening and closing curly braces, `{}` denotes the body of the closure.

____

### Closure Environment Capturing in Rust

Closure has a unique feature that allows it to capture the environment. This means the closure can use the values in its scope. For example,

```rust
fn main() {
    let num = 100;
    
    // A closure that captures the num variable
    let print_num = || println!("Number = {}", num);
    
    print_num(); 
}
```

#### Output

```bash
Number = 100
```

Here, the closure bound to `print_num` uses the variable `num` which was not defined in it. This is known as closure environment capturing.

____

### Closure Environment Capturing Modes in Rust

Environment capturing of closures can be of 3 different modes based on the variable and the closure definition.

1. Variable is not modified inside closure

2. Variable is modified inside closure

3. Variable is moved inside closure

Let's look at each of these modes of environment capturing.

#### 1. Variable is not modified inside closure

```rust
fn main() {
    let word = String::from("Hello");
    
    // immutable closure
    let print_str = || {
        println!("word = {}", word);
    };

    // immutable borrow is possible outside the closure
    println!("length of word = {}", word.len());
    
    print_str();
}
```

Here, the variable `word` is not modified inside the closure `print_str`. As the variable is immutable by default, we can make any number of immutable references of `word` inside the closure. Notice that the closure variable `print_str` is also immutable.

This mode of capture is also known as **Capture by Immutable Borrow**.

### 2. Variable is modified inside closure

```rust
fn main() {
    let mut word = String::from("Hello");
    
    // mutable closure
    let mut print_str = || {
        // value of word is changed here
        word.push_str(" World!");
        println!("word = {}", word);
    };
     
     // cannot immutable borrow because the variable is borrowed as mutable inside the closure
     // println!("length of word = {}", word.len());
    
    print_str();

    // can immutable borrow because the closure has been already used
    println!("length of word = {}", word.len());
}
```

#### Output

```bash
word = Hello World!
length of word = 12
```

Here, the variable `word` is modified inside the closure `print_str` with `word.push_str("World!");`. Thus, we have to make the variable `word` mutable as well as the closure variable `print_str`. This means no other references of the `word` variable can exist unless the closure is used.

This mode of capture is also known as **Capture by Mutable Borrow**.

### 3. Variable is moved inside closure

```rust
fn main() {
    let word = String::from("Hello");

    // immutable closure
    let print_str = || {
        // word variable is moved to a new variable
        let new_word = word;
        println!("word = {}", new_word);
    };

    print_str();

    // cannot immutable borrow because word variable has moved inside closure
    // println!("length of word = {}", word.len());
}
```

#### Output

```bash
word = Hello
```

Here, we move the variable `word` to a new variable `new_word` inside the closure. As the variable is moved, we cannot use it anywhere else except for inside the closure.

This mode of capture is also known as **Capture by Move**.

____

## [Rust Stack and Heap](https://www.programiz.com/rust/stack-and-heap)

Stack and Heap are parts of memory available to our Rust code to use at runtime.

Rust is a memory-safe programming language. To ensure that Rust is memory-safe, it introduces concepts like ownership, references and borrowing.

To understand these concepts, we must first understand how to allocate and deallocate memory into the Stack and Heap.

____

### The Stack

The Stack can be thought of as a stack of books. When we add more books, we add them on the top of the pile. When we need a book, we take one from the top.

The stack inserts values in order. It gets them and removes the values in the opposite order.

- Adding data is called **pushing onto the stack**

- Removing data is called **popping off the stack**

This phenomenon is called **Last In, First Out (LIFO)** in programming.

Data stored on the stack must have a fixed size during compile time. Rust, by default, allocates memory on the stack for primitive types.

Let's visualize how memory is allocated and deallocated on the stack with an example.

```rust
fn foo() {
    let y = 999;
    let z = 333;
}

fn main() {
    let x = 111;
    
    foo();
}
```

In the above example, we first call the function `main()`. The `main()` function has one variable binding `x`.

When `main()` executes, we allocate a single 32-bit integer (`x`) to the stack frame.

| Address | Name | Value |
|---------|------|-------|
| 0       | x    | 111   |

In the table, the **"Address"** column refers to the memory address of the RAM.

It starts from **0** and goes to how much RAM (number of bytes) your computer has. The **"Name"** column refers to the variable, and the **"Value"** column refers to the variable's value.

When `foo()` is called a new stack frame is allocated. The `foo()` function has two variable bindings, `y` and `z`.

| Address | Name | Value |
|---------|------|-------|
| 2       | z    | 333   |
| 1       | y    | 999   |
| 0       | x    | 111   |

The numbers **0**, **1**, and **2** do not use address values the computer will use in reality. In reality, the addresses are separated by some number of bytes based on the value.

After `foo()` is completed, its stack frame is deallocated.

| Address | Name | Value |
|---------|------|-------|
| 0       | x    | 111   |

Finally, `main()` is completed, and everything goes away.

Rust automatically does allocation and deallocation of memory in and out of the stack.

____

### The Heap

As opposed to the stack, most of the time, we will need to pass variables (memory) to different functions and keep them alive for longer than a single function's execution. This is when we can use the heap.

We can allocate memory on the heap using the `Box<T>` type. For example,

```rust
fn main() {
    let x = Box::new(100);
    let y = 222;
    
    println!("x = {}, y = {}", x, y);
}
```

#### Output

```bash
x = 100, y = 222
```

Let's visualize the memory when main() is called in the above example.

| Address | Name | Value |
|---------|------|-------|
| 1       | y    | 222   |
| 0       | x    | ???   |

Like before, we allocate two variables, `x` and `y`, on the stack.

However, the value of `x` is allocated on the heap when `Box::new()` is called. Thus, the actual value of `x` is a pointer to the heap.

The memory now looks like this:

| Address | Name | Value    |
|---------|------|----------|
| 5678    |      | 100      |
| ...     | ...  | ...      |
| 1       | y    | 222      |
| 0       | x    | -> 5678  |

Here, the variable `x` holds a pointer to the address **→ 5678**, an arbitrary address used for demonstration. Heap can be allocated and freed in any order. Thus it can end up with different addresses and create holes between addresses.

So when `x` goes away, it first frees the memory allocated on the heap.

| Address | Name | Value    |
|---------|------|----------|
| 1       | y    | 222      |
| 0       | x    | ???      |

Once the `main()` is completed, we free the stack frame, and everything goes away, freeing all the memory.

We can make the memory live longer by transferring ownership where the heap can stay alive longer than the function which allocates the `Box`. To learn more about ownership, visit [Rust Ownership](https://www.programiz.com/rust/ownership).

____

### Differences between Stack and Heap

| Stack                                  |   	Heap                                 |
|----------------------------------------|-------------------------------------------|
| Accessing data in the stack is faster. | Accessing data in a heap is slower.       |
|                                        |                                           |
| Managing memory in the stack is        | Managing memory for the heap              |
| predictable and trivial.	             | (arbitrary size) is non-trivial.          |
|                                        |                                           |
| Rust stack allocates by default.	     | Box is used to allocate to the heap.      |
|                                        |                                           |
| Primitive types and local variables    | Data types that are dynamic in size,      |
| of a function are allocated            | such as String, Vector, Box, etc.,        |
| on the stack.	                         | are allocated on the heap.                |
|                                        |                                           |

[31-Tutorial-Vector-onwards.md](/documentation/31-Tutorial-Vector-onwards.md)
