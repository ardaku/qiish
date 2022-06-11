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

// section crates
extern crate alloc;

// section uses
// extern crate quantii;
extern crate std;

// section uses

mod other_commands;
use other_commands::rm::rm;

use crate::qiish::other_commands::cat::cat;
use alloc::{string::String, string::ToString};
use qiish_argparse::ArgParser;

use std::fs::{read_dir, DirEntry};

use std::{
    collections::HashMap,
    fs,
    fs::canonicalize,
    io::{stdin, stdout, Error, ErrorKind, Result, Write},
    iter::Map,
    path::Component,
    path::{Path, PathBuf},
    print, println,
    str::Lines,
    vec::Vec,
};

// section struct Qiish

/// ## `QuantII SHell`
/// The perfect shell for all your shelly  needs
pub struct Qiish {
    /// Path to the .qiishenv file. Set to `homedir` + `"/.qiishenv"`
    qiishenv: PathBuf,
    /// Current working directory of shell
    pub cwd: PathBuf,
    /// home directory
    homedir: PathBuf,
}

// section impl Qiish

impl Qiish {
    ///
    ///
    /// # Arguments
    ///
    /// * `homedir`: home directory to set '~' to
    ///
    /// returns: Qiish
    pub fn new(home_dir: &Path) -> Result<Self> {
        let qiishenv_loc = if let Ok(path) = rewrite_relative_dir(
            home_dir.join(".qiishenv"),
            home_dir,
            &*rewrite_relative_dir(home_dir.to_owned(), home_dir, home_dir).unwrap(),
        ) {
            path
        } else {
            println!(
                "Failed to instantiate Qiiish: no such file or directory: {}",
                home_dir.to_owned().join(".qiishenv").to_str().unwrap()
            );
            return Err(Error::new(
                ErrorKind::Other,
                format!("No such directory {}", home_dir.to_str().unwrap()),
            ));
        };

        Ok(Self {
            qiishenv: qiishenv_loc,
            cwd: home_dir.to_owned(),
            homedir: home_dir.to_owned(),
        })
    }

    /// `QuantII SHell`
    ///
    /// sh variant. Kinda like bash or zsh
    pub fn call_qiish(&mut self, entrance_code: u8) {
        let mut exit: bool = false;

        let env = get_env(self);

        let computer_name: &str = match env.get("computername") {
            Some(name) => name,
            None => "comp_name_unk",
        };

        let username: &str = match env.get("computername") {
            Some(name) => name,
            None => "user_name_unk",
        };

        flush();

        while !exit {
            print!(
                "{}#{}@{} {} % ",
                computer_name,
                username,
                entrance_code,
                self.simplify_display(self.cwd.clone())
            );
            flush();

            let mut line: String = String::new();
            let _ = stdin().read_line(&mut line);

            let command: (&str, &str) = match line.split_once(' ') {
                Some((before, after)) => (before, after),
                None => (line.as_str(), ""),
            };

            let full_command: (String, Vec<&str>) = (
                command.0.trim().to_owned(),
                command.1.split_whitespace().collect(),
            );
            let exit_code: (
                i16,  // Exit code itself
                bool, // Whether or not the shell should exit
            ) = self.call_command(&full_command, &env, self.cwd.clone().as_path());

            if exit_code.0 > 0 {
                println!("\nProgram exited with error code {}", exit_code.0);
            } else if exit_code.0 < -1 {
                println!("\nProgram exited with irregular error code {}", exit_code.0);
            } else {
                // Returned 0 or -1 (standard success and simple error codes)
            }
            exit = exit_code.1;
            println!();
        }
    }

    /// |
    ///
    /// # Arguments
    ///
    /// * `command`: full command `(String, Vec<&str>)`
    /// * `environment`: environment variables
    ///
    /// returns: `(i16, bool)` (exit code, should exit)
    fn call_command(
        &mut self,
        command: &(String, Vec<&str>),
        environment: &HashMap<String, String>,
        cwd: &Path,
    ) -> (i16, bool) {
        match command.0.as_str() {
            "" => (0, false),
            "help" => Self::help(),
            "exit" => (0, true),
            "cd" => self.cd(command, environment),
            "ls" => self.ls(command, environment, &self.homedir.clone()),
            "clear" => Self::clear(),
            "mkdir" => self.mkdir(command, environment),
            "rm" => rm(command.clone(), &self.homedir.clone(), cwd),
            "rmdir" => rm(
                (command.0.clone(), {
                    let mut args = command.1.clone();
                    args.insert(0, "-r");
                    args
                }),
                &self.homedir,
                cwd,
            ),
            "cat" => cat(command.clone(), &self.homedir, cwd),
            "echo" => self.echo(command.clone()),
            "pwd" => self.pwd(command.clone(), environment),
            // "touch" => touch(command.clone(), environment),
            // "cp" => self.cp(command.clone(), environment),
            // "mv" => self.mv(command.clone(), environment),
            _ => {
                println!("qiish: Unrecognized command: {}", command.0);
                println!("qiish: Run 'help' for a list of commands");
                (0, false)
            }
        }
    }

