# Tutorial-Tuple-onwards

(Back to [27-Tutorial.md](/documentation/27-Tutorial.md) )

## [Rust Tuple](https://www.programiz.com/rust/tuple)

### Example: Tuple with Data Type

```rust 
fn main() {
    // initialization of tuple with data type
    let tuple: (&str, f32, u8) = ("Rust", 3.14, 100);
    
    println!("Tuple contents = {:?}", tuple);
}
```

#### Ouptut

```bash
Tuple contents = ("Rust", 3.14, 100)
```

#### Note

##### `:?` operator

We use the `:?` in the `println!` function to print an entire tuple.

____

### Tuple without Data Type in Rust

We can create a tuple **without mentioning the type of data** it is storing. 

The Rust compiler can automatically detect and set the data type. 

For example,

```rust
// create a tuple without data type
let student_info = ("Ricky", 21, 3.56);
```

Here,

`let student_info` - specifies the variable name of the tuple

`("Ricky", 21, 3.56)` - specifies the elements of the tuple

____

### Example: Tuple without Data Types

```rust
fn main() {
    // initialization of tuple without data type
    let tuple = ("Rust", "fun", 100);

    println!("Tuple contents = {:?}", tuple);
}
```

#### Output

```bash
Tuple contents = ("Rust", 3.14, 100)
```

____

### Accessing Elements in a Tuple

Each element in a tuple is associated with a unique sequence of numbers. 

This number is known as the tuple index or just index.

Suppose we have a tuple,

```rust
let random_tuple = ("Hello", 200, 3.14);
```

The tuple indexes of the `random_tuple` look like,

![TupleIndexImage.png](/static/images/TupleIndexImage.png)

#### Tuple Dot notation access

In Rust, we can access individual tuple elements using their corresponding tuple indexes and the dot . notation. 

For example,

- `random_tuple.0` - access the element at `index 0` (first element)

- `random_tuple.1` - access the element at `index 1` (second element)

- `random_tuple.2` - access the element at `index 2` (third element)

_____

### Example: Access Tuple Elements

```rust
fn main() {
    let random_tuple = ("Hello", 200, 3.14);

    // accessing tuple element at index 0
    println!("Value at Index 0 = {}", random_tuple.0);
    
    // accessing tuple element at index 1
    println!("Value at Index 1 = {}", random_tuple.1);
    
    // accessing tuple element at index 2
    println!("Value at Index 2 = {}", random_tuple.2);
}
```

```bash
Value at Index 0 = Hello
Value at Index 1 = 200
Value at Index 2 = 3.14
```

#### Note: 

The tuple index always starts at 0; hence the first element of the tuple is at position 0, not 1.

____

### Mutable Tuple

#### `mut` Keyword

In Rust, a tuple is immutable, which means we cannot change its elements once it is created.

However, we can create a mutable array by using the mut keyword before assigning it to a variable. For example,

```rust
// create a mutable tuple 
let mut mountains = ("Everest", 8848, "Fishtail", 6993);
```

Now, we can make changes to this tuple.

Let's take a look at an example,



```rust
fn main() {
    // initialize a mutable tuple
    let mut mountain_heights = ("Everest", 8848, "Fishtail", 6993);
    
    println!("Original tuple = {:?}", mountain_heights);
    
    // change 3rd and 4th element of a mutable tuple
    mountain_heights.2 = "Lhotse";
    mountain_heights.3 = 8516;
    
    println!("Changed tuple = {:?}", mountain_heights);
}
```

#### Output

```bash
Original tuple = ("Everest", 8848, "Fishtail", 6993)
Changed tuple = ("Everest", 8848, "Lhotse", 8516)
```

#### Note

Here, we create a mutable tuple named `mountain_heights`. 

We then change its 

`3rd` and `4th` element, the 

`2nd` and `3rd` tuple index.

```rust
mountain_heights.2 = "Lhotse";
mountain_heights.3 = 8516;
```

#### Note

You can only change the element of a tuple to the same type as when it was created. 

Changing data types is not allowed after tuple creation.

____

### Destructuring a Tuple

We can break down tuples into smaller variables in Rust, known as **destructuring**.

Suppose we have a tuple,

```rust
let tuple = ("John Doe", 18, 178);
```

Now, we can perform destructuring using,

```rust
let (name, age, height) = tuple;
```

Now, we access the `name`, `age` and `height` variables directly without using tuple indexes.

- `name` instead of `tuple.0`

- `age` instead of `tuple.1`

- `height` instead of `tuple.2`

You can name the variables as you like while destructuring a tuple.

#### Note

Destructuring a tuple is also known as **tuple unpacking**.

### Example: Destructuring a Tuple

```rust
fn main() {
    let mixture = ("Hello, World!", 16, 2.71828);
    
    // destructuring a tuple
    let (message, number, float) = mixture;
    
    println!("message = {}", message);
    println!("number = {}", number);
    println!("float = {}", float);
}
```

