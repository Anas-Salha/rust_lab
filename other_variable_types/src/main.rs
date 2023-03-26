//uncomment for panic example
// use std::io;  

fn main() {
    // floating point
    let x = 2.1;
    let y: f32 = 3.02;
    println!("the values of x and y are {x} and {y}");

    let sum = x + 11.1;
    let difference = 99.0 - y;
    println!("the values of sum and difference are {sum} and {difference}");

    let product = 4*30;
    let floored_quotient = 30/4;
    println!("the values of product and floored_quotient are {product} and {floored_quotient}");

    let quotient = 10.5 / 5.0;
    println!("the value of quotient is {quotient}");

    let remainder = 43 % 4;
    println!("the value of remainder is {remainder}");

    let t = true;
    let f = false;
    println!("the values of t and f are {t} and {f}");
    
    //char in rust represents a unicode scalar value, which means it can represent a lot more than just ascii.
    let heart_eyed_cat = '😻';
    println!("the value of heart_eyed_cat is {heart_eyed_cat}"); 
    

    let player = ("anas", 26, 175.0);
    let (name, _age, height) = player;
    println!("the player name is {name}"); 
    println!("the player age is {}", player.1); 
    println!("the player height is {:.1}", height); 
    println!("the player's name, age, and height values are {:#?}", player); 

    let months: [&str; 12];
    months = ["january", "february", "march", "april", "may", "june", "july",
              "august", "september", "october", "november", "december"];
    println!("the months of the year are {:#?}", months);

    let mut initialized_array = [3;5];
    initialized_array[3] = 7;
    println!("the values of initialized_array are {:#?}", initialized_array);

    let t = ([1; 2], [3; 4]);
    let (a, _) = t;
    println!("{}", a[0] + t.1[0]); 

    let i = 33;
    let j = 31;
    let mut arr = [i, j];
    arr[1] = 32; 
    println!("The value of j is {j} and the value of arr[1] is {}", arr[1]);
    
    //panic example
    // let a = [1, 2, 3, 4, 5];

    // println!("Please enter an array index.");

    // let mut index = String::new();

    // io::stdin()
    //     .read_line(&mut index)
    //     .expect("Failed to read line");

    // let index: usize = index
    //     .trim()
    //     .parse()
    //     .expect("Index entered was not a number");

    // let element = a[index];

    // println!("The value of the element at index {index} is: {element}");

}
