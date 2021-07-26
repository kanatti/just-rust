use std::io;
use std::io::prelude::*;
use std::str::FromStr;

fn read_vec<T: FromStr>() -> Vec<T> {
    let mut vec: Vec<T> = Vec::new();
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let line = line.unwrap();

        match line.trim().parse::<T>() {
            Ok(t) => vec.push(t),
            Err(_) => {
                println!("Cannot parse")
            }
        };
    }
    vec
}

use part02::{vec_min, Nothing, Something, SomethingOrNothing};

pub trait Print {
    fn print2(self);
}

impl<T: Print> SomethingOrNothing<T> {
    pub fn print2(self) {
        match self {
            Nothing => print!("The value is Nothing"),
            Something(t) => t.print2(),
        }
    }
}

impl Print for i32 {
    fn print2(self) {
        println!("The number is {} !!", self);
    }
}

impl Print for f32 {
    fn print2(self) {
        println!("The float is {:.2}", self)
    }
}

pub fn main() {
    println!("Enter Numbers");
    let vec: Vec<i32> = read_vec::<i32>();
    let min = vec_min(vec);
    min.print2();

    println!("Enter Floats");
    let vec: Vec<f32> = read_vec::<f32>();
    let min = vec_min(vec);
    min.print2();
}
