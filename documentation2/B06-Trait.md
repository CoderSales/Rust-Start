# Trait

A Rust trait defines shared functionality for multiple types.

Rust traits promote type-safety, prevent errors at compile time, and act like interfaces in other languages with some distinctions.

____

## Defining a Trait in Rust

We can define a Rust trait using the `trait` keyword followed by the trait name and the methods that are part of the trait.

Let's look at the syntax of a trait.

```bash
trait TraitName {
    fn method_one(&self, [arguments: argument_type]) -> return_type;
    fn method_two(&mut self, [arguments: argument_type]) -> return_type;
    ...
}
```

Here,

- `TraitName` - name of the trait.

- `method_one()` and `method_two()` - names of the methods in the trait.

`&self` and `&mut self` - references to the self value. A method can take either a mutable or immutable reference to the current object, depending on whether it needs to modify its value.

- `[arguments: argument_type]` (optional) - list of arguments, where each argument has a name and a type.
return_type - type that method returns.

- `return_type` - type that method returns.

Now, let's define a trait.

```rust
trait MyTrait {
    fn method_one(&self);
    fn method_two(&mut self, arg: i32) -> bool;
}
```

Here, we declare a trait called `MyTrait` with method signatures for `method_one(&self)` and `method_two(&mut self, arg: i32) -> bool`. The method signatures describe the behaviors of the types that implement this trait.

A trait can have multiple method signatures in its body, one per line. Traits by default do nothing and only are definitions. In order to use a trait, a type needs to implement it.

____

### Implementing a Trait in Rust

To implement a trait, we use the `impl` keyword. The syntax for the implementation (impl) block is:

```rust
impl TraitName for TypeName {
    fn method_one(&self, [arguments: argument_type]) -> return_type {
        // implementation for method_one
    }

    fn method_two(&mut self, [arguments: argument_type]) -> return_type {
        // implementation for method_two
    }

    ...
}
```

Here, `TraitName` is the name of the trait being implemented and `TypeName` is the name of the type that is implementing the trait.

____

**Note:** The implementation of a trait must have the same signature as the methods in the trait, including the name, the argument types, and the return type.

____

Now, let's implement the trait. We will use the MyTrait as the trait and MyStruct as the type for which we implement the trait.

```rust
trait MyTrait {
    // method signatures
    fn method_one(&self);
    fn method_two(&mut self, arg: i32) -> bool;
}

struct MyStruct {
    value: i32,
}

impl MyTrait for MyStruct {
    // implementation of method_one
    fn method_one(&self) {
        println!("The value is: {}", self.value);
    }

    // implementation of method_two
    fn method_two(&mut self, arg: i32) -> bool {
        if arg > 0 {
            self.value += arg;
            return true;
        } else {
            return false;
        }
    }
}
```

In this example,

- `method_one()` takes a reference to self and prints its value `self.value` field.

- `method_two()` takes a mutable reference to self and an argument `arg` of type `i32`. If `arg` is greater than zero, we add `arg` to the value field and return `true`, otherwise we return `false`.

____

#### Example: Defining, Implementing and Using a Trait in Rust

```rust
// Define a trait Printable
trait Printable {
    fn print(&self);
}

// Define a struct to implement a trait
struct Person {
    name: String,
    age: u32,
}

// Implement trait Printable on struct Person
impl Printable for Person {
    fn print(&self) {
        println!("Person {{ name: {}, age: {} }}", self.name, self.age);
    }
}

// Define another struct to implement a trait
struct Car {
    make: String,
    model: String,
}

// Define trait Printable on struct Car
impl Printable for Car {
    fn print(&self) {
        println!("Car {{ make: {}, model: {} }}", self.make, self.model);
    }
}

// Utility function to print any object that implements the Printable trait
fn print_thing<T: Printable>(thing: &T) {
    thing.print();
}

fn main() {
    // Instantiate Person and Car
    let person = Person { name: "Hari".to_string(), age: 31 };
    let car = Car { make: "Tesla".to_string(), model: "Model X".to_string() };
    
    // Call print_thing with reference of Person and Car
    print_thing(&person);
    print_thing(&car);
}
```

```bash
cargo build
```

```bash
cargo run
```

##### Output

```bash
Person { name: Hari, age: 31 }
Car { make: Tesla, model: Model X }
```
