use crate::parse::{ParsedToken, ParsedTokens};
use crate::Options;
use std::env;
use std::path::Path;

use crate::commands::Environment;

/// Executes the parsed tokens.
pub fn run(tokens: ParsedTokens, options: Options) -> i32 {
    for tok in tokens {
        if let ParsedToken::Command(name, args) = tok.clone() {
            run_command(&tok, options);
        } else {
            log::error!("Invalid token: {:?}", tok);
        }
    }
    0
}

/// Executes a [`ParsedToken::Command`].
fn run_command(command: &ParsedToken, options: Options) -> i32 {
    match command {
        ParsedToken::Command(name, args) => {
            let mut args = args.clone();

            log::info!("{}", name);
            match name.as_str() {
                "ls" => crate::commands::ls::ls(&Environment {
                    args: args.clone(),
                    options,
                    cwd: env::current_dir().unwrap_or_else(|_| std::path::PathBuf::from("/")),
                }),
                _ => {
                    let command_real = find_command(name);
                    if command_real.is_none() {
                        return 127;
                    }
                    let command_real: String = command_real.unwrap();
                    let command_real: &Path = Path::new(&command_real);
                    return execute_command(command_real, args.clone());
                }
            }
        }
        _ => 1,
    }
}

/// Actually executes a command.
fn execute_command(command: &Path, args: Vec<String>) -> i32 {
    std::process::Command::new(command)
        .args(args)
        .spawn()
        .expect("Failed to execute command")
        .wait()
        .expect("Failed to wait on child")
        .code()
        .unwrap_or(1)
}

/// Finds the command in the PATH.
fn find_command(command: &String) -> Option<String> {
    let path = env::var("PATH").unwrap();
    let path = path.split(':').collect::<Vec<&str>>();
    for dir in path {
        let path = format!("{}/{}", dir, command);
        if Path::new(&path).exists() {
            println!("Found command: {}", path);
            return Some(path);
        }
    }
    None
}
