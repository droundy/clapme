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
    println!("help: {}", SuperOpt::help_message("foo"));
    assert!(SuperOpt::help_message("foo").contains("--arg"));
    assert!(SuperOpt::help_message("foo").contains("--arg-arg"));

    assert_eq!(
        SuperOpt { arg: Opt { arg: 7 }, other: "hello".to_string() },
        SuperOpt::from_iter(&["", "--arg-arg", "7", "--other", "hello"]).unwrap());

    assert!(SuperOpt::from_iter(&["", "--arg"]).is_err());
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
    println!("help: {}", SuperOpt::help_message("foo"));
    assert!(SuperOpt::help_message("foo").contains("--arg "));

    assert_eq!(
        SuperOpt { _arg: Opt { arg: 7 }, other: "hello".to_string() },
        SuperOpt::from_iter(&["", "--arg", "7", "--other", "hello"]).unwrap());
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
    println!("help: {}", SuperOpt::help_message("foo"));
    assert!(SuperOpt::help_message("foo").contains("--arg1 "));
    assert!(SuperOpt::help_message("foo").contains("--arg2 "));

    assert_eq!(
        SuperOpt { _arg: Some(Foo { arg1: 37, arg2: -3 }), other: "hello".to_string() },
        SuperOpt::from_iter(&["", "--arg1", "37", "--arg2=-3",
                               "--other", "hello"]).unwrap());

    assert_eq!(
        SuperOpt { _arg: None, other: "hello".to_string() },
        SuperOpt::from_iter(&["", "--other", "hello"]).unwrap());

    assert!(SuperOpt::from_iter(&["", "--arg1", "7", "--other", "hello"]).is_err());

    assert!(SuperOpt::from_iter(&["", "--arg2", "7", "--other", "hello"]).is_err());
}
