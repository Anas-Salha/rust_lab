fn main() {
    let x: Box<i32> = Box::new(-1);
    let x_abs1 = i32::abs(*x); // explicit dereference
    let x_abs2 = x.abs(); // implicit dereference
    assert_eq!(x_abs1, x_abs2);

    let r: &Box<i32> = &x;
    let r_abs1 = i32::abs(**r); // explicit dereference (twice)
    let r_abs2 = r.abs(); // implicit dereference (twice)
    assert_eq!(r_abs1, r_abs2);

    let s = String::from("Hello");
    let s_len1 = str::len(&s); // explicit reference
    let s_len2 = s.len(); // implicit reference
    assert_eq!(s_len1, s_len2);

    let mut v = vec![0, 1, 2];
    let n = get_first(&v);
    println!("{} {}", n, v[1]);

    let mut vec: Vec<i32> = vec![1, 2, 3];
    let num: &i32 = &vec[2];
    println!("Third element is {}", *num);
    vec.push(4);

    let mut vec = vec![1, 2, 3];
    let num = &mut vec[2];
    //vec[1] += 1; // can't mutate ANY element in vec while num refers to one element
    *num += 1;
    println!("Third element is {}", *num);
    println!("Vector is now {:?}", vec);

    let mut vec: Vec<i32> = vec![1, 2, 3];
    let num: &mut i32 = &mut vec[2];
    let num2: &i32 = &*num; // By creating a new alias, num is "downgraded" to read only
    println!("{} {}", *num, *num2);
    *num += 1; // Since num2 is no longer used, num is regains it's write rights
    vec.push(3);
    //println!("{}", *num2); // Adding this print means num2 is still in use -> invalidates the previous 2 lines

    let s = String::from("Hello world");
    let s_ref = &s;
    //drop(s); cannot drop s while it's borrowed
    println!("{}", s_ref);
}

fn get_first(vr: &Vec<i32>) -> i32 {
    vr[0]
}
