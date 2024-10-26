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
