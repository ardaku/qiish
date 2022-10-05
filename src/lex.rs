#![warn(clippy::all)]
#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]
#![warn(clippy::cargo)]
#![warn(clippy::suspicious)]

use log::error;
use std::fs::File;
use std::str::Chars;

#[derive(Clone, Debug, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum Token {
    // Redirection
    Pipe,
    HereDoc,
    HereString,
    Redir,
    RedirClobber,
    RedirAppend,
    RedirInput,

    String(String),

    // After Command
    AndAnd,
    OrOr,

    // Special
    Eof,
    PresumedCommand,
    PresumedOption,
    PresumedArgument,
}

pub type Tokens = Vec<Token>;

#[allow(clippy::too_many_lines)]
pub fn lex(infile: &str, options: super::Options) -> (i32, Tokens) {
    let mut tokens: Tokens = vec![];

    let infile = match File::open(infile) {
        Ok(file) => file,
        Err(e) => {
            eprintln!("{}", e);
            return (1, tokens);
        }
    };

    error!("{:?}, {:?}", infile, options);

    let file = std::io::read_to_string(infile).unwrap();
    let mut file_chars = file.chars();

    let mut in_double_string: bool = false;
    let mut in_single_string: bool = false;

    let mut j: usize = 0;

    for (mut i, c) in file_chars.clone().enumerate() {
        error!("{}: {}", i, c);
        if in_double_string && in_single_string {
            unreachable!("Cannot be in both single- and double-quoted string");
        }

        if options.verbose {
            dbg!("{}: {}", i, c);
        }

        if in_double_string {
            let string = match &mut tokens[j] {
                Token::String(content) => content,
                _ => unreachable!(),
            };
            match c {
                '\\' => match file_chars
                    .nth(i + 1)
                    .expect("ERROR: Could not index `file_chars`: unexpected end of string")
                {
                    '"' => {
                        string.push('\"');
                    }

                    'n' => {
                        string.push('\n');
                    }

                    'r' => {
                        string.push('\r');
                    }

                    't' => {
                        string.push('\t');
                    }

                    '\\' => {
                        string.push('\\');
                    }

                    _ => {
                        eprintln!(
                            "ERROR: Unknown escape code: {}",
                            file_chars.nth(i + 1).expect(
                                "ERROR: Could not index 1file_chars1: unexpected end of string"
                            )
                        );
                    }
                },
                '"' => {
                    j += 1;
                    in_double_string = false;
                }
                x => {
                    string.push(x);
                }
            }
        } else if in_single_string {
            match in_single_string_fn(
                &mut tokens,
                &mut j,
                &mut i,
                &file_chars,
                &mut in_single_string,
            ) {
                Ok(_) => {}
                Err(e) => return (e, tokens),
            }
        } else {
            match c {
                ' ' => {
                    j += 1;
                }
                '"' => {
                    tokens.push(Token::String(String::new()));
                    j += 1;
                    in_double_string = true;
                }
                '\'' => {
                    tokens.push(Token::String(String::new()));
                    j += 1;
                    in_single_string = true;
                }
                '|' => {
                    if file_chars
                        .nth(i + 1)
                        .expect("ERROR: Could not index `file_chars`: unexpected end of string")
                        == '|'
                    {
                        tokens.push(Token::OrOr);
                    } else {
                        tokens.push(Token::Pipe);
                    }
                    j += 1;
                }
                '>' => {
                    if file_chars
                        .nth(i + 1)
                        .expect("ERROR: Count not index `file_chars`: unexpected end of string")
                        == '|'
                    {
                        tokens.push(Token::RedirClobber);
                    } else if file_chars
                        .nth(i + 1)
                        .expect("ERROR: Count not index `file_chars`: unexpected end of string")
                        == '>'
                    {
                        tokens.push(Token::RedirAppend);
                    } else {
                        tokens.push(Token::Redir);
                    }
                }

                '<' => {
                    match file_chars
                        .nth(i + 1)
                        .expect("ERROR: Could not index `file_chars`: unexpected end of string")
                    {
                        '<' => {
                            if file_chars.nth(i + 2).expect(
                                "ERROR: Could not index `file_chars`: unexpected end of string",
                            ) == '<'
                            {
                                tokens.push(Token::HereString);
                            } else {
                                tokens.push(Token::HereDoc);
                            }
                        }
                        _ => {
                            tokens.push(Token::RedirInput);
                        }
                    }
                    j += 1;
                }
                '&' => {
                    tokens.push(Token::AndAnd);
                    j += 1;
                }
                _ => {
                    continue;
                }
            }
        }
    }

    (0, tokens)
}

fn in_single_string_fn(
    tokens: &mut Tokens,
    mut j: &mut usize,
    i: &mut usize,
    chars: &Chars,
    in_single_string: &mut bool,
) -> Result<(), i32> {
    let string = match &mut tokens[*j] {
        Token::String(content) => content,
        _ => unreachable!(),
    };
    match chars
        .clone()
        .nth(*i + 1)
        .expect("ERROR: Could not index `file_chars`: unexpected end of string")
    {
        '\\' => match chars
            .clone()
            .nth(*i + 1)
            .expect("ERROR: Could not index `file_chars`: unexpected end of string")
        {
            '\'' => {
                string.push('\'');
            }

            'n' => {
                string.push('\n');
            }

            'r' => {
                string.push('\r');
            }

            't' => {
                string.push('\t');
            }

            '\\' => {
                string.push('\\');
            }

            _ => {
                eprintln!(
                    "ERROR: Unknown escape code: {}",
                    chars
                        .clone()
                        .nth(*i + 1)
                        .expect("ERROR: Could not index `file_chars`: unexpected end of string")
                );
                return Err(1);
            }
        },
        '\'' => {
            (*j) += 1;
            *in_single_string = false;
        }
        x => {
            string.push(x);
        }
    }
    Ok(())
}
