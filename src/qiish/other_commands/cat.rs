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


use std::io::{Read, stdin};
use std::path::{Path, PathBuf};


/// Data for CAT command
struct DataCAT {
    /// Files to print
    files: Vec<Input>,
}

///
///
/// # Arguments
///
/// * `command`:
/// * `homedir`:
/// * `cwd`:
///
/// returns: (i16, bool)
///
/// # Examples
///
/// ```
///
/// ```
pub fn cat(command: (String, Vec<&str>), homedir: &Path, cwd: &Path) -> (i16, bool) {
    let (_, args) = command;
    let mut data = DataCAT { files: Vec::new() };

    let mut i: usize = 0;
    while i < args.len() {
        let arg = args[i];
        if arg.starts_with('-') {
            if arg == "-" {
                data.files.push(Input::Stdin);
            } else {
                println!("cat: invalid option '{}'", arg);
                return (-1, false);
            }
        } else {
            let path = Path::new(arg);
            if path.is_relative() {
                data.files.push(Input::File(cwd.join(path)));
            } else {
                data.files.push(Input::File(homedir.join(path)));
            }
        }
        i += 1;
    }
    let result = 0;
    for file_in in data.files {
        match file_in {
            Input::File(path) => {
                let mut file = std::fs::File::open(path).unwrap();
                let mut buffer = String::new();
                file.read_to_string(&mut buffer).unwrap();
                println!("{}", buffer);
            }
            Input::Stdin => {
                let mut buffer = String::new();
                let mut good: bool = true;
                while good {
                    match stdin().read_line(&mut buffer) {
                        Ok(0) => break,
                        Ok(_) => {
                            println!("{}", buffer);
                            buffer.clear();
                        }
                        Err(_) => {
                            good = false;
                        }
                    };
                }
            }
        }
    }
    (result, false)
}

enum Input { File(PathBuf), Stdin }