// Reference: The Rust Programming Language (online book)
// Ch6.1 - https://rust-book.cs.brown.edu/ch06-01-defining-an-enum.html

fn main() {
    let home = IpAddr::V4(Ipv4Addr {
        octet1: 127,
        octet2: 0,
        octet3: 0,
        octet4: 1,
    });
    let loopback = IpAddr::V6(String::from("::1"));

    println!("{:?}", home);
    println!("{:?}", loopback);
    home.print();
    loopback.print()
}

#[derive(Debug)]
enum IpAddr {
    V4(Ipv4Addr),
    V6(String),
}

#[derive(Debug)]
struct Ipv4Addr {
    octet1: u8,
    octet2: u8,
    octet3: u8,
    octet4: u8,
}

impl IpAddr {
    fn print(&self) {
        match self {
            IpAddr::V4(ipv4) => println!(
                "{}.{}.{}.{}",
                ipv4.octet1, ipv4.octet2, ipv4.octet3, ipv4.octet4
            ),
            IpAddr::V6(ipv6) => println!("{ipv6}"),
        }
    }
}

// Variations within an enum type
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}
