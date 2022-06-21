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
#![allow(clippy::struct_excessive_bools)]
#![allow(clippy::integer_arithmetic)]
#![allow(clippy::too_many_lines)]

// section uses
use super::super::rewrite_relative_dir;
use qiish_argparse::ArgParser;
use std::path::{Path, PathBuf};

/// Data for the CP command.
#[derive(Debug)]
pub struct DataCP {
    /// Source path(s).
    /// Args `0..=args.len() - 1`
    pub source: Vec<PathBuf>,
    /// Destination path.
    /// Arg `args.len()`
    pub destination: PathBuf,
    /// Prompt for each file/directory.
    /// `-i`, `--interactive`
    pub interactive: bool,
    /// Disallow overwriting files.
    /// `-n`, `--no-clobber`
    pub no_clobber: bool,
    /// Copy directories recursively.
    /// `-r`, `--recursive`
    pub recursive: bool,
}

impl From<(Vec<String>, Vec<String>)> for DataCP {
    fn from(arguments: (Vec<String>, Vec<String>)) -> Self {
        let (args, flags): (Vec<String>, Vec<String>) = arguments;
        let interactive: bool =
            flags.contains(&"i".to_owned()) || flags.contains(&"interactive".to_owned());
        let no_clobber: bool =
            flags.contains(&"n".to_owned()) || flags.contains(&"no-clobber".to_owned());
        let recursive: bool =
            flags.contains(&"r".to_owned()) || flags.contains(&"recursive".to_owned());

        Self {
            source: args[0..(args.len() - 1)].iter().map(Into::into).collect(),
            destination: args[args.len() - 1].clone().into(),
            interactive,
            no_clobber,
            recursive,
        }
    }
}

/// The CP command.
pub fn cp(command: (String, Vec<&str>), homedir: &Path, cwd: &Path) -> (i16, bool) {
    let (_, args) = command;

    let data: DataCP = DataCP::from({
        let mut parser: ArgParser = ArgParser::new(args.join(" "));
        parser.parse();
        (parser.args, parser.flags)
    });

    let mut result: i16 = 0;

    if data.source.is_empty() {
        println!("cp: missing input operand");
        result = -1;
    } else if data.destination.to_str().unwrap() == "" {
        println!("cp: Missing destination operand");
        result = -1;
    } else {
        for source_file in data.source.iter().map(|path: &PathBuf| {
            rewrite_relative_dir(path.clone(), homedir, cwd).unwrap_or_else(|_| path.clone())
        }) {
            let destination_file: PathBuf =
                rewrite_relative_dir(data.destination.clone(), homedir, cwd)
                    .unwrap_or_else(|_| data.destination.clone());
            if destination_file.is_dir() {
                if !data.recursive {
                    println!("cp: {}: Is a directory", destination_file.to_str().unwrap());
                    println!("cp: Hint: Try 'cp -r'");
                    result = -1;
                }

                if destination_file.exists() {
                    if data.no_clobber {
                        println!("cp: {}: File exists", destination_file.to_str().unwrap());
                        result = -1;
                    } else {
                        if data.interactive {
                            println!(
                                "cp: overwrite {}? (y/n)",
                                destination_file.to_str().unwrap()
                            );
                            let mut input = String::new();
                            std::io::stdin().read_line(&mut input).unwrap();
                            if input.trim() != "y" {
                                continue;
                            }
                        }

                        match std::fs::remove_dir_all(&destination_file) {
                            Ok(_) => continue,
                            Err(e) => {
                                println!(
                                    "cp: Could not remove '{}': {}",
                                    destination_file.display(),
                                    e.kind()
                                );
                            }
                        };
                        std::fs::create_dir_all(destination_file).unwrap();
                    }
                }
            } else if destination_file.exists() {
                if data.no_clobber {
                    println!("cp: cannot overwrite '{}'", destination_file.display());
                    result = -1;
                    continue;
                } else if data.interactive {
                    println!("cp: overwrite '{}'?", destination_file.display());
                    let mut input = String::new();
                    std::io::stdin().read_line(&mut input).unwrap();
                    if input.trim().to_lowercase() == "n" {
                        continue;
                    }
                } else {
                    // Proceed with overwriting.
                }
            } else {
                // Destination file does not exist.
                if source_file.is_dir() {
                    std::fs::create_dir_all(&destination_file).unwrap();
                } else {
                    std::fs::copy(&source_file, &destination_file).unwrap();
                }
            }
        }
    }

    (result, false)
}
