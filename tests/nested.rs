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
        other: String,
    }
    println!("help: {}", SuperOpt::test_help());
    assert!(SuperOpt::test_help().contains("--arg"));
    assert!(SuperOpt::test_help().contains("--arg-arg"));

    assert_eq!(
        SuperOpt { arg: Opt { arg: 7 }, other: "hello".to_string() },
        SuperOpt::parse_from(&["", "--arg-arg", "7", "--other", "hello"]).unwrap());

    assert!(SuperOpt::parse_from(&["", "--arg"]).is_err());
}

#[test]
fn required_option_with_flattened_name() {
    #[derive(ClapMe, PartialEq, Debug)]
    struct Opt {
        arg: i32,
    }
    #[derive(ClapMe, PartialEq, Debug)]
    struct SuperOpt {
        _arg: Opt,
        other: String,
    }
    println!("help: {}", SuperOpt::test_help());
    assert!(SuperOpt::test_help().contains("--arg "));

    assert_eq!(
        SuperOpt { _arg: Opt { arg: 7 }, other: "hello".to_string() },
        SuperOpt::parse_from(&["", "--arg", "7", "--other", "hello"]).unwrap());
}

#[test]
fn optional_option() {
    #[derive(ClapMe, PartialEq, Debug)]
    struct Foo {
        arg1: u32,
        arg2: i32,
    }
    #[derive(ClapMe, PartialEq, Debug)]
    struct SuperOpt {
        _arg: Option<Foo>,
        other: String,
    }
    println!("help: {}", SuperOpt::test_help());
    assert!(SuperOpt::test_help().contains("--arg1 "));
    assert!(SuperOpt::test_help().contains("--arg2 "));

    assert_eq!(
        SuperOpt { _arg: Some(Foo { arg1: 37, arg2: -3 }), other: "hello".to_string() },
        SuperOpt::parse_from(&["", "--arg1", "37", "--arg2=-3",
                               "--other", "hello"]).unwrap());

    assert_eq!(
        SuperOpt { _arg: None, other: "hello".to_string() },
        SuperOpt::parse_from(&["", "--other", "hello"]).unwrap());

    assert!(SuperOpt::parse_from(&["", "--arg1", "7", "--other", "hello"]).is_err());

    assert!(SuperOpt::parse_from(&["", "--arg2", "7", "--other", "hello"]).is_err());
}
