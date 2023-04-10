// Reference: The Rust Programming Language (online book)
// Ch6.3 - https://rust-book.cs.brown.edu/ch06-03-if-let.html

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    Arizona,
    Arkansas,
    California,
    Colorado,
    Connecticut,
    Delaware,
    Florida,
    Georgia,
    Hawaii,
    Idaho,
    Illinois,
    Indiana,
    Iowa,
    Kansas,
    Kentucky,
    Louisiana,
    Maine,
    Maryland,
    Massachusetts,
    Michigan,
    Minnesota,
    Mississippi,
    Missouri,
    Montana,
    Nebraska,
    Nevada,
    NewHampshire,
    NewJersey,
    NewMexico,
    NewYork,
    NorthCarolina,
    NorthDakota,
    Ohio,
    Oklahoma,
    Oregon,
    Pennsylvania,
    RhodeIslands,
    SouthCarolina,
    SouthDakota,
    Tennessee,
    Texas,
    Utah,
    Vermont,
    Virginia,
    Washington,
    WestVirginia,
    Wisconsin,
    Wyoming,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

impl Coin {
    fn value_in_cents(&self) -> u8 {
        match self {
            Coin::Penny => 1,
            Coin::Nickel => 5,
            Coin::Dime => 10,
            Coin::Quarter(state) => {
                println!("State quarter from {:?}!", state);
                25
            }
        }
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}
fn main() {
    // Enum matching
    let coin = Coin::Quarter(UsState::Wisconsin);
    let value = coin.value_in_cents();
    println!("{value}");

    // Option enum matching
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
    dbg!(&six);
    dbg!(&none);

    // Since String is non-copyable, the data in opt will be moved to s if we don't match by ref
    let opt: Option<String> = Some(String::from("Hello world"));
    match &opt {
        Some(s) => println!("Some: {}", s), // s has type &string in this case
        None => println!("None!"),
    };
    println!("{:?}", opt);
}
