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
/// command-line flags are constructed from your type, starting with
/// simple cases and moving gradually to more complex ones.  In no
/// case does your code involve more than defining a type, deriving
/// `ClapMe` and calling the `from_args` method of your type.

/// I want to note that `ClapMe` *almost always* produces long flags.
/// This is because I feel that long flags are generally the easiest
/// to use.  If you want to fine-tune your command-line interface,
/// `ClapMe` may not be for you.

#[test]
fn guide() {
    let mut strings = Vec::new();
    /// ## Just a flag

    /// Most often, you will define a struct as your type.  We'll
    /// start out with the simplest case, which is a single boolean
    /// field within that struct.
    #[derive(ClapMe)]
    // START CODE
    struct Foo {
        foo: bool,
    }
    // STOP CODE
    /// This gives the following usage.
    strings.push(Foo::help_message("foo"));
    // INSERT STRING
    /// A single boolean flag is treated as an optional flag.

    /// ## How the flag is determined

    /// We saw above that the flag just had `--` prepended to the
    /// field name.  The rule in general is only slightly more
    /// complicated: every underscore is replaced with a `-`.
    #[derive(ClapMe)]
    #[allow(non_snake_case)]
    // START CODE
    struct Flags {
        verbose: bool,
        blue_is_nice: bool,
        min_T: bool,
    }
    // STOP CODE
    /// This gives the following usage.
    strings.push(Flags::help_message("flags"));
    // INSERT STRING
    /// Thus you can create most any flag name you care for, and it is
    /// easy to tell which flag corresponds to which field in your
    /// struct.

    /// ## Adding help information

    /// We add help information simply by adding ordinary doc comments
    /// to our struct.
    #[derive(ClapMe)]
    #[allow(non_snake_case)]
    // START CODE
    struct Help {
        /// Print excess messages.
        verbose: bool,
        /// The lowest temperature.
        min_T: bool,
    }
    // STOP CODE
    /// This gives the following usage.
    strings.push(Help::help_message("help"));
    // INSERT STRING
    /// In most of this documentation I'll avoid adding help text,
    /// just to keep the page short, but I would always add it for
    /// actual projects!

    /// ## Other types

    /// You can add most standard library types to your struct,
    /// basically anything that can be read or parsed from a `&str`.
    /// I'd recommend sticking to owned types.
    #[derive(ClapMe)]
    #[allow(non_snake_case)]
    // START CODE
    struct Types {
        name: String,
        T: f64,
        directory: std::path::PathBuf,
    }
    // STOP CODE
    /// This gives the following usage.
    strings.push(Types::help_message("types"));
    // INSERT STRING

    /// I will note here that when parsing user input, a specified
    /// temperature of `1/3` will correctly give a value of one third,
    /// and `sqrt(2)` will give root 2.  Allowing simple arithmetic as
    /// floating point input makes it easier to give high-precision
    /// input when needed.

    /// ## Optional flags

    /// In the previous examples, every flag (except a `bool` flag)
    /// was required to be specified by the user.  If you want a flag
    /// to be optional, you just use the standard `Option` type.
    #[derive(ClapMe)]
    // START CODE
    struct Optional {
        name: Option<String>,
    }
    // STOP CODE
    /// This gives the following usage.
    strings.push(Optional::help_message("optional"));
    // INSERT STRING
    /// The value is then `None` if the user did not specify that flag.

    /// ## Exclusive flags

    /// If you want to make certain flags/values mutually exclusive,
    /// you use an `enum` (just as always, in rust).
    #[derive(ClapMe)]
    // START CODE
    enum Exclusive {
        First {
            a: String,
            b: String,
        },
        SecondFlag(String),
        Third_,
    }
    // STOP CODE
    /// This gives the following usage.
    strings.push(Exclusive::help_message("exclusive"));
    // INSERT STRING
    /// This example illustrates the three kinds of `enum` variants.
    /// Sadly, the help message does not indicate that these flags are
    /// exlusive.  However, if a user tries to specify both `--third`
    /// and `--second FOO`, however, they will get a nice error
    /// message.  Note that you cannot use a tuple variant with more
    /// than one field.

    /// Note that the rules for constructing flags from enum variants
    /// are more complicated than for struct fields.  This is because
    /// by convention variants are given `CamelCase` names, which
    /// aren't suitable as flags.  If a variant name contains an
    /// underscore, then it is treated like a field name (as described
    /// above), with any trailing underscores removed.  Otherwise the
    /// name is converted from `CamelCase` to `kebab-case`.

    /// ## Nesting types

    /// You can use any `ClapMe` type as a field within a struct or
    /// enum.  Doing so will give flag names that combine the nested
    /// field names.
    // IGNORE CODE
    #[derive(ClapMe)]
    struct Vec2d {
        x: f64, y: f64,
    }
    #[derive(ClapMe)]
    struct Nested {
        position: Vec2d,
        velocity: Vec2d,
    }
    // STOP CODE
    /// This gives the following usage.
    strings.push(Nested::help_message("nested"));
    // INSERT STRING


    /// ## Flattened nesting types

    /// As you say in the last example, nesting types allows you to
    /// make your own complex types that can be reused.  Sometimes,
    /// however, you would like to nest structs for a different
    /// reason: to separate concerns in the code.  In this case, you
    /// may not want the nesting to be visible in the user interface.
    /// This can be acheived with a leading underscore on a field
    /// name.  The catch is that when you do this, you could run into
    /// a runtime error if you have duplicate field names.
    // IGNORE CODE
    #[derive(ClapMe)]
    struct MyConfig {
        name: String,
    }
    #[derive(ClapMe)]
    struct YourConfig {
        address: String,
    }
    #[derive(ClapMe)]
    struct Flattened {
        _mine: MyConfig,
        _yours: YourConfig,
    }
    // STOP CODE
    /// This gives the following usage.
    strings.push(Flattened::help_message("flattened"));
    // INSERT STRING
    /// This may be a good idea if `MyConfig` and `YourConfig` are
    /// implementation details that your user need not be aware of.

    /// ## Other possibilities

    /// There may be a few other features that clapme has, for which I
    /// have not bothered to create an entire example.  I will list
    /// them here when they come to mind.

    /// 1. You can use a `Vec<T>` for many values of `T` to create an
    ///    option that can be specified more than once.

    /// ## Conclusion

    /// There is more that could be said and more possible examples,
    /// but I think this is enough to get you started using `ClapMe`.
    /// The intent is that any reasonable type that *can* be obtained
    /// from one or more strings should work with clapme.  Please fill
    /// an issue on github if there is a type that you would like to
    /// have supported by clapme.  Pull requests are most welcome.

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
        if l.contains(&format!("{}{}", "//","/")) && !am_writing {
            let l = l.replacen(&format!("{}{}", "//","/"), "", 1);
            writeln!(f, "//! {}", &l.trim()).unwrap();
        } else if l.contains(&format!("{} {}", "START", "CODE")) {
            am_writing = true;
            chars_to_trim = l.find(|c: char| !c.is_whitespace()).unwrap();
            writeln!(f, "//! ```").unwrap();
        } else if l.contains(&format!("{} {}", "IGNORE", "CODE")) {
            am_writing = true;
            chars_to_trim = l.find(|c: char| !c.is_whitespace()).unwrap();
            writeln!(f, "//! ```ignore").unwrap();
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

