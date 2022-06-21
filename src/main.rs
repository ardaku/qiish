//! Not really a crate :P
#![warn(
    clippy::all,
    clippy::restriction,
    clippy::pedantic,
    clippy::nursery,
    clippy::cargo
)]
#![allow(clippy::implicit_return)]
#![allow(clippy::missing_inline_in_public_items)]
#![allow(clippy::print_stdout)]
#![allow(clippy::blanket_clippy_restriction_lints)]
#![allow(clippy::redundant_else)]

use ::qiish::Qiish;
use qiish_argparse::ArgParser;
use std::env;
use std::path::Path;

/// Actual qiish bit
#[path = "qiish/qiish.rs"]
mod qiish;

fn main() {
    println!(
        "{}",
        qiish::rewrite_relative_dir(
            Path::new("~/qiishhome").to_owned(),
            Path::new("/Users/will"),
            Path::new("/Users/will/qiishhome")
        )
        .unwrap()
        .to_str()
        .unwrap()
    );

    if let Ok(qiish) = Qiish::new(Path::new("/Users/will/qiishhome")) {
        qiish
    } else {
        println!("Mate that aint gonna work sorry bro");
        return;
    }
    .call_qiish(0);
}

fn main_quantii() {
    let (args, flags): (Vec<String>, Vec<String>) = {
        let mut argparser: ArgParser = ArgParser::new(env::args().collect::<String>());
        argparser.parse();
        (argparser.args, argparser.flags)
    };

    if args.contains(&"--help".to_owned()) || args.contains(&"-h".to_owned()) {
        println!("{}", help_message());
        return;
    } else if args.contains(&"--version".to_owned()) || args.contains(&"-V".to_owned()) {
        println!("{}", version_message());
        return;
    } else if flags.len() != 0 {
        println!("{}", help_message());
        return;
    } else if args.len() != 2 {
        println!("{}", help_message());
        return;
    } else {
        let homedir = args[0].clone();
        let mut qiish = Qiish::new(Path::new(&homedir)).unwrap();
        qiish.call_qiish(args[1].parse::<u8>().unwrap());
    }
}

fn help_message() -> &'static str {
    "\
    Usage: quantii-link [-hV] <homedir> <entrance>\n\
    Options:\n\
    -h, --help                 Show this help message and exit\n\
    -V, --version              Show version information and exit\n"
}

fn version_message() -> &'static str {
    "\
    qiish v0.1.0\n"
}
