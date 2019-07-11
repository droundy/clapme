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
    assert!(
        Opt::help_message("foo").contains("--arg")
    );

    assert_eq!(
        Opt { arg: 7 },
        Opt::from_iter(&["", "--arg", "7"]).unwrap());

    assert!(Opt::from_iter(&["", "--arg"]).is_err());
}