#### Output

```bash
message = Hello, World!
number = 16
float = 2.71828
```

Here, we destructure the tuple `mixture` to variables `message`, `number` and `float`.

```rust
let (message, number, float) = mixture;
```

Finally, we print these variables to the screen.

## [Struct](https://www.programiz.com/rust/struct)

Rust structs or structures are user-defined data types used to store different types of data together.

Suppose we want to store a person's name, age, and height. To do this, we can create variables for each property/field.

```rust
let personName: String = String::from("John Doe");
let personAge: u8 = 18;
let personHeight: u8 = 178;
```

The problem with this approach is we have to maintain all these variables separately. To store these fields for more than one person, we will have to create different variables for each person.

Instead, we can create a struct to store all the fields together as a single unit. For example,

```rust
struct Person {
    name: String,
    age: u8,
    height: u8
}
```

### Defining a Struct in Rust

In Rust, we use the `struct` keyword to define a structure. The syntax of a structure is:

```rust
struct struct_name {
    field1: data_type,
    field2: data_type,
    field3: data_type
}
```

Here,

- `struct` - keyword to define a structure

- `struct_name` - name of the structure

- `field1: data_type/field2: data_type` - name and data type of the fields inside the struct.


Let's look at an example.

```rust
struct Person {
    name: String,
    age: u8,
    height: u8
}
```

Here, we have defined a structure named `Person`. It contains three fields:

- `name` - with data type `String`
- `age` - with data type `u8`
- `height` - with data type `u8`

### Instantiating Rust Struct

To use a structure in Rust, we first have to create an instance of structures. For example,

```rust
// define a structure
struct Person {
    name: String
    age: u8,
    height: u8
}

// create an instance of Person struct
let person1 = Person {
    ...
};
```

Here, `Person {...}` creates an instance of the **Person struct**, and we have assigned it to the `person1` variable.

We can also assign values to the struct fields while creating the instance. For example,

```rust
let person1 = Person {
    name: String::from("John Doe"),
    age: 18,
    height: 178
};
```

Here, we have initialized the values of the `name`, `age` and `height` fields of the Person struct. This process of initializing the values of struct fields is known as an 

**instantiation of a struct**.

#### Note

The struct definition is a template, 

and the struct instances fill in that template with data.

____

### Accessing Fields of a Struct

We can use the struct instance along with the dot . notation to access values of fields in a structure. For example,

```rust
fn main() {
    // define a Person struct
    struct Person {
        name: String,
        age: u8,
        height: u8
    }
    
    // instantiate Person struct
    let person = Person {
        name: String::from("John Doe"),
        age: 18,
        height: 178
    };
    
    // access value of name field in Person struct
    println!("Person name = {}", person.name);

    // access value of age field in Person struct
    println!("Person age = {}", person.age);

    // access value of height field in Person struct
    println!("Person height = {}", person.height);
}
```

#### Output

```bash
Person name = John Doe
Person age = 18
Person height = 178
```

Here,

- `person.name` - reads the field `name` of the Person struct (`John Doe`)

- `person.age` - reads the field `age` (`18`)

- `person.height` - reads the field `height` (`178`)

____

### Destructuring Fields of a Rust Struct

Destructuring is the process of breaking down fields of a data type (array, tuple, etc.) into smaller variables. We can break down the struct fields into smaller variables in Rust.

Suppose we have a struct and a struct instance,

```rust
struct Person {
    name: String,
    age: u8,
    height: u8
}

let person = Person {
    name: String::from("John Doe"),
    age: 18,
    height: 178
};
```

We can now perform destructuring using:

```rust
// destructuring the Person struct
let Person { name, age, height } = person;
```

Now, we access the `name`, `age` and `height` fields using the field names directly:

- `name` instead of `person.name`

- `age` instead of `person.age`

- `height` instead of `person.height`

However, you should note that the name of the variables while destructuring should be the same as the name of the fields.

____

#### Example: Destructuring Fields of Struct

```rust
fn main() {
    // define a Person struct
    struct Person {
        name: String,
        age: u8,
        height: u8
    }
    
    // instantiate Person struct
    let person = Person {
        name: String::from("John Doe"),
        age: 18,
        height: 178
    };
    
    // destructure Person struct into name, age and height variables
    let Person { name, age, height } = person;
    
    println!("Person name = {}", name);
    println!("Person age = {}", age);
    println!("Person height = {}", height);
}
```

```bash
Person name = John Doe
Person age = 18
Person height = 178
```

Here, the destructing happens with this expression,

```rust
let Person { name, age, height } = person;
```

The pattern on the left has declarations, and the right side of the expression has a struct instance.

On the left side of the expression, we are making `let` declarations for the `Person` struct with field `name`, `age` and `height`. On the right side of the expression, we assign the instantiated struct of the `Person`.

As a result, we get the `name`, `age` and `height` of the person and print it to the screen.