    // section builtins

    /// Prints directory contents
    ///
    fn ls(
        &mut self,
        command: &(String, Vec<&str>),
        environment: &HashMap<String, String>,
        homedir: &Path,
    ) -> (i16, bool) {
        let mut parser = ArgParser::new(command.1.join(" "));
        parser.parse();
        let files = parser.args;
        let options = parser.flags;

        let mut out: String = String::new();

        let all: bool = options.contains(&"a".to_owned()) || options.contains(&"all".to_owned());
        let color: bool = options.contains(&"color".to_owned());
        let classify: bool =
            options.contains(&"F".to_owned()) || options.contains(&"classify".to_owned());
        let dirs_first: bool = options.contains(&"group-directories-first".to_owned());
        let comma_separated: bool = options.contains(&"m".to_owned());
        let dir_indicator: bool =
            options.contains(&"p".to_owned()) || options.contains(&"indicator-style".to_owned());

        if files.is_empty() {
            return self.ls(
                &(command.0.clone(), {
                    let mut newcommand = command.1.clone();
                    newcommand.push(".");
                    newcommand
                }),
                environment,
                homedir,
            );
        } else if files.len() == 1 {
            for file_str in files {
                let file_vec: Vec<PathBuf> = read_dir(file_str)
                    .unwrap()
                    .map(Result::unwrap)
                    .map(|x: DirEntry| PathBuf::from(x.file_name()))
                    .collect();

                let sorted: Vec<PathBuf> = sort_files_dirs(file_vec, dirs_first);

                if all {
                    if color {
                        out.push_str("\x1b[34m");
                    }
                    out.push('.');
                    if dir_indicator {
                        out.push('/');
                    }
                    out.push_str("..");
                    if dir_indicator {
                        out.push('/');
                    }
                    if color {
                        out.push_str("\x1b[0m");
                    }
                }

                for file in sorted {
                    println!("{}", file.to_str().unwrap());
                    if file.is_dir() {
                        if color {
                            out.push_str("\x1b[34m");
                        }
                        out.push_str(file.to_str().unwrap());
                        if dir_indicator {
                            out.push('/');
                        }
                        if color {
                            out.push_str("\x1b[0m");
                        }
                    } else {
                        out.push_str(file.to_str().unwrap());
                    }
                }
            }
        } else {
        }

        println!("{}", out);

        (0, false)
    }

    ///
    ///
    /// # Arguments
    ///
    /// * `command`: full command `(String, Vec<&str>)`
    /// * `environment`: environment variables
    ///
    /// returns: `(i16, bool)` (exit code, should exit)
    fn cd(
        &mut self,
        command: &(String, Vec<&str>),
        environment: &HashMap<String, String>,
    ) -> (i16, bool) {
        if command.1.is_empty() {
            return self.call_command(
                &("cd".to_owned(), vec!["~"]),
                environment,
                &*self.cwd.clone(),
            );
        }

        let path_pbuf: PathBuf;

        if command.1[0].starts_with('/') {
            path_pbuf = Path::new(command.1[0]).to_owned();
        } else if command.1[0].starts_with('~') {
            path_pbuf = self.homedir.join(command.1[0].split_once('~').unwrap().1);
        } else {
            path_pbuf = self.cwd.join(command.1[0]);
        }

        return if path_pbuf.is_file() {
            println!(
                "cd: Cannot change directory into a file: {}",
                path_pbuf.file_name().unwrap().to_str().unwrap()
            );
            (-1, false)
        } else {
            self.cwd = path_pbuf;
            (0, false)
        };
    }

    /// Convert an absolute path to one that references home directory ('~')
    ///
    /// # Arguments
    ///
    /// * `path`: path to simplify
    ///
    /// returns: String
    fn simplify_display(&mut self, path: PathBuf) -> String {
        return if path.starts_with(self.homedir.clone()) {
            Path::new("~")
                .to_owned()
                .join(path.strip_prefix(self.homedir.clone()).unwrap())
                .to_str()
                .unwrap()
                .to_owned()
        } else {
            path.to_str().unwrap().to_owned()
        };
    }

    /// Clear the screen
    fn clear() -> (i16, bool) {
        flush();
        print!("\x1B[2J\x1B[1;1H");
        flush();

        (0, false)
    }

