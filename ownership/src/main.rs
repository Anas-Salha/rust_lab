// Reference: The Rust Programming Language (online book)
// Ch4.1 - https://doc.rust-lang.org/book/ch02-00-guessing-game-tutorial.html

fn main() {
    // Cannot access a and c in the commented out prints below, since the 'Box' type doesn't have a Copy trait.
    // Meaning ownerships of 'a' and 'c' were moved to 'b' and 'd'.

    let a = Box::new([0; 1_000_000]);
    let b = a;
    println!("The box as seen from b: {:?}", b);
    // println!("The box as seen from a: {:?}", a);

    let c = Box::new([0, 1, 2, 3, 4, 5]);
    let mut d = c;
    d[0] = 6;
    // println!("c[0] = {}", c[0]);

    let e = Box::new([5, 4, 3, 2, 1, 0]);
    printer(&e);
    println!("e = {:?}", e); // passing by reference means 'e' remains accessible, since 'e' retains box ownership.

    // Collections, such as Vec and String, use box-like types; ie. they are allocated on the heap.
    let first = String::from("Ferris");
    let mut full = add_suffix(first); // When name is returned, ownership is transferred to 'full'.
    println!("{full}");
    add_prefix_by_ref(&mut full); // Passing by mutable reference does NOT transfer ownership.
    println!("{full}");

    // clone() makes a deep copy of 'full'. Meaning 'full_clone' is pointing to a different memory location.
    let full_clone = full.clone();
    let new = pop_last(full_clone); //full_clone is deallocated; full remains intact.
    println!("{new}, originally {full}");
}

fn printer(tmp: &Box<[i32; 6]>) {
    println!("Tmp = {:?}", tmp)
}

fn add_suffix(mut name: String) -> String {
    name.push_str(" Jr."); // The operation push_str transfers ownership deallocates 'first' and transfers ownership to 'name'.
    name
}

fn add_prefix_by_ref(name: &mut String) {
    name.insert_str(0, "Mr. ");
}

fn pop_last(mut name: String) -> String {
    name.pop();
    name
}

// Moved heap data principle: if a variable x moves ownership of heap data to another variable y, then x cannot be used after the move.
