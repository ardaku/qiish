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
clippy::cargo,
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
extern crate dirs_next;
// section uses
// extern crate quantii;
extern crate std;

// section uses

mod other_commands;
use other_commands::rm::rm;

use alloc::string::String;
use alloc::string::ToString;
use std::{
    fs,
    io,
    collections::HashMap,
    fs::ReadDir,
    io::{Error, ErrorKind, stdin, stdout, Write},
    iter::Map,
    path::{Path, PathBuf},
    print,
    println,
    str::Lines,
    vec,
    vec::Vec
};
use std::fs::canonicalize;
use std::path::Component;


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
    pub fn new(home_dir: &Path) -> io::Result<Self> {

        let qiishenv_loc = if let Ok(path) = rewrite_relative_dir(
            home_dir.join(".qiishenv"),
            home_dir,
            &*rewrite_relative_dir(home_dir.to_owned(),
                                 home_dir,
                                 home_dir)
                .unwrap()
        ) {
            path
        } else {
            println!("Failed to instantiate Qiiish: no such file or directory: {}", home_dir.to_owned()
                .join(".qiishenv")
                .to_str()
                .unwrap());
            return Err(Error::new(ErrorKind::Other, format!("No such directory {}", home_dir.to_str().unwrap())));
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
            None => "comp_name_unk"
        };


        let username: &str = match env.get("computername") {
            Some(name) => name,
            None => "user_name_unk"
        };

        flush();


        while !exit {
            print!("{}#{}@{} {} % ", computer_name, username, entrance_code, self.simplify_display(self.cwd.clone()));
            flush();

            let mut line: String = String::new();
            let _ = stdin().read_line(&mut line);

            let command: (&str, &str) = match line.split_once(' ') {
                Some((before, after)) => (before, after),
                None => (line.as_str(), "")
            };


            let full_command: (String, Vec<&str>) = (command.0.trim().to_owned(), command.1.split_whitespace().collect());
            let exit_code: (
                i16, // Exit code itself
                bool // Whether or not the shell should exit
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
    fn call_command(&mut self, command: &(String, Vec<&str>),
                    environment: &HashMap<String, String>, cwd: &Path) -> (i16,
                                                               bool) {
        match command.0.as_str() {
            "" => (0, false),
            "exit" => (0, true),
            "cd" => self.cd(command, environment),
            "ls" => self.ls(command, environment),
            "clear" => Self::clear(),
            "mkdir" => self.mkdir(command, environment),
            "rm" => rm(command.clone(), &self.homedir, cwd),
            "rmdir" => {
                rm((command.0.clone(), {
                    let mut args = command.1.clone(); args.insert(0, "-r"); args
                }), &self.homedir, cwd)
            },
            _ => {
                println!("Unrecognized command: {}", command.0);
                (-1, false)
            }
        }
    }

    // section builtins

    ///
    ///
    /// # Arguments
    ///
    /// * `command`: full command `(String, Vec<&str>)`
    /// * `environment`: environment variables
    ///
    /// returns: `(i16, bool)` (exit code, should exit)
    fn ls(&mut self, command: &(String, Vec<&str>),
          environment: &HashMap<String, String>) -> (i16,
                                                     bool) {
        if command.1.is_empty() {
            let cwd_str = self.cwd.to_str().unwrap().to_owned();
            let ret = self.call_command(&("ls".to_owned(), vec![cwd_str.as_str()]), environment, &*self.cwd.clone());
            return ret;
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
            println!("ls: Cannot read file as directory: {}",
                     path_pbuf.file_name()
                         .unwrap()
                         .to_str()
                         .unwrap());
            (-1, false)
        } else {
            let paths: ReadDir = fs::read_dir(path_pbuf).unwrap();

            for path_c in paths {
                print!("{} ", path_c.unwrap().path().display());
            }

            (0, false)
        };
    }

    ///
    ///
    /// # Arguments
    ///
    /// * `command`: full command `(String, Vec<&str>)`
    /// * `environment`: environment variables
    ///
    /// returns: `(i16, bool)` (exit code, should exit)
    fn cd(&mut self, command: &(String, Vec<&str>),
          environment: &HashMap<String, String>) -> (i16, bool) {
        if command.1.is_empty() {
            return self.call_command(&("cd".to_owned(), vec!["~"]), environment, &*self.cwd.clone());
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
            println!("cd: Cannot change directory into a file: {}",
                     path_pbuf.file_name()
                         .unwrap()
                         .to_str()
                         .unwrap());
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
                .join(path.strip_prefix(
                    self.homedir.clone()).unwrap())
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
    fn mkdir(&mut self, command: &(String, Vec<&str>), _environment: &HashMap<String, String>) -> (i16, bool) {
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
        }
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

    let qiishenv_contents_raw: String =
        match fs::read_to_string(&qiish.qiishenv) {
            Ok(contents) => contents,
            Err(_) => "Could not load /dev/home/.qiishenv".to_owned()
        };

    let qiishenv_contents: Map<Lines, fn(&_) -> String> = qiishenv_contents_raw
        .lines().into_iter()
        .map(ToString::to_string);

    for line in qiishenv_contents {
        let key_val: (String, String) = line
            .split_once('=')
            .map(|(k, v)| {
                (k.to_owned(), v.to_owned())
            })
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
pub fn rewrite_relative_dir(path: PathBuf, homedir: &Path, cwd: &Path) -> io::Result<PathBuf> {
    let mut iter = path.components();
    let abs_path = match iter.next() {
        Some(Component::Normal(osstr)) if osstr == "~" => homedir.join(iter.collect::<PathBuf>()),
        Some(Component::Normal(_) | Component::CurDir | Component::ParentDir) => cwd.join(path),
        _ => path,
    };
    canonicalize(&*abs_path)
}