# regex crate for regular expressions

<https://crates.io/crates/regex>

The Rust regex crate is a regular expression library for the Rust programming language. It provides a fast and efficient way to search, match, and manipulate text using regular expressions.

The main types provided by the regex crate are the `Regex` and `Captures` types. The `Regex` type represents a compiled regular expression pattern that can be used to search for matches in a text string. The `Captures` type represents the groups captured by a successful match and allows for easy extraction of matched substrings.

The regex crate supports a wide range of regular expression syntax, including Perl-style regular expressions and POSIX extended regular expressions. It also supports Unicode character properties and provides a range of Unicode-aware matchers and modifiers.

The regex crate is highly performant and is designed to handle large inputs efficiently. It provides a range of options for controlling the matching behavior, such as case-insensitive matching, multi-line matching, and greedy or lazy quantifiers.

Example:

```rust
use regex::Regex;

fn main() {
    // Find the first occurrence of a digit char and word char
    let r = Regex::new(r"(\d)(\w)").unwrap();
    let captures = r.captures("a1b2c3").unwrap();
    println!("{:?}", captures)
}
```

Output:

```txt
Captures({0: Some("1b"), 1: Some("1"), 2: Some("b")})
```
