// Reference: The Rust Programming Language (online book)
// Ch7.2 - https://rust-book.cs.brown.edu/ch07-02-defining-modules-to-control-scope-and-privacy.html
// Ch7.3 - https://rust-book.cs.brown.edu/ch07-03-paths-for-referring-to-an-item-in-the-module-tree.html

mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}

        pub fn seat_at_table() {}
    }

    mod serving {
        fn take_order() {}

        fn serve_order() {}

        fn take_payment() {}
    }
}

mod back_of_house {
    // A public struct does NOT mean it's members are public.
    pub struct Breakfast {
        pub toast: String,      // Public
        seasonal_fruit: String, // Private
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }

    // Unlike structs, a public enum makes all its variants public.
    pub enum Appetizer {
        Soup,
        Salad,
    }

    fn fix_incorrect_order() {
        cook_order();
        super::deliver_order(); // super is eqivalent to .. in filesystems.
    }

    fn cook_order() {}
}

fn deliver_order() {}

// The function eat_at_restaurant is defined in the same module as front_of_house and back_of_house.
// Thus, eat_at_restaurant and front_of_house are siblings and can access each other.
// Meaning 'pub' is not needed to make the module front_of_house accessible by eat_at_restaurant.
// However, eat_at_restaurant cannot access front_of_house's "children" without 'pub'.
pub fn eat_at_restaurant() {
    // Absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative path
    front_of_house::hosting::add_to_waitlist();

    // Doesn't compile -> serving is not public.
    // crate::front_of_house::serving::take_order();

    // Order a breakfast in the summer with Rye toast
    let mut meal = back_of_house::Breakfast::summer("Rye");
    // Change our mind about what bread we'd like
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);

    // Does not compile -> seasonal_fruit is private.
    // i.e. the customer does not get to choose or even know the fruit.
    // meal.seasonal_fruit = String::from("blueberries");

    // The variants are accessible/usable because the enum is public
    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;
}
