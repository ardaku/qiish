#![warn(clippy::all)]
#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]
#![warn(clippy::cargo)]
#![warn(clippy::suspicious)]

use crate::Options;
use log::info;
use logos::Logos;
use std::fs::File;
use std::io::Read;

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
    #[regex(r"[ ]+")]
    Space,
    #[error]
    #[regex(r"[\t\f\n]+", logos::skip)]
    Error,
}

pub type Tokens = Vec<Token>;

#[must_use] pub fn lex(input: &str, _options: Options) -> (i32, Tokens) {
    let tokens = Token::lexer(input).collect();
    (0, tokens)
}
