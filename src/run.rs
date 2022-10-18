use std::io::Read;
use std::path::Path;
use crate::Options;
use crate::parse::{ParsedToken, ParsedTokens};

/// Executes the parsed tokens.
pub fn run(tokens: ParsedTokens, options: Options) -> i32 {
    for tok in tokens {
        match tok.clone() {
            ParsedToken::Command(name, args) => {
                run_command(&tok);
            }
            _ => {}
        }
    }
    0
}

/// Executes a [`ParsedToken::Command`].
fn run_command(command: &ParsedToken) -> i32 {
    match command {
        ParsedToken::Command(name, args) => {
            let mut args = args.clone();
            let command_real = find_command(name);

            if command_real.is_none() {
                return 127;
            }

            let command_real = command_real.unwrap();
            let command_real = Path::new(&command_real);

            execute_command(command_real, args)
        }
        _ => 1,
    }
}

fn execute_command(command: &Path, args: Vec<String>) -> i32{

    // TEMPORARY
    // TODO: Make this work for real
    let mut child = unsafe {
        std::process::Command::new(command)
            .args(args)
            .spawn()
            .expect("Failed to execute command")
    };

    let exit_code = child.wait().unwrap();
    exit_code.code().unwrap_or(1)
}

/// Finds the command in the PATH.
fn find_command(command: &String) -> Option<String> {
    let path = std::env::var("PATH").unwrap();
    let path = path.split(':').collect::<Vec<&str>>();
    for dir in path {
        let path = format!("{}/{}", dir, command);
        if std::path::Path::new(&path).exists() {
            println!("Found command: {}", path);
            return Some(path);
        }
    }
    None
}