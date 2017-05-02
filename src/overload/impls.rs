use super::{Show, Base, Derived};

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
