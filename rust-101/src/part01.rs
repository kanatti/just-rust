use self::NumberOrNothing::{Nothing, Number};

fn sqr(i: i32) -> i32 {
    i * i
}

fn abs(i: i32) -> i32 {
    if i >= 0 {
        i
    } else {
        -i
    }
}

fn min_i32(a: i32, b: i32) -> i32 {
    if a < b {
        a
    } else {
        b
    }
}

enum NumberOrNothing {
    Number(i32),
    Nothing,
}

impl NumberOrNothing {
    fn print(self) {
        match self {
            Nothing => println!("The Number is <Nothing>"),
            Number(n) => println!("the number is {}", n),
        };
    }
}

fn number_or_default(n: NumberOrNothing, default: i32) -> i32 {
    match n {
        Number(n) => n,
        Nothing => default,
    }
}

fn vec_min(vec: Vec<i32>) -> NumberOrNothing {
    let mut min = Nothing;
    for e in vec {
        min = Number(match min {
            Nothing => e,
            Number(n) => min_i32(n, e),
        });
    }
    min
}

fn vec_sum(vec: Vec<i32>) -> NumberOrNothing {
    let mut sum = Nothing;
    for e in vec {
        sum = Number(match sum {
            Nothing => e,
            Number(n) => n + e,
        })
    }
    sum
}

fn vec_print(vec: Vec<i32>) {
    for e in vec {
        println!("Item is {}", e)
    }
}

pub fn main() {
    let min = vec_min(vec![10, 12, 5, 78, 9, 2, 11]);
    min.print();

    let sum = vec_sum(vec![1, 2, 3, 4, 5, 6]);
    sum.print();
}
