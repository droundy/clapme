// Copyright 2018 David Roundy <roundyd@physics.oregonstate.edu>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#![deny(missing_docs)]

//! This crate defines the `ClapMe` trait and its custom derrive.


//! ## How to `derive(ClapMe)`
//!
//! First, let's look at an example:
//!
//! ```should_panic
//! #[macro_use]
//! extern crate clapme;
//!
//! use std::path::PathBuf;
//! use clapme::ClapMe;
//!
//! #[derive(Debug, ClapMe)]
//! struct Opt {
//!     /// Activate debug mode
//!     debug: bool,
//!     /// Set speed
//!     speed: f64,
//!     /// Input file
//!     input: PathBuf,
//!     /// Output file, stdout if not present
//!     output: Option<PathBuf>,
//! }
//!
//! fn main() {
//!     let opt = Opt::parse_args();
//!     println!("{:?}", opt);
//! }
//! ```

extern crate clap as _clap;

#[allow(unused_imports)]
#[macro_use]
extern crate clapme_derive;

#[doc(hidden)]
pub use clapme_derive::*;

use std::str::FromStr;
use std::ffi::OsString;

/// Re-export of clap
pub mod clap {
    pub use _clap::*;
}

/// Information needed to set up an argument.
#[derive(Clone)]
pub struct ArgInfo<'a> {
    /// The name of the argument, which is also its long flag.
    pub name: &'a str,
    /// Is the argument required?
    pub required: bool,
    /// Flags that are required by this argument.
    pub required_flags: &'a [&'a str],
    /// Can we repeat the flag?
    pub multiple: bool,
    /// Help string (if any)
    pub help: &'a str,
}

impl<'a> ArgInfo<'a> {
    /// A new `ArgInfo` with sensible defaults.
    pub fn new(name: &'a str) -> Self {
        ArgInfo {
            name: name,
            required: true,
            required_flags: &[],
            multiple: false,
            help: "",
        }
    }
}

/// Any type of trait `ClapMe` can be used as an argument value.
pub trait ClapMe : Sized {
    /// Updates and returns the corresponding `clap::App`.
    fn with_clap<T>(_info: ArgInfo,
                    app: clap::App,
                    f: impl FnOnce(clap::App) -> T) -> T {
        f(app)
    }
    /// Parses the clap info to obtain a value.  `None` is returned if
    /// the argument was not required, and was also not provided.
    fn from_clap(_name: &str, _app: &clap::ArgMatches) -> Option<Self> {
        None
    }
    /// Parses the clap info to obtain a value.  `None` is returned if
    /// the argument was not required, and was also not provided.
    fn requires_flags(name: &str) -> Vec<String> {
        vec![name.to_string()]
    }
    /// Test the help message
    fn test_help() -> String {
        let info = ArgInfo::new("");
        Self::with_clap(info, clap::App::new("foo"),
                        |a| {
                            let mut help_data = Vec::new();
                            a.write_help(&mut help_data).unwrap();
                            String::from_utf8_lossy(&help_data).into_owned()
                        })
    }

    /// Parse command line arguments.
    fn parse_args() -> Self {
        Self::with_clap(ArgInfo::new(""),
                        clap::App::new("foo"),
                        |a| {
                            let matches = a.get_matches();
                            Self::from_clap("", &matches).unwrap()
                        })
    }

    /// Parse command line arguments.
    fn parse_from<I,T>(args: I) -> Result<Self, clap::Error>
        where
        I: IntoIterator<Item = T>,
        T: Into<OsString> + Clone,
    {
        Self::with_clap(ArgInfo::new(""),
                        clap::App::new("foo"),
                        |a| {
                            let matches = a.get_matches_from_safe(args)?;
                            Ok(Self::from_clap("", &matches).unwrap())
                        })
    }
}

