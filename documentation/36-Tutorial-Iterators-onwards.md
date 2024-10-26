# [Iterators](https://www.programiz.com/rust/iterator)

[Back to 35-Tutorial-HashSet-continued-onwards.md](/documentation/35-Tutorial-HashSet-continued-onwards.md)

An iterator in Rust is responsible for creating a sequence of values and allows us to iterate over each item of the sequence. It is primarily used for looping and we can only loop over iterators in Rust.

Let's look at a simple example on how we can loop through an array.

```rust
let numbers = [2, 1, 17, 99, 34, 56];
```

Now, let's change the array to an iterable array by calling the `iter()` method. If a data structure has the `iter()` method, it is called iterable.

```rust
let numbers_iterator = numbers.iter();
```

Finally, we can loop through the values and print them out.

```rust
for number in numbers_iterator {
    println!("{}", number);
}
```

#### Note:

Collections like Array, Vector, HashMap and HashSet are not iterable by default. We can use the `iter()` method to tell Rust that it can be used to loop over the values.

____

### Example: Iterator in Rust

```rust
fn main() {
    let numbers = [2, 1, 17, 99, 34, 56];
    
    // iterator
    let numbers_iterator = numbers.iter();
    
    for number in numbers_iterator {
        println!("{}", number);
    }
}
```

#### Output

```bash
2
1
17
99
34
56
```

____

Here, `for..in` loop is called using the iterator in `numbers_iterator`, each value in the iterator is used in one iteration and then printed to the screen.

____

### next() Method of an Iterator in Rust

Another important method of iterator is the `next()` method. The `next()` method of an iterator can be used to traverse through the values in the iterator.

Every iterator in Rust by definition will have the `next()` method. The `next()` method is used to fetch individual values from the iterator.

Let's take a look at an example.

```rust
fn main() {
    let colors = vec!["Red", "Yellow", "Green"];
    
    // iterator
    let mut colors_iterator = colors.iter();
    println!("colors iterator = {:?}", colors_iterator);
    
    // fetch values from iterator one by one using next() method
    println!("{:?}", colors_iterator.next());
    println!("{:?}", colors_iterator.next());
    println!("{:?}", colors_iterator.next());
    println!("{:?}", colors_iterator.next());
}
```

#### Output

```bash
colors iterator = Iter(["Red", "Yellow", "Green"])
Some("Red")
Some("Yellow")
Some("Green")
None
```

Here, we fetch values from the iterator in `colors_iterator` using the `next()` method. The `next()` method either returns `Some` value or `None`.

Notice that we need to make the `colors_iterator` a mutable variable because calling `next()` will change the internal state of the iterator. Each call to `next()` will consume an item from the iterator.

The `next()` method returns `None` when the iterator reaches the end of the sequence.

____

### Ways to Create Iterator in Rust

We can create an iterator by converting a collection into an iterator. There are three ways to create an iterator.

1. Using `iter()` method

2. Using `into_iter()` method

3. Using `iter_mut()` method

All these methods provide different views of the data within the iterator.

#### 1. Using iter() method

Using the `iter()` method on a collection will borrow (reference) each element of the collection in each iteration. Thus, the collection will be available for use after we have looped through it.

For example,

```rust
fn main() {
    let colors = vec!["Red", "Yellow", "Green"];
    
    // using iter() to iterate through a collection
    for color in colors.iter() {
        // reference to the items in the iterator
        println!("{}", color);
    }
    
    // the collection is untouched and still available here
    println!("colors = {:?}", colors);
}
```

##### Output

```bash
Red
Yellow
Green
colors = ["Red", "Yellow", "Green"]
```

____

#### 2. Using into_iter() method

Using the `into_iter()` method on a collection will iterate on the same element of the collection in each iteration. Thus, the collection will no longer be available for reuse as the value moves within the loop.

For example,

```rust
fn main() {
    let colors = vec!["Red", "Yellow", "Green"];
    
    // using into_iter() to iterate through a collection
    for color in colors.into_iter() {
        // the items in the collection move into this scope
        println!("{}", color);
    }
    // end of scope of for loop
    
    // error
    // the collection is not available here as the for loop scope ends above
    println!("colors = {:?}", colors);
}
```

##### Output

```bash

```
