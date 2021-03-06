trait A {
    fn a(&self) -> i32;
}

trait B {
    fn b(&self) -> f32;
}

trait AB : A + B {}

struct Impl;

impl A for Impl {
    fn a(&self) -> i32 {
        return 1;
    }
}

impl B for Impl {
    fn b(&self) -> f32 {
        return 1.0;
    }
}

impl AB for Impl {}

fn print(ab: &AB) {
    println!("{}", ab.a());
    println!("{}", ab.b());

    // can't cast a trait object to another trait object
    // since vtable is constructed in compile time using actual type of object
    // println!("{}", (ab as &B).b());
}

fn main() {
    let x = Impl;
    print(&x);
    println!("{}", (&x as &B).b());
}