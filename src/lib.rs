// Copyright 2018 David Roundy <roundyd@physics.oregonstate.edu>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#![deny(missing_docs)]

//! This crate defines the `ClapMe` trait and its custom derrive.

extern crate clap as _clap;

#[allow(unused_imports)]
#[macro_use]
extern crate opt_derive;

#[doc(hidden)]
pub use opt_derive::*;

use std::str::FromStr;
use std::ffi::OsString;

/// Re-export of clap
pub mod clap {
    pub use _clap::*;
}

/// Information needed to set up an argument.
#[derive(Clone, Copy)]
pub struct ArgInfo<'a> {
    /// The name of the argument, which is also its long flag.
    pub name: &'a str,
    /// Is the argument required?
    pub required: bool,
    /// Can we repeat the flag?
    pub multiple: bool,
}

impl<'a> ArgInfo<'a> {
    /// A new `ArgInfo` with sensible defaults.
    pub fn new(name: &'a str) -> Self {
        ArgInfo {
            name: name,
            required: true,
            multiple: false,
        }
    }
}

/// Any type of trait `ClapMe` can be used as an argument value.
pub trait ClapMe : Sized {
    /// Updates and returns the corresponding `clap::App`.
    fn augment_clap<'a, 'b>(_info: ArgInfo<'a>,
                            app: clap::App<'a,'b>) -> clap::App<'a,'b> {
        app
    }
    /// Parses the clap info to obtain a value.  `None` is returned if
    /// the argument was not required, and was also not provided.
    fn from_clap<'a,'b>(_name: &str, _app: &clap::ArgMatches) -> Option<Self> {
        None
    }
    /// Test the help message
    fn test_help() -> String {
        let info = ArgInfo::new("");
        let mut help_data = Vec::new();
        Self::augment_clap(info, clap::App::new("foo")).write_help(&mut help_data).unwrap();
        String::from_utf8_lossy(&help_data).into_owned()
    }

    /// Parse command line arguments.
    fn parse_args() -> Self {
        let matches = Self::augment_clap(ArgInfo::new(""),
                                         clap::App::new("foo")).get_matches();
        Self::from_clap("", &matches).unwrap()
    }

    /// Parse command line arguments.
    fn parse_from<I,T>(args: I) -> Result<Self, clap::Error>
        where
        I: IntoIterator<Item = T>,
        T: Into<OsString> + Clone,
    {
        let matches =
            Self::augment_clap(ArgInfo::new(""),
                               clap::App::new("foo")).get_matches_from_safe(args)?;
        Ok(Self::from_clap("", &matches).unwrap())
    }
}

impl ClapMe for bool {
    fn augment_clap<'a, 'b>(info: ArgInfo<'a>, app: clap::App<'a,'b>) -> clap::App<'a,'b> {
        app.arg(clap::Arg::with_name(info.name).long(info.name))
    }
    fn from_clap(name: &str, matches: &clap::ArgMatches) -> Option<Self> {
        Some(matches.is_present(name))
    }
}

macro_rules! impl_fromstr {
    ($t:ty) => {
        impl ClapMe for $t {
            fn augment_clap<'a, 'b>(info: ArgInfo<'a>, app: clap::App<'a,'b>) -> clap::App<'a,'b> {
                app.arg(clap::Arg::with_name(info.name)
                        .long(info.name)
                        .takes_value(true)
                        .required(info.required)
                        .validator(|s| Self::from_str(&s).map(|_| ()).map_err(|e| e.to_string())))
            }
            fn from_clap(name: &str, matches: &clap::ArgMatches) -> Option<Self> {
                matches.value_of(name).map(|s| Self::from_str(s).unwrap())
            }
        }
    }
}

impl_fromstr!(isize);
impl_fromstr!(i16);
impl_fromstr!(i32);
impl_fromstr!(i64);
impl_fromstr!(i128);
impl_fromstr!(u16);
impl_fromstr!(u32);
impl_fromstr!(u64);
impl_fromstr!(u128);
impl_fromstr!(f32);
impl_fromstr!(f64);

impl<T: ClapMe> ClapMe for Option<T> {
    fn augment_clap<'a, 'b>(mut info: ArgInfo<'a>, app: clap::App<'a,'b>) -> clap::App<'a,'b> {
        info.required = false;
        T::augment_clap(info, app)
    }
    fn from_clap(name: &str, matches: &clap::ArgMatches) -> Option<Self> {
        Some(T::from_clap(name, matches))
    }
}

impl<T> ClapMe for Vec<T> where T: FromStr, <T as FromStr>::Err: std::fmt::Debug {
    fn augment_clap<'a, 'b>(info: ArgInfo<'a>, app: clap::App<'a,'b>) -> clap::App<'a,'b> {
        app.arg(clap::Arg::with_name(info.name)
                .long(info.name)
                .takes_value(true)
                .required(false)
                .multiple(true)
                .validator(|s| T::from_str(&s).map(|_| ()).map_err(|_| "oops".to_owned())))
    }
    fn from_clap(name: &str, matches: &clap::ArgMatches) -> Option<Self> {
        Some(matches.values_of(name).unwrap_or(clap::Values::default())
             .map(|s| T::from_str(s).unwrap()).collect())
    }
}
