# regex! macro for lazy static optimization

<https://crates.io/crates/once-cell-regex>

The `regex!` macro takes a string literal and returns an expression that evaluates to a `&'static Regex`. This macro can be useful to avoid the problem of compiling a regex on every loop iteration.

The `regex!` macro capabilities are provided by the `once_cell` crate and `once-cell-regex` crate.

Add to `Cargo.toml`:

```toml
[dependencies]
once_cell = "*"
once-cell-regex = "*"
```

Example:

```rust
use once_cell_regex::regex;

fn main() {
    let r = regex!("hello");
    let x = r.is_match("hello world");
    println!("{}", x); // prints "true"
}
```

## once_cell crate

The `once_cell` crate can provide optimizations in many more ways, such as safe initialization of global data, general purpose lazy evaluation, runtime bytes, late initialization, and more. 

There are similar crates if you need related features: 

* If you want asynchronous capabilities, try the `async_once_cell` crate.
  
* If you want spinlocks, try the `lazy_static` crate.
