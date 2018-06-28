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
fn unique_flag() {
    #[derive(ClapMe, PartialEq, Debug)]
    struct Opt {
        /// Documentation for alice
        alice: bool,
    }

    assert!(
        Opt::test_help().contains("--alice")
    );
    assert!(
        Opt::test_help().contains("Documentation for alice")
    );

    assert_eq!(
        Opt { alice: true },
        Opt::parse_from(&["", "--alice"]).unwrap());

    assert_eq!(
        Opt { alice: false },
        Opt::parse_from(&[""]).unwrap());

    assert!(Opt::parse_from(&["", "--bob"]).is_err());
}
