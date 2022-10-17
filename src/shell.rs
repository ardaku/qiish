#![allow(dead_code)]


use std::env;
use std::vec::IntoIter;

#[derive(Copy, Clone, Debug, Default, Eq, PartialEq)]
pub struct Options {
    pub help: bool,
    pub version: bool,
    pub verbose: bool,
}

pub struct Shell {
    pub args: Vec<String>,
    pub options: Options,
}



impl Shell {
    #[must_use] pub const fn new(args: Vec<String>, options: Options) -> Self {
        Self {
            args,
            options,
        }
    }


    pub const fn run(&self) -> Result<(), i32> {
        Ok(())
    }

    fn parse_shell_options(options: Vec<String>) -> Result<(Vec<String>, Options), i32> {
        let mut ret_options = Options {
            help: false,
            version: false,
            verbose: false,
        };
        let iter: IntoIter<String> = options.into_iter();

        let raw_double_dash_options = iter.clone().filter(|x| x.starts_with("--")).collect::<Vec<String>>();
        let raw_single_dash_options = iter.clone().filter(|x| x.starts_with('-')).collect::<Vec<String>>();
        let raw_double_dash_options = raw_double_dash_options.iter().map(|x| x.trim_start_matches("--")).collect::<Vec<&str>>();
        let raw_single_dash_options = raw_single_dash_options.iter().map(|x| x.trim_start_matches('-')).collect::<Vec<&str>>();

        let ret_args = iter.filter(|x| !x.starts_with('-')).collect::<Vec<String>>();

        for op in raw_single_dash_options {
            let op_chars = op.chars().collect::<Vec<char>>();
            for c in op_chars {
                match c {
                    'h' => ret_options.help = true,
                    'V' => ret_options.version = true,
                    'v' => ret_options.verbose = true,
                    'q' => ret_options.verbose = false,
                    '-' => break,
                    _ => return Err(2),
                }
            }
        }

        for op in raw_double_dash_options {
            match op {
                "help" => ret_options.help = true,
                "version" => ret_options.version = true,
                "verbose" => ret_options.verbose = true,
                "quiet" => ret_options.verbose = false,
                "" => break,
                _ => return Err(3),
            }
        }

        if ret_args.is_empty() {
            ret_options.help = true;
        }

        Ok((ret_args, ret_options))
    }

    /// Print the help message, in beautiful colors.
    pub fn print_help() {
        let mut help = String::new();
        help.push_str("\x1b[1m\x1b[32m");
        help.push_str("Usage: ");
        help.push_str("\x1b[0m");
        help.push_str("\x1b[1m");
        help.push_str("qiish [-hqVv] [file]");
        help.push_str("\x1b[0m");
        help.push_str("\x1b[1m\x1b[32m\n\n");
        help.push_str("Options:\n");
        help.push_str("\x1b[0m");
        help.push_str("\x1b[1m  -h, --help\t\t\x1b[0m Print this help message and exit.\n");
        help.push_str("\x1b[1m  -q, --quiet\t\t\x1b[0m Do not print anything to stdout.\n");
        help.push_str("\x1b[1m  -V, --version\t\t\x1b[0m Print the version and exit.\n");
        help.push_str("\x1b[1m  -v, --verbose\t\t\x1b[0m Print debug information to stdout.\n");
        println!("{}", help);

    }
}

/// The main function for the shell.
fn main() -> Result<(), i32> {
    let args = env::args().skip(1).collect::<Vec<String>>();
    let (real_args, options) = Shell::parse_shell_options(args)?;
    println!("{:?}; {:?}", options, real_args);
    let shell = Shell::new(real_args, options);
    if shell.options.help {
        Shell::print_help();
        return Ok(());
    }
    shell.run()?;
    Ok(())
}