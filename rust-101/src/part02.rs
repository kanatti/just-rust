pub enum SomethingOrNothing<T> {
    Something(T),
    Nothing,
}

pub use self::SomethingOrNothing::*;
type NumberOrNothing = SomethingOrNothing<i32>;

impl<T> SomethingOrNothing<T> {
    fn new(o: Option<T>) -> Self {
        match o {
            None => Nothing,
            Some(t) => Something(t),
        }
    }

    fn to_option(self) -> Option<T> {
        match self {
            Nothing => None,
            Something(t) => Some(t),
        }
    }
}

fn wrap(x: i32) -> SomethingOrNothing<i32> {
    SomethingOrNothing::new(Some(x))
}

pub trait Minimum: Copy {
    fn min(self, b: Self) -> Self;
}

pub fn vec_min<T: Minimum>(v: Vec<T>) -> SomethingOrNothing<T> {
    let mut min = Nothing;
    for e in v {
        min = Something(match min {
            Nothing => e,
            Something(t) => e.min(t),
        });
    }
    min
}

impl Minimum for i32 {
    fn min(self, b: Self) -> Self {
        if self < b {
            self
        } else {
            b
        }
    }
}

impl NumberOrNothing {
    fn print(self) {
        match self {
            Nothing => println!("The number is <nothing>"),
            Something(n) => println!("The number is {}", n),
        }
    }
}

fn read_vec() -> Vec<i32> {
    vec![18, 5, 7, 3, 9, 27]
}

pub fn main() {
    let vec = read_vec();
    let min = vec_min(vec);
    min.print()
}
