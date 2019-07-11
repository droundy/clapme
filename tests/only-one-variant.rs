#[macro_use]
extern crate clapme;

use clapme::ClapMe;

#[derive(ClapMe, PartialEq, Debug)]
enum Exclusive {
    First {
        a: String,
    },
}

#[test]
fn craziness() {
    println!("help: {}", Exclusive::help_message("foo"));
    println!("\n\n\n\n");
    assert!(Exclusive::help_message("foo").contains("--first-a "));

    assert!(Exclusive::help_message("foo").contains("--first-a "));

    assert!(Exclusive::from_iter(&[""]).is_err());
}
