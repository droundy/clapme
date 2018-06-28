#[macro_use]
extern crate clapme;

use clapme::ClapMe;

#[derive(ClapMe)]
struct HelloWorld {
    /// Should we or should we not greet?
    hello: bool,
    world: u16,
}

fn main() {
    println!("hello world!");
    println!("help: {}", HelloWorld::test_help());
}
