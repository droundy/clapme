//! # A user's guide for clapme.
//! 
//! ClapMe allows you to parse command line arguments by defining a
//! struct.  It combines [clap](https://crates.io/crates/clap) with
//! custom derive.
//! 
//! The basic idea is that you define a type that represents the
//! information you want on the command-line from the person running
//! your program, and `derive(ClapMe)` on that type, and then call
//! `YourType::from_args()` to find out what your user gave you.
//! To begin with, let's look at an example of how you might actually
//! use `ClapMe` in a real program.
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
//! /// Filling fraction
//! filling_fraction: f64,
//! /// Number of atoms
//! N: u32,
//! /// Output directory, working directory if not present
//! dir: Option<PathBuf>,
//! /// Activate verbose printing
//! verbose: bool,
//! }
//! 
//! fn main() {
//! let opt = Opt::from_args();
//! println!("{:?}", opt);
//! }
//! ```
//! The remainder of this guide will give examples of how the
//! command-line flags are constructed from your type.
//! ```
//! struct Foo {
//!     foo: bool,
//! }
//! ```
//! This gives the following usage.
//! ```ignore
//! foo 
//! 
//! USAGE:
//!     foo [FLAGS]
//! 
//! FLAGS:
//!         --foo    
//! ```
