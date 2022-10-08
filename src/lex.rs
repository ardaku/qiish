#![warn(clippy::all)]
#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]
#![warn(clippy::cargo)]
#![warn(clippy::suspicious)]

use std::fs::File;
use std::io::Read;
use log::info;
use logos::Logos;
use crate::Options;

#[derive(Clone, Debug, Ord, PartialOrd, Eq, PartialEq, Hash, Logos)]
pub enum Token {
    // Redirection
    #[token("|")]
    Pipe,
    #[token("<<")]
    HereDoc,
    #[token("<<<")]
    HereString,
    #[token(">")]
    Redir,
    #[token(">|")]
    RedirClobber,
    #[token(">>")]
    RedirAppend,
    #[token("<")]
    RedirInput,

    // Double-quoted string
    #[regex(r#""(\\.|[^"\\])*""#, |lex| lex.slice().to_string())]
    DQString(String),
    // Single-quoted string
    #[regex(r#"'(\\.|[^'\\])*'"#, |lex| lex.slice().to_string())]
    SQString(String),

    // After Command
    #[token("&&")]
    AndAnd,
    #[token("||")]
    OrOr,

    // Special
    // TODO: Implement `Eof` token
    #[allow(dead_code)]
    Eof,
    #[regex(r"[a-zA-Z_][a-zA-Z0-9_]*", |lex| lex.slice().to_string())]
    Text(String),
    #[error]
    #[regex(r"[ \t\f\n]+", logos::skip)]
    Error,
}

pub type Tokens = Vec<Token>;

pub fn lex(input: &str, options: Options) -> (i32, Tokens) {
    let mut input = File::open(input).unwrap();
    let mut in_string = "".to_owned();
    input.read_to_string(&mut in_string).unwrap();
    let tokens = Token::lexer(&in_string).collect();
    (0, tokens)
}