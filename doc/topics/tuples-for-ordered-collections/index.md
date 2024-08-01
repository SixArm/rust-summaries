# Tuples for ordered collections

In Rust, a tuple is an ordered collection of values with a fixed length. Tuples can contain values of different types and are represented using parentheses with the values separated by commas.

Example of a tuple containing a string and an integer:

```rust
let person = ("Alice", 30);
```

This defines a tuple called `person` containing the string "Alice" and the integer `30`. Tuples can be assigned to variables, passed as function arguments, and returned as function results, just like any other value.

You can access individual elements of a tuple using dot notation and the index of the element you want to access, starting from zero:

```rust
let name = person.0;
let age = person.1;
```

Tuples are often used to return multiple values from a function. For example, the `std::fs::metadata` function returns a tuple that contains information including a file's length and permissions:

```rust
use std::fs;

fn main() -> std::io::Result<()> {
    let metadata = fs::metadata("file.txt")?;
    let (len, perms) = (
        metadata.len(),
        metadata.permissions(),
    );
    println!("File len:{}, permissions:{}", len, permissions);
    Ok(())
}
```
