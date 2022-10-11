#![warn(clippy::all)]
#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]
#![warn(clippy::cargo)]
#![warn(clippy::suspicious)]


use crate::lex::{Token, Tokens};
use crate::lookahead::Lookahead;
use crate::Options;
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
}

pub type ParsedTokens = Vec<ParsedToken>;
pub type TokenStream = Lookahead<Token>;

pub fn parse(mut in_: Tokens, options: Options) -> (i32, ParsedTokens) {
    let mut parsed_tokens: Vec<ParsedToken> = vec![];
    // Temporary until EOF token is implemented
    in_.push(Token::Eof);
    let mut token_stream: TokenStream = in_.into();

    if options.verbose {
        info!("Parsing tokens...");
    }

    for tok in &mut token_stream {
        info!("Parsing token: {:?}", tok);
        match tok {
            Token::AndAnd => {
                let left = parsed_tokens.len() - 1;
                let right = parsed_tokens.len() + 1;
                parsed_tokens.push(ParsedToken::AndAnd(left, right));
            }
            // Currently, EoF token is not implemented
            Token::Eof => parsed_tokens.push(ParsedToken::Eof),
            Token::Error => {}
            Token::HereDoc => {}
            Token::HereString => {}
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
                let mut next = token_stream.current();
                if let Some(Token::Text(s)) = next {
                    parsed_tokens.push(ParsedToken::Redir(s.to_string()));
                } else {
                    parsed_tokens.push(ParsedToken::Error(
                        "Expected a filename after redirection".to_string(),
                    ));
                }
            }
            Token::RedirAppend => {}
            Token::RedirClobber => {}
            Token::RedirInput => {}
            Token::DQString(s) => {
                token_stream.backward(1);
                if token_stream.current()
            }
            Token::SQString(s) => {}
            Token::Text(_) => {}
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
