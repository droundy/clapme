// Copyright 2018 David Roundy <roundyd@physics.oregonstate.edu>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#[macro_use]
extern crate clapme;

use clapme::ClapMe;

#[test]
fn simple_enum() {
    #[derive(ClapMe, PartialEq, Debug)]
    enum Opt {
        First { first: String },
        Second { second: i32 },
        Third { third: u16 },
    }
    println!("help: {}", Opt::help_message("foo"));
    assert!(Opt::help_message("foo").contains("--first"));
    assert!(Opt::help_message("foo").contains("--second"));
    assert!(Opt::help_message("foo").contains("--third"));

    assert_eq!(
        Opt::First { first: "hello".to_string() },
        Opt::from_iter(&["", "--first", "hello"]).unwrap());

    assert_eq!(
        Opt::Second { second: 5 },
        Opt::from_iter(&["", "--second", "5"]).unwrap());

    assert!(Opt::from_iter(&[""]).is_err());

    assert!(Opt::from_iter(&["", "--first", "hello", "--second", "5"]).is_err());
}
