#![warn(clippy::all)]
#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]
#![warn(clippy::cargo)]
#![warn(clippy::suspicious)]

use log::info;
use std::process::exit;
use crate::options::Options;

/// Lex's the input string into a vector of tokens.
pub(crate) mod lex;
/// Implements a Peekable-like trait so you can peek multiple items ahead.
pub(crate) mod lookahead;
/// Parses the vector of tokens into a vector of parsed tokens.
pub(crate) mod parse;
/// Runs the shell.
pub(crate) mod run;
/// Options for the shell.
pub(crate) mod options;

fn main() {
    env_logger::init();

    let args = std::env::args();
    let args = args.collect::<Vec<String>>();
    let args = args.iter().skip(1).collect::<Vec<&String>>().clone();

    // Check if args has exactly 1 or 2 elements
    match args.len() {
        1 => {
            exit(run(args[0], Options::default()));
        }

        2 => {
            let options = match_options(args[0]);
            let options: Options = parse_options(options);
            exit(run(args[1], options));
        }

        _ => {
            exit(1);
        }
    }
}

fn parse_options(options: Vec<char>) -> Options {
    let mut ret = Options {
        help: false,
        version: false,
        verbose: false,
    };
    if options.contains(&'v') {
        ret.verbose = true;
    }
    if options.contains(&'V') {
        ret.version = true;
    }
    if options.contains(&'h') {
        ret.help = true;
    }
    ret
}

fn run(infile: &str, options: Options) -> i32 {
    let tokens = match lex::lex(infile, options) {
        (0, tok) => tok,
        (exit_, _) => return exit_,
    };

    info!("{:?}", tokens);

    let parsed_tokens = match parse::parse(tokens, options) {
        (0, tok) => tok,
        (exit_, _) => return exit_,
    };
    0
}

fn match_options(options: &str) -> Vec<char> {
    let mut options = options.chars();
    options.next();

    let mut ret = vec![];
    for option in options {
        ret.push(option);
    }
    ret
}

