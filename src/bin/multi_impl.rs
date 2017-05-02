struct Man;

trait Walker {
    fn walk(&self) {
        println!("someone walks");
    }
}

trait Thinker {
    fn walk(&self) {
        println!("thinker walks");
    }
}

impl Walker for Man {

}

impl Thinker for Man {

}

/// Problem occurs when a trait provider add a method,
/// having same name as other trait's method.
/// The provider don't know this conflicted trait,
/// so it's user's responsibility to resolve this conflict.
/// Thus, add a method to a trait is no longer a backwards-compatible improvement.
/// Bump a major version for a method addition? It's embarrassing.
fn main() {
    let man = Man;
//    man.walk();
    Walker::walk(&man);
}
