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
fn simple_generic() {
    #[derive(ClapMe, PartialEq, Debug)]
    struct GenericOpt<T> {
        first: T,
        second: String,
    }
    println!("help: {}", <GenericOpt<i32>>::help_message("foo"));
    assert!(<GenericOpt<i32>>::help_message("foo").contains("--first"));
    assert!(<GenericOpt<i32>>::help_message("foo").contains("--second"));

    assert_eq!(
        GenericOpt::<i32> { first: 3, second: "hello".to_string() },
        <GenericOpt<i32>>::from_iter(&["", "--first", "3","--second=hello"]).unwrap());

    assert!(<GenericOpt<i32>>::from_iter(&[""]).is_err());
}

#[test]
fn optional_generic() {
    #[derive(ClapMe, PartialEq, Debug)]
    struct GenericOpt<T> {
        first: Option<T>,
        second: String,
    }
    println!("help: {}", <GenericOpt<i32>>::help_message("foo"));
    assert!(<GenericOpt<i32>>::help_message("foo").contains("--first"));
    assert!(<GenericOpt<i32>>::help_message("foo").contains("--second"));

    assert_eq!(
        GenericOpt::<i32> { first: Some(3), second: "hello".to_string() },
        <GenericOpt<i32>>::from_iter(&["", "--first", "3","--second=hello"]).unwrap());

    assert_eq!(
        GenericOpt::<i32> { first: None, second: "hello".to_string() },
        <GenericOpt<i32>>::from_iter(&["", "--second=hello"]).unwrap());

    assert!(<GenericOpt<i32>>::from_iter(&[""]).is_err());
}
