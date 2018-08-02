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

#[derive(ClapMe, PartialEq, Debug)]
struct Unit;

#[test]
fn unit_struct() {
    println!("help: {}", Unit::help_message("foo"));
    assert!(!Unit::help_message("foo").contains("--first"));

    assert_eq!(Unit,
               Unit::from_iter(&[""]).unwrap());
}

#[test]
fn struct_with_unit() {
    #[derive(ClapMe, PartialEq, Debug)]
    struct Opt {
        first: i32,
        second: Unit,
    };
    println!("help: {}", Opt::help_message("foo"));
    assert!(Opt::help_message("foo").contains("--first"));
    assert!(!Opt::help_message("foo").contains("--second"));

    assert_eq!(Opt{ first: 7, second: Unit },
               Opt::from_iter(&["","--first=7"]).unwrap());

    assert_eq!(None,
               Opt::from_iter(&["","--first=7", "--second"]).ok());

    assert!(Opt::from_iter(&[""]).is_err());

    assert!(Opt::from_iter(&["hello"]).is_err());

    assert!(Opt::from_iter(&["--first"]).is_err());
}
