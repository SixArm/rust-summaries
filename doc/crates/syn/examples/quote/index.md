# syn crate - example of quote and parse_quote

One way to use the `syn` crate and its `to_tokens` capability is to use the `quote!` macro to convert an instance to tokens, and the macro `parse_quote!` to convert the tokens to an instance.

Example:

```rust
use syn::parse_quote;

fn main() {
    let person: Person = Person {
        name: String::from("John Doe"),
        age: 35,
    };
    let tokens = quote! {
        #person
    };
    let parsed: Person = parse_quote!(#tokens);
    assert_eq!(parsed.name, "John Doe");
    assert_eq!(parsed.age, 35);
}
```

In this example, we create an instance of `Person` and generate a token stream using the `quote!` macro. We then parse the token stream using the `parse_quote` function, which returns an instance of `Person` with the same values as the original one.

Note that this is just a simple example, and the Syn crate can be used for much more complex code generation tasks, especially when combined with other Rust macro libraries.
