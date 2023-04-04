use std::rc::Rc;

fn main() {
    let s = return_a_string();
    println!("{}", *s);
}

fn return_a_string() -> Rc<String> {
    here_is_the_string()
}

fn here_is_the_string() -> Rc<String> {
    let s = Rc::new(String::from("Hello World"));
    Rc::clone(&s)
}
