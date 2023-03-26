const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

fn main() {

    println!("The value of THREE_HOURS_IN_SECONDS is: {THREE_HOURS_IN_SECONDS}");

    let mut mutable_variable = 5;
    println!("The value of mutable_variable is: {mutable_variable}");
    mutable_variable = 6;
    println!("The value of mutable_variable is: {mutable_variable}");


    let shadowed_variable = 14;
    println!("The value of shadowed_variable is: {shadowed_variable}");

    let shadowed_variable = shadowed_variable + 1;
    println!("The value of shadowed_variable is: {shadowed_variable}");

    {
        let shadowed_variable = shadowed_variable * 2;
        println!("The value of shadowed_variable is: {shadowed_variable}");
    }

    println!("The value of shadowed_variable is: {shadowed_variable}");

    let mut small_integer: i8 = 120;
    println!("The value of small_integer is: {small_integer}");

    // u8 overflows when (120 + 10) => None => small_integer value doesnt change.
    small_integer = match small_integer.checked_add(10){
        Some(result) => result,
        None => small_integer,
    };
    println!("The value of small_integer is: {small_integer}");


    // u8 can hold (120 + 5) => Some => store result.
    small_integer = match small_integer.checked_add(5){
        Some(result) => result,
        None => small_integer,
    };
    println!("The value of small_integer is: {small_integer}");

    let num_literal = 1_000;
    println!("The value of num_literal is: {num_literal}");

    let num_literal = 0xFF;
    println!("The value of num_literal is: {num_literal}");

    let num_literal = 0b1111_1110;
    println!("The value of num_literal is: {num_literal}");

    let num_literal = 10u8;
    println!("The value of num_literal is: {num_literal}");


}
