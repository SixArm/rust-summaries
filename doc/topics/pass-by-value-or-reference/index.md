# Pass by value or reference

Passing a value to a function can be done by value or by reference.

Pass by value means that a copy of the original value is passed to the function. Any changes made to the value inside the function will not affect the original value. In other popular languages, pass by value is typical for passing primitive data types.

Pass by reference means that a reference to the original value is passed to the function instead of a copy. This allows the function to modify the original value, as it has access to the actual memory location of the value. In other popular languages, pass by reference is typical for passing object data type pointers.

For example:

```rust
fn increment_with_pass_by_value(num: i32) {
    num + 1;
}

fn increment_with_pass_by_reference(num: &mut i32) {
    *num += 1;
}

fn main() {
    let mut x = 1;
    increment_with_pass_by_value(x);
    println!("x is {}", x); // x is still 1
    increment_with_pass_by_reference(&mut x);
    println!("x is {}", x); // x is now 2
}
```

One of the advantages of Rust is compiler warnings and help. In the pass by value function, the compiler detects that the function result is never used, and shows warnings such as "the arithmetic operation produces a value" and "note: #[warn(unused_must_use)] on by default" and "help: use let _ = ... to ignore the resulting value".
