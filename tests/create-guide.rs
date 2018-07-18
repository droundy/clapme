#[macro_use]
extern crate clapme;

use clapme::ClapMe;
use std::io::{Write,BufRead};

/// # A user's guide for clapme.
///
/// ClapMe allows you to parse command line arguments by defining a
/// struct.  It combines [clap](https://crates.io/crates/clap) with
/// custom derive.
///
/// The basic idea is that you define a type that represents the
/// information you want on the command-line from the person running
/// your program, and `derive(ClapMe)` on that type, and then call
/// `YourType::from_args()` to find out what your user gave you.

/// To begin with, let's look at an example of how you might actually
/// use `ClapMe` in a real program.
///
/// ```should_panic
/// #[macro_use]
/// extern crate clapme;
///
/// use std::path::PathBuf;
/// use clapme::ClapMe;
///
/// #[derive(Debug, ClapMe)]
/// struct Opt {
///     /// Filling fraction
///     filling_fraction: f64,
///     /// Number of atoms
///     N: u32,
///     /// Output directory, working directory if not present
///     dir: Option<PathBuf>,
///     /// Activate verbose printing
///     verbose: bool,
/// }
///
/// fn main() {
///     let opt = Opt::from_args();
///     println!("{:?}", opt);
/// }
/// ```

/// The remainder of this guide will give examples of how the
/// command-line flags are constructed from your type.

#[test]
fn guide() {
    let mut strings = Vec::new();
    #[derive(ClapMe)]
    // START CODE
    struct Foo {
        foo: bool,
    }
    // STOP CODE
    /// This gives the following usage.
    strings.push(Foo::help_message("foo"));
    // INSERT STRING

    strings.reverse();

    println!("current dir is {:?}", std::env::current_dir());
    println!("hello world");
    let src = std::path::Path::new("tests/create-guide.rs");
    println!("hello world");
    let dest = std::path::Path::new("src/guide.rs");
    println!("creating {:?}", &dest);
    let mut f = std::fs::File::create(&dest).unwrap();
    println!("opening {:?}", &src);
    let i = std::fs::File::open(&src).unwrap();
    let lines = std::io::BufReader::new(&i);
    let mut am_writing = false;
    let mut chars_to_trim = 0;
    for line in lines.lines() {
        let l: String = line.unwrap();
        if l.contains(&format!("{}{}", "//","/")) {
            let l = l.replacen(&format!("{}{}", "//","/"), "", 1);
            writeln!(f, "//! {}", &l.trim()).unwrap();
        } else if l.contains(&format!("{} {}", "START", "CODE")) {
            am_writing = true;
            chars_to_trim = l.find(|c: char| !c.is_whitespace()).unwrap();
            writeln!(f, "//! ```").unwrap();
        } else if l.contains(&format!("{} {}", "STOP", "CODE")) {
            am_writing = false;
            writeln!(f, "//! ```").unwrap();
        } else if l.contains(&format!("{}.{}", "strings", "push")) {
            let val = strings.pop().unwrap();
            writeln!(f, "//! ```ignore").unwrap();
            for ll in val.lines() {
                writeln!(f, "//! {}", &ll).unwrap();
            }
            writeln!(f, "//! ```").unwrap();
        } else if am_writing {
            writeln!(f, "//! {}", &l.split_at(chars_to_trim).1).unwrap();
        }
    }
}
