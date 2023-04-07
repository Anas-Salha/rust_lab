// Reference: The Rust Programming Language (online book)
// Ch5.2 - https://rust-book.cs.brown.edu/ch05-02-example-structs.html
// Ch5.3 - https://rust-book.cs.brown.edu/ch05-03-method-syntax.html

fn main() {
    // Using single variables
    let width = 30;
    let height = 50;
    println!(
        "The area of the rectangle is {} square pixels.",
        area(width, height)
    );

    // Using a tuple
    let rect_tuple = (40, 50);
    println!(
        "The area of the rectangle is {} square pixels.",
        area_tup(rect_tuple)
    );

    // Using a struct
    let rect1 = Rectangle {
        width: 50,
        height: 50,
    };
    println!(
        "The area of the rectangle is {} square pixels.",
        area_struct(&rect1)
    );

    println!("rect1 is {:#?}", rect1);

    let scale = 2;
    let rect2 = Rectangle {
        width: dbg!(30 * scale), //dbg returns ownership, thus the return value can be used
        height: 50,
    };
    dbg!(&rect2); //dbg takes ownership, thus we pass by reference

    println!(
        "The area of the ractangle is {} square pixels.",
        rect2.area()
    );

    let rect3 = Rectangle {
        width: 10,
        height: 15,
    };

    println!("Can rect2 hold rect1? {}", rect2.can_hold(&rect1));
    println!("Can rect2 hold rect3? {}", rect2.can_hold(&rect3));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));

    let square = Rectangle::square(20);
    dbg!(&square);

    // Example below demonstrates the usefulness of the dot operator.
    // Lines 65 and 66 are equivalent.
    // The dot operator does as many references/dereferences as needed to match self.
    let r = &mut Box::new(Rectangle {
        width: 1,
        height: 2,
    });
    let area1 = r.area();
    let area2 = Rectangle::area(&**r);
    assert_eq!(area1, area2);
}

fn area(w: u32, h: u32) -> u32 {
    println!("\nWidth x height area function");
    w * h
}

fn area_tup(dimensions: (u32, u32)) -> u32 {
    println!("\nTuple area function");
    dimensions.0 * dimensions.1
}

fn area_struct(rectangle: &Rectangle) -> u32 {
    println!("\nRectangle area function");
    rectangle.width * rectangle.height
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        println!("\nRectangle area method");
        self.width * self.height
    }

    fn can_hold(&self, inner: &Rectangle) -> bool {
        println!("\nRectangle can_hold method");
        self.height > inner.height && self.width > inner.width
    }

    fn square(side: u32) -> Self {
        Self {
            width: side,
            height: side,
        }
    }
}
