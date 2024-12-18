# Cargo

Cargo is the Rust package manager. It comes pre-installed with Rust and can be used to package dependencies, manage them as well as build and distribute our own packages/libraries.

____

## Features of Cargo in Rust

Cargo is a command line tool for Rust which comes with these features:

- Dependency management

Cargo makes it easy to manage the dependencies of our project, including adding, updating, and removing dependencies.

- Building and packaging

Cargo can automatically build and package our Rust projects, creating binary or library code that can be shared with others.

- Document generation

Cargo can automatically generate documentation for our code, making it easier for other developers to understand and use our library.

- Download crates

Cargo can download and install libraries from [crates.io](https://crates.io/), which is a central repository for Rust crates.

- Run a binary or tests

Cargo can build our source code, run the executable binary and also run our tests.

____

### Dependency Management with Cargo in Rust

One of the primary features of cargo is that it can download, manage external libraries.

Let's look at an example of how we can use an external crate from [crates.io](https://crates.io/). **crates.io** is a central repository where we can pull and publish shared libraries for Rust.

Start by creating a Rust project using cargo:

```bash
cargo new hello_world
```

Now,

`Cargo.toml` file in the root of our project directory `hello_world` is used to manage the dependencies.

If we want to use the "[rand](https://crates.io/crates/rand)" crate, we add the following line to the `[dependencies]` section of the `Cargo.toml`.

**Note:** We can also add dependency to our project using the command cargo add rand.

- Next, we'll need to import the crate in our `src/main.rs` Rust file. We can do this by using `extern crate <crate_name>` line at the top of the file.

```bash
extern crate rand;
```

Now, we can use the "rand" crate to generate a random number between **1** and **6**. The `use` keyword is used here to import items (such as functions, types, and constants) from the "rand" crate in the current scope.

```bash
extern crate rand;
```

```rust
use rand::Rng;

fn main() {
    let mut rng = rand::thread_rng();

    // simulate rolling a die
    println!("roll = {}", rng.gen_range(1..=6));
}

# Output: roll = 5
```
____

### Building and Running Project with Cargo in Rust

We can use cargo to install, build and run our hello_world project with the "rand" crate. Here's how:

**Build the project**

```bash
cargo build
```

#### Output

![Image of Output of building crate](../static/images/A23-image.png)

The `cargo build` command first installs any crates that are in use inside the `src/` directory and then proceeds to compile the project.

____

### Run the project

```bash
cargo run
```

#### Output

![Screenshot of Output](../static/images/A22-image-1.png)

Hello, world!

____

### Useful Cargo Commands in Rust

Cargo can do a bunch of repetitive tasks for us. Here are some of the commonly used cargo commands.

|Command    |	Description           |
|-----------|-------------------------|
|cargo new  |	Create a new Rust project with basic directory structure |
|cargo build|	Build (compile) the current project and generate a binary executable |
|cargo run	|Build and run your current project (cargo build + run) |
|cargo check|	Build the current project without generating a binary executable |
|cargo add|	Add a new dependency and include it in Cargo.toml file |
|cargo update|	Update all dependencies of current project to latest version |
