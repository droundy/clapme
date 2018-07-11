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
    enum EnumOpt {
        First { first: String },
        Second { second: i32 },
        Third { third: u16 },
    }
    println!("help: {}", EnumOpt::help_message("foo"));
    assert!(EnumOpt::help_message("foo").contains("--first-first "));
    assert!(EnumOpt::help_message("foo").contains("--second-second "));
    assert!(EnumOpt::help_message("foo").contains("--third-third "));

    assert_eq!(
        EnumOpt::First { first: "hello".to_string() },
        EnumOpt::from_iter(&["", "--first-first", "hello"]).unwrap());

    assert_eq!(
        EnumOpt::Second { second: 5 },
        EnumOpt::from_iter(&["", "--second-second", "5"]).unwrap());

    assert!(EnumOpt::from_iter(&[""]).is_err());

    assert!(EnumOpt::from_iter(&["", "--first-first", "hello",
                                 "--second-second", "5"]).is_err());
}

#[test]
fn unit_enum() {
    #[derive(ClapMe, PartialEq, Debug)]
    enum EnumOpt {
        First,
        Second,
        Third,
    }
    println!("help: {}", EnumOpt::help_message("foo"));
    assert!(EnumOpt::help_message("foo").contains("--first "));
    assert!(EnumOpt::help_message("foo").contains("--second "));
    assert!(EnumOpt::help_message("foo").contains("--third "));

    assert_eq!(
        EnumOpt::First,
        EnumOpt::from_iter(&["", "--first"]).unwrap());

    assert_eq!(
        EnumOpt::Second,
        EnumOpt::from_iter(&["", "--second"]).unwrap());

    assert!(EnumOpt::from_iter(&[""]).is_err());

    assert!(EnumOpt::from_iter(&["", "--first", "--second"]).is_err());
}

#[test]
fn unit_enum_with_underscores() {
    #[derive(ClapMe, PartialEq, Debug)]
    enum EnumOpt {
        First_World,
        Second,
        T_,
    }
    println!("help: {}", EnumOpt::help_message("foo"));
    assert!(EnumOpt::help_message("foo").contains("--First-World "));
    assert!(EnumOpt::help_message("foo").contains("--second "));
    assert!(EnumOpt::help_message("foo").contains("--T "));

    assert_eq!(
        EnumOpt::First_World,
        EnumOpt::from_iter(&["", "--First-World"]).unwrap());

    assert_eq!(
        EnumOpt::Second,
        EnumOpt::from_iter(&["", "--second"]).unwrap());

    assert!(EnumOpt::from_iter(&[""]).is_err());

    assert!(EnumOpt::from_iter(&["", "--first", "--second"]).is_err());
}
