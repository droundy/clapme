#[macro_use]
extern crate clapme;

use clapme::ClapMe;
use std::io::{Write,BufRead};

/// A user's guide for clapme.
///
#[test]
fn guide() {
    let mut strings = Vec::new();
    /// A user's guide for clapme.
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
            let l = l.replace(&format!("{}{}", "//","/"), "");
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
