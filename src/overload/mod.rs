pub fn overload() {
    println!("Hello, overload!");
}

#[derive(Clone)]
pub struct Base;

#[derive(Clone)]
pub struct Derived;

pub trait Show {
    fn show(&self) -> String;
}

impl <'a> Show for &'a str {
    fn show(&self) -> String {
        self.to_string()
    }
}

impl Show for Base {
    fn show(&self) -> String {
        "base".to_string()
    }
}

impl Show for Derived {
    fn show(&self) -> String {
        "derived".to_string()
    }
}

impl <T: Show> Show for Vec<T> {
    fn show(&self) -> String {
        let mut result = "".to_string();
        for base in self {
            result.push_str(&base.show())
        }
        result
    }
}

pub fn log<A: Show>(a: A) {
    println!("{}", a.show());
}
