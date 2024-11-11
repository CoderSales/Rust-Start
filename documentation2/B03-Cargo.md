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


____
