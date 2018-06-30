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
fn required_option() {
    #[derive(ClapMe, PartialEq, Debug)]
    struct Opt {
        arg: i32,
    }
    #[derive(ClapMe, PartialEq, Debug)]
    struct SuperOpt {
        arg: Opt,
        other: i32
    }
    println!("help: {}", SuperOpt::test_help());
    assert!(SuperOpt::test_help().contains("--arg"));
    assert!(SuperOpt::test_help().contains("--arg-arg"));

    assert_eq!(
        SuperOpt { arg: Opt { arg: 7 }, other: 0 },
        SuperOpt::parse_from(&["", "--arg-arg", "7", "--other", "0"]).unwrap());

    assert!(SuperOpt::parse_from(&["", "--arg"]).is_err());
}