impl ClapMe for bool {
    fn with_clap<T>(info: ArgInfo, app: clap::App,
                    f: impl FnOnce(clap::App) -> T) -> T {
        f(app.arg(clap::Arg::with_name(info.name).long(info.name)
                  .requires_all(info.required_flags)
                  .help(&info.help)))
    }
    fn from_clap(name: &str, matches: &clap::ArgMatches) -> Option<Self> {
        Some(matches.is_present(name))
    }
    fn requires_flags(_name: &str) -> Vec<String> {
        vec![]
    }
}

macro_rules! impl_fromstr {
    ($t:ty) => {
        impl ClapMe for $t {
            fn with_clap<T>(info: ArgInfo, app: clap::App,
                            f: impl FnOnce(clap::App) -> T) -> T {
                f(app.arg(clap::Arg::with_name(info.name)
                          .long(info.name)
                          .takes_value(true)
                          .requires_all(info.required_flags)
                          .required(info.required)
                          .help(&info.help)
                          .validator(|s| Self::from_str(&s).map(|_| ())
                                     .map_err(|e| e.to_string()))))
            }
            fn from_clap(name: &str, matches: &clap::ArgMatches) -> Option<Self> {
                matches.value_of(name).map(|s| Self::from_str(s).unwrap())
            }
        }

        impl ClapMe for Vec<$t> {
            fn with_clap<TT>(info: ArgInfo, app: clap::App,
                             f: impl FnOnce(clap::App) -> TT) -> TT {
                f(app.arg(clap::Arg::with_name(info.name)
                          .long(info.name)
                          .takes_value(true)
                          .required(false)
                          .requires_all(info.required_flags)
                          .multiple(true)
                          .help(&info.help)
                          .validator(|s| <$t>::from_str(&s).map(|_| ())
                                     .map_err(|_| "oops".to_owned()))))
            }
            fn from_clap(name: &str, matches: &clap::ArgMatches) -> Option<Self> {
                Some(matches.values_of(name).unwrap_or(clap::Values::default())
                     .map(|s| <$t>::from_str(s).unwrap()).collect())
            }
            fn requires_flags(_name: &str) -> Vec<String> {
                vec![]
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
impl_fromstr!(usize);
impl_fromstr!(f32);
impl_fromstr!(f64);


macro_rules! impl_from {
    ($t:ty) => {
        impl ClapMe for $t {
            fn with_clap<T>(info: ArgInfo, app: clap::App,
                            f: impl FnOnce(clap::App) -> T) -> T {
                f(app.arg(clap::Arg::with_name(info.name)
                          .long(info.name)
                          .takes_value(true)
                          .requires_all(info.required_flags)
                          .required(info.required)
                          .help(&info.help)))
            }
            fn from_clap(name: &str, matches: &clap::ArgMatches) -> Option<Self> {
                matches.value_of(name).map(|s| Self::from(s))
            }
        }

        impl ClapMe for Vec<$t> {
            fn with_clap<TT>(info: ArgInfo, app: clap::App,
                             f: impl FnOnce(clap::App) -> TT) -> TT {
                f(app.arg(clap::Arg::with_name(info.name)
                          .long(info.name)
                          .takes_value(true)
                          .required(false)
                          .requires_all(info.required_flags)
                          .multiple(true)
                          .help(&info.help)))
            }
            fn from_clap(name: &str, matches: &clap::ArgMatches) -> Option<Self> {
                Some(matches.values_of(name).unwrap_or(clap::Values::default())
                     .map(|s| <$t>::from(s)).collect())
            }
            fn requires_flags(_name: &str) -> Vec<String> {
                vec![]
            }
        }
    }
}

impl_from!(String);
impl_from!(std::path::PathBuf);

impl<T: ClapMe> ClapMe for Option<T> {
    fn with_clap<TT>(mut info: ArgInfo, app: clap::App,
                     f: impl FnOnce(clap::App) -> TT) -> TT {
        info.required = false;
        T::with_clap(info, app, f)
    }
    fn from_clap(name: &str, matches: &clap::ArgMatches) -> Option<Self> {
        Some(T::from_clap(name, matches))
    }
    fn requires_flags(_name: &str) -> Vec<String> {
        vec![]
    }
}
