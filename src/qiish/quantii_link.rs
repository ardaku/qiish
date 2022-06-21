// Copyright (c) 2022 The Quantii Contributors
//
// This file is part of Quantii.
//
// Quantii is free software: you can redistribute
// it and/or modify it under the terms of the GNU
// Lesser General Public License as published by
// the Free Software Foundation, either version 3
// of the License, or (at your option) any later
// version.
//
// Quantii is distributed in the hope that it
// will be useful, but WITHOUT ANY WARRANTY;
// without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR
// PURPOSE. See the GNU Lesser General Public
// License for more details.
//
// You should have received a copy of the GNU
// Lesser General Public License along with
// Quantii. If not, see <https://www.gnu.org/licenses/>.

// section clippy
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
#![allow(clippy::unwrap_used)]
#![allow(clippy::let_underscore_drop)]
#![allow(clippy::indexing_slicing)]
#![allow(clippy::inline_always)]
#![allow(clippy::unwrap_in_result)]

use qiish_argparse::ArgParser;
use std::env;

fn main() {
    let (args, flags): (Vec<String>, Vec<String>) = {
        let mut argparser: ArgParser =
            ArgParser::new(env::args().collect::<String>());
        argparser.parse();
        (argparser.args, argparser.flags)
    };

    if args.contains(&"--help".to_owned()) || args.contains(&"-h".to_owned()) {
        println!("{}", help_message());
        return;
    } else if args.contains(&"--version".to_owned()) || args.contains(&"-V".to_owned()) {
        println!("{}", version_message());
        return;
    } else {
        if flags.len() != 0 {
            println!("{}", help_message());
            return;
        } else if args.len() != 2 {
            println!("{}", help_message());
            return;
        } else {
        }
    }
}

fn help_message() -> &str {
    "\
    Usage: quantii-link [options] <url> <name>\n\
    Options:\n\
    -h, --help                 Show this help message and exit\n\
    -V, --version              Show version information and exit\n"
}

fn version_message() -> &str {
    "\
    qiish v0.1.0\n"
}
