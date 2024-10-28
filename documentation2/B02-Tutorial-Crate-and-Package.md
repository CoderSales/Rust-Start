# [Rust Crate and Package](https://www.programiz.com/rust/crate-and-package)

A crate can contain one or more Rust modules, which in turn can contain code, such as functions, types, and constants.

A crate is of two types:

Binary crate
Library crate
A binary crate is a Rust program that compiles to an executable or multiple executables and has a main() function for each executable.

A library crate doesn't compile to an executable and doesn't have a main() function. A library crate generally defines a shared functionality that can be used in multiple projects.

Crates can be bundled together into a package.

____

## Creating a Package in Rust

Packages can be created using the Cargo package manager, which is built into Rust. Cargo comes pre-installed with Rust.

We can use cargo to create a package. A package contains one or more crates that provides a set of functionality.

Note: A package can contain many binary crates, but at most only one library crate.

Creating a Binary Package in Rust

To create a binary package, we can use the cargo command in the terminal.

____

### Aside: Note:

```bash
cd programiz/attempt-01
```

____

```bash
cargo new hello_world --bin
```

### Output

```bash

```
