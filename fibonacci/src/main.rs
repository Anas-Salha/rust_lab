// Practice example (control flow):
// Fibonacci nth number generator.

// n = 0 -> result(n) = 0
// n = 1 -> result(n) = 1
// n = 2 -> result(n) = 1 + 0 = 1
// n = 3 -> result(n) = 1 + 1 = 2
// n = 4 -> result(n) = 2 + 1 = 3
// Condition: n = i -> result(n) = result(i-1) + result(i-2), where i != 0 or 1.

use std::io;
fn main() {
    static FIB0: u32 = 0;
    static FIB1: u32 = 1;
    static MAX_ALLOWED_INPUT: u32 = 47; //The 48th Fibonacci number > u32::MAX -> program will panic

    let out_msg = "The Fibonacci number is: ";
    let prog_name = "Fibonacci Generator";
    println!("Welcome to {prog_name}!");

    'infinite_loop: loop {
        // User interface
        println!("Select a number (0-47) to get the corresponding Fibonacci number (enter 'q' to quit).");
        let mut user_input = String::new();

        io::stdin()
            .read_line(&mut user_input)
            .expect("Failed to read line");

        if user_input == "q\n"{
            break 'infinite_loop;
        }

        let user_input: u32 = match user_input.trim().parse() {
            Ok(num) => {
                if num > MAX_ALLOWED_INPUT {
                    println!("As she often says; TOO BIG");
                    continue 'infinite_loop;
                } else {
                    num
                }
            },
            Err(_) => {
                println!("Don't be weird. Pick a number!");
                continue 'infinite_loop;
            }
        };

        // Logic
        if user_input == 0 {
            println!("{out_msg} {FIB0}");
            continue 'infinite_loop;
        }
        if user_input == 1 {
            println!("{out_msg} {FIB1}");
            continue 'infinite_loop;
        }

        let mut next_fib_number = FIB0 + FIB1;
        let mut prev_fib_number = FIB1;

        for _ in 2..user_input{
            let buffer = next_fib_number;
            next_fib_number = next_fib_number + prev_fib_number;
            prev_fib_number = buffer;
        }
        println!("{out_msg} {next_fib_number}");
    }

    println!("Hasta la vista!");
}

