// Copyright 2018 David Roundy <roundyd@physics.oregonstate.edu>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

extern crate clapme;

use clapme::ClapMe;

#[derive(Debug, ClapMe, PartialEq, Eq)]
struct SubOpt {
    max_iter: Option<u64>,
    quiet: bool,
}

#[derive(Debug, ClapMe, PartialEq, Eq)]
struct Opt {
    foo: String,
    _report: SubOpt,
}

#[derive(Debug, ClapMe, PartialEq, Eq)]
enum Parent {
    CaseOne,
    _CaseTwo(Opt),
}

#[test]
fn craziness() {
    println!("help: {}", Parent::help_message("foo"));
    assert!(Parent::help_message("foo").contains("--quiet "));
    assert_eq!(
        Parent::_CaseTwo(Opt {
            foo: "foo".to_string(),
            _report: SubOpt {
                max_iter: None,
                quiet: false,
            }
        }),
        Parent::from_iter(&["", "--foo=foo"]).unwrap()
    );
}
