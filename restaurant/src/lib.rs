// Reference: The Rust Programming Language (online book)
// Ch7.2 - https://rust-book.cs.brown.edu/ch07-02-defining-modules-to-control-scope-and-privacy.html

mod front_of_house {
    mod hosting {
        fn add_to_waitlist() {}

        fn seat_at_table() {}
    }

    mod serving {
        fn take_order() {}

        fn serve_order() {}

        fn take_payment() {}
    }
}
