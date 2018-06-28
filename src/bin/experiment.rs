#[macro_use]
extern crate clapme;

use clapme::ClapMe;

#[derive(ClapMe)]
struct HelloWorld {
    /// Should we or should we not greet?
    hello: bool,
    /// Our oyster!
    world: u16,
}

fn main() {
    let opts = HelloWorld::parse_args();
    println!("hello is {}", opts.hello);
    println!("world is {}", opts.world);
}
