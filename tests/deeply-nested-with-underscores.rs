// Copyright 2018 David Roundy <roundyd@physics.oregonstate.edu>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

extern crate clapme;

use clapme::ClapMe;

/// The parameters needed to configure a square well system.
#[derive(PartialEq, Debug, ClapMe)]
struct Vector3d<T> {
    x: T,
    y: T,
    z: T,
}

/// A description of the cell dimensions
#[derive(PartialEq, Debug, ClapMe)]
#[allow(non_snake_case)]
enum CellDimensions {
    /// The three widths of the cell
    CellWidth(Vector3d<f64>),
    /// The volume of the cell
    CellVolume(f64),
}

/// The parameters needed to configure a square well system.
#[derive(PartialEq, Debug, ClapMe)]
struct SquareWellParams {
    well_width: f64,
    _dim: CellDimensions,
}

#[allow(non_snake_case)]
#[derive(PartialEq, Debug, ClapMe)]
struct SadParams {
    /// The minimum temperature we are interested in.
    min_T: f64,
    /// The seed for the random number generator.
    seed: Option<u64>,
}

#[derive(PartialEq, Debug, ClapMe)]
enum Params<MP, SP> {
    ResumeFrom(String),
    _Params { _sys: SP, _mc: MP },
}

#[derive(PartialEq, Debug, ClapMe)]
struct Simple {
    simple: u64,
}

#[derive(PartialEq, Debug, ClapMe)]
struct Naive {
    naive: String,
}

#[test]
fn craziness() {
    type P = Params<SadParams, SquareWellParams>;
    println!("help: {}", P::help_message("foo"));
    println!("\n\n\n\n");
    assert!(P::help_message("foo").contains("--resume-from "));
    assert!(P::help_message("foo").contains("--well-width "));
    assert!(P::help_message("foo").contains("--cell-volume "));
    assert!(P::help_message("foo").contains("--cell-width-x "));
    assert!(!P::help_message("foo").contains("--dim- "));

    assert_eq!(
        Params::ResumeFrom::<SadParams, SquareWellParams>("hello".to_string()),
        P::from_iter(&["", "--resume-from", "hello"]).unwrap()
    );

    assert_eq!(
        Params::ResumeFrom::<Naive, Simple>("hello".to_string()),
        Params::<Naive, Simple>::from_iter(&["", "--resume-from", "hello"]).unwrap()
    );

    assert_eq!(
        Params::_Params::<Naive, Simple> {
            _sys: Simple { simple: 37 },
            _mc: Naive {
                naive: "goodbye".to_string(),
            },
        },
        Params::<Naive, Simple>::from_iter(&["", "--simple", "37", "--naive", "goodbye"]).unwrap()
    );

    assert_eq!(
        Params::_Params::<SadParams, Simple> {
            _sys: Simple { simple: 137 },
            _mc: SadParams {
                min_T: 0.2,
                seed: None,
            },
        },
        Params::<SadParams, Simple>::from_iter(&["", "--simple", "137", "--min-T", "0.2"]).unwrap()
    );

    assert_eq!(
        Params::_Params::<SadParams, SquareWellParams> {
            _sys: SquareWellParams {
                well_width: 1.3,
                _dim: CellDimensions::CellVolume(5.0),
            },
            _mc: SadParams {
                min_T: 0.2,
                seed: None,
            },
        },
        P::from_iter(&[
            "",
            "--well-width",
            "1.3",
            "--cell-volume",
            "5",
            "--min-T",
            "0.2"
        ])
        .unwrap()
    );
}
