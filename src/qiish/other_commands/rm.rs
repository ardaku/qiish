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

use std::fs::remove_dir_all;
use std::path::Path;
#[allow(unused_imports)]
use std::str::FromStr;
use crate::qiish::rewrite_relative_dir;

/// Data used in [rm]
#[derive(Clone)]
struct DataRM {
    /// -r, -R, --recursive
    recursive: bool,
    /// files and dirs to remove
    to_remove: Vec<String>,
}

/// Removes a file or directory. Shouldn't be called directly.
///
/// # Arguments
///
/// * `command`: item \[0] is command types, item \[1]
/// is vector of arguments.
///
/// returns: (i16, bool)
///
#[allow(clippy::needless_pass_by_value)]
pub fn rm(command: (String, Vec<&str>), homedir: &Path, cwd: &Path) -> (i16, bool) {
    let rm_data: DataRM = match DataRM::generate_rm_data(command.1.clone()) {
        None => return (-1, false),
        Some(data) => data
    };
    let mut exit: i16 = 0;

    for file in rm_data.to_remove {
        #[allow(clippy::single_match_else)]
        let file_path = &*match rewrite_relative_dir(
            Path::new(&file).to_owned(),
            &*homedir, cwd.clone()) {
            Ok(pathbuf) => pathbuf,
            Err(_) => {
                return (-1, false)
            }
        };

        if !file_path.exists() {
            println!("rm: No such file or directory: {}", file);
        } else if file_path.is_dir() && !rm_data.recursive {
            println!("rm: Cannot remove directory: {}", file);
            println!("rm: hint: Use --recursve, -r, or -R option to remove directory recursively");
        } else if remove_dir_all(file_path).is_ok() {
            exit = 0 ;
        } else {
            exit = -1;
            println!("rm: Permission denied: {}", file);
        }
    }

    (exit, false)
}


impl DataRM {

    /// Generates necessary RM data from a [vector](Vec) of arguments.
    ///
    #[allow(clippy::redundant_closure_for_method_calls)]
    #[inline(always)]
    fn generate_rm_data(args: Vec<&str>) -> Option<Self> {


        let mut recursive: bool = false;
        let mut to_remove: Vec<String> = vec![];

        for arg in {
            args.into_iter()
                .map(<String as std::str::FromStr>::from_str)
                .map(|x| x.unwrap())
        } {

            if !arg.starts_with('-') {
                to_remove.push(arg.clone());
            } else if arg == "--recursive" {
                recursive = true;
            } else {
                recursive = arg.contains(
                    'r') || arg.contains('R');
            }
        }

        Option::from(Self {
            recursive,
            to_remove
        })
    }
}
