#![warn(clippy::all)]
#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]
#![warn(clippy::cargo)]
#![warn(clippy::suspicious)]

use crate::{
    lex::{Token, Tokens},
    lookahead::Lookahead,
    Options,
};
use log::info;

#[derive(Clone, Debug, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum ParsedToken {
    Command(String, Vec<String>),
    Redir(String),
    Pipe(usize, usize),
    HereDoc(usize, String),
    HereString(usize, String),
    RedirClobber(String),
    RedirAppend(String),
    RedirInput(String),
    AndAnd(usize, usize),
    OrOr(usize, usize),
    Eof,
    Error(String),
    String(String),
    Delimiter,
}

pub type ParsedTokens = Vec<ParsedToken>;
pub type TokenStream = Lookahead<Token>;

#[allow(clippy::too_many_lines)]
pub fn parse(mut in_: Tokens, options: Options) -> (i32, ParsedTokens) {
    let mut parsed_tokens: Vec<ParsedToken> = vec![];
    // Temporary until EOF token is implemented
    // TODO: REMOVE THIS
    in_.push(Token::Eof);
    let mut token_stream: TokenStream = <TokenStream>::from(in_);

    if options.verbose {
        info!("Parsing tokens...");
    }

    let iter = token_stream.clone();

    for tok in iter {
        info!("Parsing token: {:?}", tok);
        match tok {
            Token::AndAnd => {
                let left = parsed_tokens.len() - 1;
                let right = parsed_tokens.len() + 1;
                parsed_tokens.push(ParsedToken::AndAnd(left, right));
            }
            // Currently, EoF token is not implemented
            Token::Eof => parsed_tokens.push(ParsedToken::Eof),
            Token::Error => {
                parsed_tokens.push(ParsedToken::Error("Error parsing token".to_string()));
            }
            Token::HereDoc => {
                let mut here_doc = String::new();
                let mut end_token = String::new();
                let mut end_token_found = false;
                let mut end_token_len = 0;

                token_stream.forward(1);
                if let Some(Token::Text(s)) = token_stream.current() {
                    end_token = s.clone();
                    end_token_len = s.len();
                }

                token_stream.forward(1);
                while let Some(Token::Text(s)) = token_stream.current() {
                    if s == end_token {
                        end_token_found = true;
                        break;
                    }
                    here_doc.push_str(&s);
                    token_stream.forward(1);
                }

                if end_token_found {
                    token_stream.forward(end_token_len);
                    parsed_tokens.push(ParsedToken::HereDoc(parsed_tokens.len(), here_doc));
                } else {
                    parsed_tokens.push(ParsedToken::Error(
                        "HereDoc: End token not found".to_string(),
                    ));
                }
            }
            Token::HereString => {
                let here_string;

                token_stream.forward(1);
                if let Some(Token::Text(s)) = token_stream.current() {
                    here_string = s.clone();
                } else {
                    parsed_tokens.push(ParsedToken::Error(
                        "HereString: No string found".to_string(),
                    ));
                    continue;
                }

                parsed_tokens.push(ParsedToken::HereString(parsed_tokens.len(), here_string));
            }
            Token::OrOr => {
                let left = parsed_tokens.len() - 1;
                let right = parsed_tokens.len() + 1;
                parsed_tokens.push(ParsedToken::OrOr(left, right));
            }
            Token::Pipe => {
                let left = parsed_tokens.len() - 1;
                let right = parsed_tokens.len() + 1;
                parsed_tokens.push(ParsedToken::Pipe(left, right));
            }
            Token::Redir => {
                token_stream.forward(1);
                let next = token_stream.current();
                if let Some(Token::Text(s)) = next {
                    parsed_tokens.push(ParsedToken::Redir(s.to_string()));
                } else {
                    parsed_tokens.push(ParsedToken::Error(
                        "Expected a filename after redirection".to_string(),
                    ));
                }
            }
            Token::RedirAppend => {
                token_stream.forward(1);
                let next = token_stream.current();
                if let Some(Token::Text(s)) = next {
                    parsed_tokens.push(ParsedToken::RedirAppend(s.to_string()));
                } else {
                    parsed_tokens.push(ParsedToken::Error(
                        "Expected a filename after redirection".to_string(),
                    ));
                }
            }
            Token::RedirClobber => {
                token_stream.forward(1);
                let next = token_stream.current();
                if let Some(Token::Text(s)) = next {
                    parsed_tokens.push(ParsedToken::RedirClobber(s.to_string()));
                } else {
                    parsed_tokens.push(ParsedToken::Error(
                        "Expected a filename after redirection".to_string(),
                    ));
                }
            }
            Token::RedirInput => {
                token_stream.forward(1);
                let next = token_stream.current();
                if let Some(Token::Text(s)) = next {
                    parsed_tokens.push(ParsedToken::RedirInput(s.to_string()));
                } else {
                    parsed_tokens.push(ParsedToken::Error(
                        "Expected a filename after redirection".to_string(),
                    ));
                }
            }
            Token::DQString(s) => {
                let mut s_chars = s.chars().peekable();
                let mut new_s = String::new();

                while let Some(c) = s_chars.next() {
                    if c == '\\' {
                        if let Some(next) = s_chars.peek() {
                            match next {
                                '$' => {
                                    s_chars.next();
                                    new_s.push('$');
                                }
                                '`' => {
                                    s_chars.next();
                                    new_s.push('`');
                                }
                                '"' => {
                                    s_chars.next();
                                    new_s.push('"');
                                }
                                '\\' => {
                                    s_chars.next();
                                    new_s.push('\\');
                                }
                                '!' => {
                                    s_chars.next();
                                    new_s.push_str("\\!");
                                }
                                _ => {
                                    new_s.push('\\');
                                    new_s.push(c);
                                }
                            }
                        }
                    }
                    if c == '`' {
                        new_s.push_str("\\`");
                    } else if c == '$' {
                        if let Some(next) = s_chars.peek() {
                            if *next == '(' {
                                s_chars.next();
                                new_s.push_str("\\$(");
                            } else if *next == '{' {
                                s_chars.next();
                                new_s.push_str("\\${");
                            } else {
                                new_s.push(c);
                            }
                        }
                    }

                    if let Some(ParsedToken::String(s)) = parsed_tokens.last_mut() {
                        s.push(c);
                    } else {
                        parsed_tokens.push(ParsedToken::String(c.to_string()));
                    }
                }
            }
            Token::SQString(s) => {
                if let Some(ParsedToken::String(s1)) = parsed_tokens.last_mut() {
                    s1.push_str(&s);
                } else {
                    parsed_tokens.push(ParsedToken::String(s));
                }
            }
            Token::Text(s) => {
                if let Some(ParsedToken::Command(s1, args)) = parsed_tokens.last_mut() {
                    args.push(s.clone());
                } else {
                    parsed_tokens.push(ParsedToken::Command(s.clone(), vec![]));
                }
            }
            Token::Space => {
                #[allow(clippy::equatable_if_let)]
                if let Some(ParsedToken::String(s)) = parsed_tokens.last_mut() {
                    s.push(' ');
                } else if let Some(ParsedToken::Delimiter) = parsed_tokens.last_mut() {
                    continue;
                } else if let Some(ParsedToken::Command(_, _)) = parsed_tokens.last_mut() {
                    continue;
                }
                parsed_tokens.push(ParsedToken::Delimiter);
            }
        }
    }

    if options.verbose {
        info!("Finished parsing tokens.");
        info!(
            "Parsed tokens: {:?}",
            parsed_tokens
                .clone()
                .into_iter()
                .map(|x| format!("{:?}", x))
                .collect::<Vec<String>>()
        );
    }

    (0, parsed_tokens)
}
