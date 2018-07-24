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

#[derive(Debug, ClapMe)]
enum Params {
    ResumeFrom(String),
    _Params {
        sys: SquareWellParams,
    },
}

#[derive(Debug, ClapMe)]
enum Dimensions {
    /// The three widths of the cell
    Width(String),
    /// The volume of the cell
    Volume(String),
}

#[derive(Debug, ClapMe)]
struct SquareWellParams {
    well_width: String,
    cell: Dimensions,
}


#[test]
fn craziness() {
    println!("help: {}", Params::help_message("foo"));
    assert!(Params::help_message("foo").contains("--resume-from "));
}
