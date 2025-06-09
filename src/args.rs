use color_eyre::eyre::{ErrReport, Error};
use color_eyre::Result;
use std::str::FromStr;

// Deliberately didn't use clap as to avoid unnecassary heavy dependencies

pub struct Args {
    /// The number of weeks to look back.
    /// Default: 0 (last week)
    pub n: i64,
    /// True if the `-h` or `--help` argument was provided
    pub help: bool,
}

impl Args {
    /// Try to parse the command line arguments
    ///
    /// # Errors
    ///
    /// If the command line arguments could not be parsed
    pub fn try_parse() -> Result<Self> {
        let args = Self::args();

        Ok(Self {
            n: Self::try_find_parse_value(&args, "-n")?.unwrap_or(0),
            help: Self::has_arg_value(&args, "-h") || Self::has_arg_value(&args, "--help"),
        })
    }

    /// Print the help menu
    pub fn print_help() {
        println!(
            "{} v{} by {}",
            env!("CARGO_PKG_NAME"),
            env!("CARGO_PKG_VERSION"),
            env!("CARGO_PKG_AUTHORS")
        );
        println!();
        println!("Arguments:");

        Self::print_arg_help("n", "The number of weeks ago. A value of zero means the previous week. A value of 1 means one week before last week, etc. Default: 0");
        Self::print_arg_help("-h, --help", "Shows this menu");
    }

    /// Format and print the help line for an argument
    fn print_arg_help(name: &str, desc: &str) {
        println!("- `{name}`: {desc}");
    }

    /// Try to find an argument in the list of arguments, and parse it into the required type.
    ///
    /// # Errors
    ///
    /// If the value could not be parsed into the required type.
    ///
    /// # None
    ///
    /// If the argument does not exist in the list of arguments
    fn try_find_parse_value<T>(args: &[String], name: &str) -> Result<Option<T>>
    where
        T: FromStr,
        <T as FromStr>::Err: Into<ErrReport>,
    {
        Self::find_arg_value(args, name)?
            .map(|value| T::from_str(value).map_err(|e| e.into()))
            .transpose()
    }

    /// Check if the list of arguments contains the wanted argument
    fn has_arg_value(args: &[String], name: &str) -> bool {
        args.iter().any(|s| s.eq(name))
    }

    /// Find an argument value.
    /// Returns the next element after the argument name
    fn find_arg_value<'a>(args: &'a [String], name: &str) -> Result<Option<&'a str>> {
        // Check if the argument even exists
        let name_exists = Self::has_arg_value(args, name);
        if !name_exists {
            return Ok(None);
        }

        // Find the value of the argument
        let mut iter = args.iter();
        let value = iter
            .find(|s| s.as_str().eq(name))
            .and_then(|_| iter.next())
            .map(|s| s.as_str());

        // Return an error if the name exists but no value is given
        match value {
            Some(value) => Ok(Some(value)),
            None if name_exists => Err(Error::msg(format!("Missing value for argument `{name}`"))),
            None => Ok(None),
        }
    }

    /// Get the command line arguments
    fn args() -> Vec<String> {
        std::env::args().collect::<Vec<_>>()
    }
}
