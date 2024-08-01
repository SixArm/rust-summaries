use regex::Regex;

fn main() {
    // Find the first occurrence of a digit char and word char
    let r = Regex::new(r"(\d)(\w)").unwrap();
    let captures = r.captures("a1b2c3").unwrap();
    println!("{:?}", captures)
    // Captures({0: Some("1b"), 1: Some("1"), 2: Some("b")})
}
