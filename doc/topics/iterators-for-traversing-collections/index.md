# Iterators for traversing collections

In Rust, iterators are abstractions for traversing collections of data, such as arrays, vectors, and other sequences. Iterators access the elements of a collection, and can be used with many of Rust's built-in language features, such as loops and closures.

Iterators in Rust are defined by the `Iterator` trait, which provides methods for traversing and manipulating a sequence of elements. Some common methods on iterators include:

* `next()`: Return the next iterator element, or None.

* `filter()`: Return the elements that match a predicate.

* `map()`: Transform each element of the iterator.

* `fold()`: Reduce elements to a value via a function.

Example to traverse a vector and sum up its elements:

```rust
let v = vec![1, 2, 3, 4, 5];
let sum = v.iter().fold(0, |acc, x| acc + x);
println!("The sum is: {}", sum);
```

In this example, we create a vector `v` and use the `iter()` method to create an iterator over its elements. We then use the `fold()` method to iterate over the elements, and accumulate the sum of all the elements.

Iterators can also be used in loops, as in the following example:

```rust
let v = vec![1, 2, 3, 4, 5];
for i in v.iter().map(|x| x * 2) {
    println!("{}", i);
}
```

In this example, we create an iterator over the vector elements, and use the `map()` method to transform each element by doubling it. We use a `for` loop to iterate over the transformed elements, to print each one.