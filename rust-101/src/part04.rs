use std::fmt;

fn vec_min<T: Ord + Copy>(v: &Vec<T>) -> Option<T> {
    use std::cmp;

    let mut min = None;
    for e in v.iter() {
        min = Some(match min {
            None => *e,
            Some(n) => cmp::min(n, *e),
        });
    }
    min
}

fn vec_inc(v: &mut Vec<i32>) {
    for e in v.iter_mut() {
        *e = *e + 1;
    }
}

fn print_option<T: fmt::Display>(o: Option<T>) {
    match o {
        None => println!("Nothing"),
        Some(t) => println!("Some({})", t),
    }
}

fn shared_ref_demo() {
    let v: Vec<i32> = vec![5, 4, 3, 2, 1];
    let first = &v[0];
    let min1 = vec_min(&v);
    let min2 = vec_min(&v);
    println!("The first element is: {}", *first);
    print!("The min1 is ");
    print_option(min1);
    print!("The min2 is ");
    print_option(min2);
}

fn mutable_ref_demo() {
    let mut v: Vec<i32> = vec![5, 4, 3, 2, 1];
    // let first = &v[0]; // -> This errors
    vec_inc(&mut v);
    vec_inc(&mut v);
    let first = &v[0];
    println!("The first element is: {}", *first);
}

pub fn main() {
    shared_ref_demo();
    mutable_ref_demo();
}