    /// Create a new directory
    ///
    /// # Arguments
    ///
    /// * `command`:
    /// * `environment`:
    ///
    /// returns: (i16, bool)
    fn mkdir(
        &mut self,
        command: &(String, Vec<&str>),
        _environment: &HashMap<String, String>,
    ) -> (i16, bool) {
        return if command.1.is_empty() {
            println!("mkdir: requires an argument");
            (-1, false)
        } else if command.1.len() > 1 {
            println!("mkdir: requires exactly one argument");
            (-1, false)
        } else {
            let path_pbuf: PathBuf;

            if command.1[0].starts_with('/') {
                path_pbuf = Path::new(command.1[0]).to_owned();
            } else if command.1[0].starts_with('~') {
                path_pbuf = self.homedir.join(command.1[0].split_once('~').unwrap().1);
            } else {
                path_pbuf = self.cwd.join(command.1[0]);
            }

            if path_pbuf.exists() {
                println!("mkdir: path {} already exists", path_pbuf.to_str().unwrap());
                (-1, false)
            } else {
                match fs::create_dir_all(path_pbuf.join(command.1[0])) {
                    Ok(_) => (),
                    Err(_) => println!("mkdir: Access denied"),
                }
                (0, false)
            }
        };
    }

    /// Print help message
    fn help() -> (i16, bool) {
        println!("QuantII SHell - Help\n\nAvailable commands:\n");
        println!("\tcd <dir>\t\t\tChange directory");
        println!("\tls [dir]\t\t\t\tList directory");
        println!("\tclear\t\t\t\tClear the screen");
        println!("\tmkdir <dir>\t\t\tMake a new directory");
        println!("\trmdir <dir>\t\t\tRemove a directory");
        println!("\trm <file> [files...]\t\t\tRemove a file");
        println!("\tcat <file> [files...]\t\t\tPrint a file");
        println!("\thelp\t\t\t\tDisplay this help");
        println!("\texit\t\t\t\tExit QuantII SHell");
        (0, false)
    }

    /// Print to standard output
    fn echo(&self, command: (String, Vec<&str>)) -> (i16, bool) {
        let mut output = String::new();

        for arg in command.1 {
            output.push_str(arg);
            output.push(' ');
        }

        println!("{}", output);

        (0, false)
    }

    /// Print the working directory
    fn pwd(&self, _command: (String, Vec<&str>), _env: &HashMap<String, String>) -> (i16, bool) {
        println!("{}", self.cwd.to_str().unwrap());
        (0, false)
    }
}

///
///
/// # Arguments
///
/// * `qiish`: current Qiish instance
///
/// returns: `HashMap<String, String>`
fn get_env(qiish: &Qiish) -> HashMap<String, String> {
    let mut variables: HashMap<String, String> = HashMap::new();

    println!("{}", qiish.qiishenv.to_str().unwrap());

    let qiishenv_contents_raw: String = match fs::read_to_string(&qiish.qiishenv) {
        Ok(contents) => contents,
        Err(_) => "Could not load /dev/home/.qiishenv".to_owned(),
    };

    let qiishenv_contents: Map<Lines, fn(&_) -> String> = qiishenv_contents_raw
        .lines()
        .into_iter()
        .map(ToString::to_string);

    for line in qiishenv_contents {
        let key_val: (String, String) = line
            .split_once('=')
            .map(|(k, v)| (k.to_owned(), v.to_owned()))
            .unwrap();

        variables.insert(key_val.0, key_val.1);
    }

    variables
}

/// just run `stdout().flush()`
#[inline(always)]
fn flush() {
    stdout().flush().ok();
}

///
///
/// # Arguments
///
/// * `path`: path to clean up
///
/// returns: Result<PathBuf, Error>
///
/// panics! if cwd contains `./`,
/// causing infinite recursion
/// Stack overflow
pub fn rewrite_relative_dir(path: PathBuf, homedir: &Path, cwd: &Path) -> Result<PathBuf> {
    let mut iter = path.components();
    let abs_path = match iter.next() {
        Some(Component::Normal(osstr)) if osstr == "~" => homedir.join(iter.collect::<PathBuf>()),
        Some(Component::Normal(_) | Component::CurDir | Component::ParentDir) => cwd.join(path),
        _ => path,
    };
    canonicalize(&*abs_path)
}

/// Sort the given list of files by name
#[must_use]
pub fn sort_files_dirs(paths: Vec<PathBuf>, dir_first: bool) -> Vec<PathBuf> {
    let mut files: Vec<PathBuf> = Vec::new();
    let mut dirs: Vec<PathBuf> = Vec::new();

    for path in paths {
        if path.is_dir() {
            dirs.push(path);
        } else {
            files.push(path);
        }
    }

    dirs.append(files.as_mut());
    if !dir_first {
        dirs.sort();
    }
    dirs
}
