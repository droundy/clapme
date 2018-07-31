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


/// The parameters needed to configure a square well system.
#[derive(Debug, ClapMe)]
struct Vector3d<T> {
    x: T,
    y: T,
    z: T,
}

/// A description of the cell dimensions
#[derive(Debug, ClapMe)]
#[allow(non_snake_case)]
enum CellDimensions {
    /// The three widths of the cell
    CellWidth(Vector3d<f64>),
    /// The volume of the cell
    CellVolume(f64),
}

/// The parameters needed to configure a square well system.
#[derive(Debug, ClapMe)]
struct SquareWellParams {
    well_width: f64,
    _dim: CellDimensions,
}

#[derive(Debug, ClapMe)]
enum Params<MP, SP> {
    ResumeFrom(::std::path::PathBuf),
    _Params {
        _sys: SP,
        _mc: MP,
    },
}

#[allow(non_snake_case)]
#[derive(Debug, ClapMe)]
struct SadParams {
    /// The minimum temperature we are interested in.
    min_T: f64,
    /// The seed for the random number generator.
    seed: Option<u64>,
}

#[test]
fn craziness() {
    type P = Params<SadParams,SquareWellParams>;
    println!("help: {}", P::help_message("foo"));
    assert!(P::help_message("foo").contains("--resume-from "));
}
