use self::NumberOrNothing::{Nothing, Number};

enum NumberOrNothing {
    Number(i32),
    Nothing,
}

fn vec_min(vec: Vec<i32>) -> NumberOrNothing {
    let mut min = Nothing;
    for el in vec {
        match min {
            Nothing => min = Number(el),
            Number(n) => {
                let new_min = min_i32(n, el);
                min = Number(new_min)
            }
        }
    }
    return min;
}

fn min_i32(a: i32, b: i32) -> i32 {
    if a < b {
        return a;
    } else {
        return b;
    }
}

fn read_vec() -> Vec<i32> {
    vec![10, 12, 5, 78, 9, 2, 11]
}

fn print_number_or_nothing(n: NumberOrNothing) {
    match n {
        Nothing => println!("The number is <nothing>"),
        Number(el) => println!("The number is {}", el),
    };
}

pub fn main() {
    let vec = read_vec();
    let min = vec_min(vec);
    print_number_or_nothing(min);

    let min2 = vec_min(vec![]);
    print_number_or_nothing(min2);
}
