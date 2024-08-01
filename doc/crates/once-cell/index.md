# once_cell crate for lazy global variables

<https://crates.io/crates/once_cell>

The Rust once_cell crate provides a way to create lazily evaluated, immutable, and thread-safe global variables in Rust. It is designed to provide a simple and efficient way to handle global state in Rust programs.

The main type provided by the once_cell crate is the OnceCell<T> type. This type is a container for a single value of type T that can be initialized lazily and only once. When the value is accessed for the first time, it is created using a closure that is passed to the OnceCell's get_or_init method. The closure is executed only once, and the resulting value is stored in the OnceCell for future accesses.

The OnceCell type is also thread-safe, which means that multiple threads can access the same OnceCell instance safely. If multiple threads attempt to access the OnceCell at the same time, only one of them will be allowed to execute the initialization closure, while the other threads will block until the value is fully initialized.

The OnceCell crate also provides other useful types, such as the unsync::OnceCell<T> type, which is similar to the regular OnceCell<T> but is not thread-safe, and the sync::Lazy<T> type, which is similar to the OnceCell<T> but provides an additional level of indirection that allows for even more efficient initialization and access.

Example of once_cell `Lazy` to initialize a `Regex` regular expression:

```rust
use regex::Regex;
use once_cell::sync::Lazy;

fn main() {
    static RE: Lazy<Regex> =
        Lazy::new(|| Regex::new("hello").unwrap());
    let matched = RE.is_match("hello world");
    println!("{}", matched);
}
```
