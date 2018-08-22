// Copyright 2018 David Roundy <roundyd@physics.oregonstate.edu>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

extern crate clapme;

use clapme::ClapMe;

#[test]
fn simple_u64() {
    println!("help: {}", u64::help_message("foo"));
    assert!(!u64::help_message("foo").contains("--first"));

    assert_eq!(7,
               u64::from_iter(&["","7"]).unwrap());

    assert_eq!(7,
               u64::from_iter(&["","7e0"]).unwrap());
    assert_eq!(1000000,
               u64::from_iter(&["","1e6"]).unwrap());

    assert!(u64::from_iter(&[""]).is_err());

    assert!(u64::from_iter(&["hello"]).is_err());
}

#[test]
fn simple_string() {
    println!("help: {}", String::help_message("foo"));
    assert!(!String::help_message("foo").contains("--first"));

    assert_eq!("7".to_string(),
               String::from_iter(&["","7"]).unwrap());

    assert!(String::from_iter(&[""]).is_err());
}

#[test]
fn simple_option_string() {
    println!("help: {}", <Option<String>>::help_message("foo"));
    assert!(!<Option<String>>::help_message("foo").contains("--first"));

    assert_eq!(Some("7".to_string()),
               <Option<String>>::from_iter(&["","7"]).unwrap());

    assert_eq!(None,
               <Option<String>>::from_iter(&[""]).unwrap());
}

#[test]
fn simple_option_vec_i16() {
    println!("help: {}", <Vec<i16>>::help_message("vec"));
    assert!(!<Vec<i16>>::help_message("vec").contains("--first"));

    assert_eq!(vec![7],
               <Vec<i16>>::from_iter(&["","7"]).unwrap());

    assert_eq!(Vec::<i16>::new(),
               <Vec<i16>>::from_iter(&[""]).unwrap());
}
