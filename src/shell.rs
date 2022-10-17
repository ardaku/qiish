use std::{
    iter::Peekable,
    vec::IntoIter
};

use crate::Options;


pub struct Shell {
    pub args: Vec<String>,
    pub options: Options,
}

/// The main function for the shell.
fn main() -> Result<(), i32> {
    let args = std::env::args().skip(1).collect::<Vec<&String>>().clone();
    let options = Shell::parse_shell_options(args)?;
    let mut shell = Shell::new();
    shell.run()?;
    Ok(())
}

impl Shell {
    pub fn new(args: Vec<String>, options: Options) -> Self {
        Self {
            args,
            options,
        }
    }

    pub fn run(&self) -> Result<(), i32> {
        Ok(())
    }

    fn parse_shell_options(options: Vec<&String>) -> Result<Options, i32> {
        let mut ret = Options {
            help: false,
            version: false,
            verbose: false,
        };
        let iter: Peekable<IntoIter<&String>> = options.into_iter().peekable();
        for option in iter {
            let mut option_or_arg: bool = false;
            let mut single_option: bool = false;
            let mut option_chars = option.chars();
            if option_chars.next() == Some('-') {
                option_or_arg = true;
            }
            if option_chars.next() == Some('-') {
                single_option = true;
            }

            if option_or_arg {
                if single_option {
                    let mut option_string: String = String::new();
                    while let Some(option_char) = option_chars.next() {
                        option_string.push(option_char);
                    }
                } else {
                }
            }
        }
        Ok(ret)
    }
}