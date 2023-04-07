// Reference: The Rust Programming Language (online book)
// Ch5.1 - https://rust-book.cs.brown.edu/ch05-01-defining-structs.html

struct Color(i32, i32, i32);
struct Point(i32, i32, i32);
struct AlwaysEqual;

fn main() {
    let user1 = build_user(
        String::from("joey@gmail.com"),
        String::from("Joey Tribbiani"),
    );
    // Legal
    print_user(&user1);

    let user2 = User {
        email: String::from("another@example.com"),
        ..user1
    };

    // Legal
    print_user(&user2);
    // Illegal, since user1 has been partially moved to user2.
    // Remember, String is NOT copyable.
    //print_user(&user1);

    // tuple structs
    let _black = Color(0, 0, 0);
    let _origin = Point(0, 0, 0);

    // unit struct
    let _subject = AlwaysEqual;

    let mut p = Point2d { x: 0, y: 0 };
    let x = &mut p.x;

    //print_point(&p); // Illegal: p loses permissions while x is alive.
    //p.x = -1; // Illegal: p.x loses permissions while x is alive.
    println!("{}", p.y); // Legal: p.y doesn't lose permissions while x is alive.
    *x += 1;
    print_point(&p);
}

struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}

fn print_user(user: &User) {
    println!(
        " Email: {}\n Username: {}\n Active: {}\n Sign in count: {}\n",
        user.email, user.username, user.active, user.sign_in_count
    )
}

struct Point2d {
    x: i32,
    y: i32,
}

fn print_point(p: &Point2d) {
    println!("{}, {}", p.x, p.y);
}
