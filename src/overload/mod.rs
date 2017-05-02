mod impls;

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

pub fn log<A: Show>(a: A) {
    println!("{}", a.show());
}
