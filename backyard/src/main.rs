// Reference: The Rust Programming Language (online book)
// Ch7.2 - https://rust-book.cs.brown.edu/ch07-02-defining-modules-to-control-scope-and-privacy.html

use crate::garden::vegetables::Asparagus;

pub mod garden;

fn main() {
    let plant = Asparagus {};
    println!("I'm growing {:?}!", plant);
}
