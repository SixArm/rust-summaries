# Explicit lifetimes

Explicit lifetimes are atypical, and only needed when the Rust compiler can't figure out the lifetimes. For this, the developer must write explicit lifetimes so the compiler knows what to do.

Suppose a function has two input args and an output reference:

```rust
fn trim_period(s: &String, msg: &String) -> &str {
    println!(msg);
    s.trim_matches('.')
}
```

The compiler cannot figure out the lifetimes by using the function signature:

* The compiler sees the output string slice, and knows that the string slice memory must come from somewhere.
  
* The compiler sees the function has two input args, so knows that the memory must come from either of the input args, or possibly from a combination of them. 
  
* Therefore the compiler cannot automatically set lifetimes: it's unknown which input arg must live at least as long as the output string slice, or if there's something else involved from a combination.

The compiler requires the developer to write explicit lifetimes:

```rust
fn trim_period<'a>(s: &'a String, msg: &String) -> &'a str {
    println!(msg);
    s.trim_matches('.')
}
```

The explicit lifetimes instruct the compiler that the output string slice memory comes from the first input arg, not the second input arg.
