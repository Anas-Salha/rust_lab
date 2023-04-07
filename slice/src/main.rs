// Reference: The Rust Programming Language (online book)
// Ch4.4 - https://rust-book.cs.brown.edu/ch04-04-slices.html

fn main() {
    let mut s = String::from("hello");
    let hello: &str = &s[0..5];
    println!("{hello}");
    s.push_str(" world");

    let s = String::from("hello");
    let slice1 = &s[0..2];
    let slice2 = &s[..2];
    assert_eq!(slice1, slice2);

    let slice1 = &s[3..s.len()];
    let slice2 = &s[3..];
    assert_eq!(slice1, slice2);

    let slice1 = &s[0..s.len()];
    let slice2 = &s[..];
    assert_eq!(slice1, slice2);

    let mut s = String::from("Hello world");
    let hello: &str = first_word(&s);
    //s.clear(); // ERR- slices change permissions. 's' loses W so long as 'hello' is alive
    println!("{hello}");
    s.clear();

    let mut s = "Hello, world!";
    s = first_word(&s);
    println!("{s}");

    let s = first_word("HELLO WORLD!");
    println!("{s}");

    let s = first_word(&"Hey there!"[..]);
    println!("{s}");

    let a = [1, 2, 3, 4, 5];
    let slice = &a[1..3];
    assert_eq!(slice, &[2, 3]);
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (index, &byte) in bytes.iter().enumerate() {
        if byte == b' ' {
            return &s[..index];
        }
    }
    &s[..]
}
