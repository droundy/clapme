#[macro_use]
extern crate clapme;

use clapme::ClapMe;

#[derive(ClapMe, PartialEq, Debug)]
struct Opt {
    arg: i32,
}

#[derive(ClapMe)]
struct HelloWorld {
    /// Should we or should we not greet?
    hello: bool,
    /// Our oyster!
    world: Opt,
}

fn main() {
    let opts = HelloWorld::from_args();
    println!("hello is {}", opts.hello);
    println!("world is {:?}", opts.world);
}
