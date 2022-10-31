use crate::Options;
use std::path::PathBuf;

/// The environment passed into all commands
pub struct Environment {
    pub args: Vec<String>,
    pub options: Options,
    pub cwd: PathBuf,
}

pub mod parse_args {
    use std::collections::HashMap;

    #[allow(clippy::implicit_hasher)]
    #[must_use]
    pub fn parse(
        args: Vec<String>,
        name_map: &HashMap<String, String>,
    ) -> (Vec<Option<String>>, Vec<String>) {
        let (mut ret_args, mut ret_params): (Vec<Option<String>>, Vec<String>) = (vec![], vec![]);

        for arg in args {
            if arg.starts_with('-') {
                let mut iter = arg.chars().peekable();
                // Skip the first '-'
                iter.next();

                if iter.peek() == Some(&'-') {
                    // Double dash

                    // Skip the second dash
                    iter.next();
                    let mut arg_name = String::new();
                    for c in iter.by_ref() {
                        arg_name.push(c);
                    }
                    if let Some(name) = name_map.get(&arg_name) {
                        ret_args.push(Some(name.to_string()));
                    } else {
                        ret_args.push(None);
                    }
                } else {
                    iter.next();
                    for c in iter {
                        if let Some(name) = name_map.get(&c.to_string()) {
                            ret_args.push(Some(name.to_string()));
                        } else {
                            ret_args.push(None);
                        }
                    }
                }
            } else {
                ret_params.push(arg);
            }
        }

        (ret_args, ret_params)
    }
}

impl Into<Environment> for (Vec<Option<String>>, Vec<String>) {
    fn into(self) -> Environment {
        Environment {
            args: self.1,
            options: self.0,
            cwd: std::env::current_dir().unwrap_or_else(|_| std::path::PathBuf::from("/"),
        }
    }
}

/// LS (`LiSt` files) command
pub mod ls {
    #[must_use]
    pub fn ls(environment: &super::Environment) -> i32 {
        0
    }
}
