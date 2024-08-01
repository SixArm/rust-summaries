# Zero-cost abstractions - example

Here's an example of zero-cost abstractions:

```
fn add<T: std::ops::Add<Output=T>>(x: T, y: T) -> T {
    x + y
}

fn main() {
    let x = 1;
    let y = 2;
    let z = add(x, y);
    println!("{}", z);
}
```

In this example, the `add` function takes two arguments of any type that implements the `Add` trait, adds them together using `+`, and returns the result.

The `add` function is generic, so it can be used with any type that implements `Add`, such as numbers, strings, or even custom objects.

Because the function is generic, it will be optimized by the Rust compiler to perform as efficiently as possible. This means that using the `add` function will not incur any additional runtime overhead, even though it uses an abstraction (the `Add` trait) to make the function more generic and reusable.

In this way, Rust demonstrates the concept of zero-cost abstraction, allowing developers to write modular, reusable code without sacrificing performance.
