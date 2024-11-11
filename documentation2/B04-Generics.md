# [Generics](https://www.programiz.com/rust/generics)

Generics allows us to write code that is flexible and can be reused with different types of data, without having to write separate implementations for each type. It helps us write code that can handle values of any type in a type-safe and efficient way.

With the help of generics, we can define placeholder types for our methods, functions, structs, enums and traits.

____

## Using Generics in Rust

We can understand generics by taking a look at [Rust Hashmap](https://www.programiz.com/rust/hashmap).

HashMap uses generics which allows creation of reusable and efficient code, as a single implementation that works with different types.

A Rust HashMap has two generic types, one for the key and the second for the value.

A HashMap type looks like this:

```rust
HashMap<K, V>
```

where <K, V>: K is the type of the key and V is the type of the value.

Now, when we create a HashMap we can set any type to K and V.

```rust
let mut numbers: HashMap<i32, &str> = HashMap::new();
```

Here, the angle bracket <i32, &str> notation denotes the type of key and type of value of the HashMap. The type of the key K is i32 and the type of the value V is &str.

Similarly, we create a HashMap and set the type of both key and value to &str.

```rust
let mut language_codes: HashMap<&str, &str> = HashMap::new();
```

Using generics to define the type of HashMap helps us work with numerous arbitrary types available in Rust.

To know the basics of HashMap, visit [Rust HashMap](https://www.programiz.com/rust/hashmap).

**Note:**

- Generics or generic types use a single character like K, V, T, U to distinguish from actual concrete types like String, &str, i32.

- As a convention,

    - T, U are used for arbitrary types

    - K, V are used for key-value types

    - E is used for error type

____
