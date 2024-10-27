# [Module](https://www.programiz.com/rust/module)

Modules in Rust help in splitting a program into logical units for better readability and organization.

Once a program gets larger, it is important to split it into multiple files or namespaces. Modules help in structuring our program.

A module is a collection of items: functions, structs and even other modules.

____

## Defining a Module in Rust

The mod keyword is used to define a module. The syntax of module is:

```rust
// syntax of a module
mod module_name {
  // code
}
```

Here, module_name is the name of the module.

Now, let's define a module.

```rust
// a module named config
mod config {
    // a function print inside of the module 
    fn print() {
        println!("config!");
    }
}
```

____


### Aside: Note:

#### Compiler Messages

##### Compiler error

```bash
error[E0601]: `main` function not found in crate `main`
 --> main.rs:7:2
  |
7 | }
  |  ^ consider adding a `main` function to `main.rs`

error: aborting due to 1 previous error

For more information about this error, try `rustc --explain E0601`.
```

____


In the above example, we create a module named config using the mod keyword.

Inside the module we can define multiple items. Here, we have defined the print() function.

____

## Visibility of Items inside a Module in Rust

Items inside a module can be private or public. By default, a module is private. It means items inside the module cannot be accessed outside of the module.

The pub keyword can be used to give an item public visibility.

Let's look at an example.

```rust
mod config {
    // items in modules by default have private visibility
    fn select() {
        println!("called config::select");
    }

    // use the `pub` keyword to override private visibility
    pub fn print() {
        println!("called config::print");
    }
}
```

____

### Debug

Add a stock main function to avoid compiler error no main function defined [E601].

#### Compiler

warning: function `select` is never used
 --> main.rs:3:7
  |
3 |    fn select() {
  |       ^^^^^^
  |
  = note: `#[warn(dead_code)]` on by default

warning: function `print` is never used
 --> main.rs:8:11
  |
8 |    pub fn print() {
  |           ^^^^^

warning: 2 warnings emitted



#### Output

```bash
Hello world!
```

____

Here, we define a module named config with two functions select() and print().

The print() function starts with the pub keyword which means it has public visibility. However, the select() function does not.

If we compile the above program, we don't get any output because we have not used the functions yet.

#### Repeat Compiler error

```rust
warning: function `select` is never used
 --> main.rs:3:7
  |
3 |    fn select() {
  |       ^^^^^^
  |
  = note: `#[warn(dead_code)]` on by default

warning: function `print` is never used
 --> main.rs:8:11
  |
8 |    pub fn print() {
  |           ^^^^^
```

Now, let's call the functions inside the module.

```rust
mod config {
    // items in modules by default have private visibility
    fn select() {
        println!("called config::select");
    }

    // use the `pub` keyword to override private visibility
    pub fn print() {
        println!("called config::print");
    }
}

fn main() {
    // public items inside module can be accessed outside the parent module
    // call public print function from display module
    config::print();
}
```

#### Updated compiler message

```rust
warning: function `select` is never used
 --> main.rs:3:7
  |
3 |    fn select() {
  |       ^^^^^^
  |
  = note: `#[warn(dead_code)]` on by default

warning: 1 warning emitted
```

#### Updated Output

```rust
called config::print
```
