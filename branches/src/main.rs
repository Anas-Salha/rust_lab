// Reference: The Rust Programming Language (online book)
// Ch3.5 - https://doc.rust-lang.org/book/ch03-05-control-flow.html


fn main() {
    let number = 3;
    if number != 0 {
        println!("number was something other than zero");
    }

    let condition = true;
    let number = if condition { 5 } else { 6 };
    println!("The value of number is: {number}");

    let mut counter = 0;
    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };
    println!("The result is {result}");

    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }
        count += 1;
    }
    println!("End count = {count}");

    let mut number = 3;
    'can_I_label_while: while number != 0 {
        println!("{number}!");

        number -= 1;
    }
    println!("LIFTOFF!!!");

    let a = [10, 20, 30, 40, 50];
    'can_I_label_for: for element in a {
        println!("the value is: {element}");
        //println!("Can I print a label? {'can_I_label_for}"); //obviously not
    }

    //for i in 4..1 { //Apparently this compiles but doesn't countdown. Curious.
    for i in (1..4).rev() {

        println!("{i}!");
    }
    println!("LIFTOFF!!!");

}
