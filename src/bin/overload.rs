extern crate playground;

use playground::overload::{log, Base, Derived};

fn main() {
    log("test");
    log(Base);
    log(Derived);
    let base_vec = vec![Base; 3];
    log(base_vec);
    let derived_vec = vec![Derived; 3];
    log(derived_vec);
}
