#![warn(clippy::all)]
#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]
#![warn(clippy::cargo)]
#![warn(clippy::suspicious)]

use logos::Logos;

#[derive(Clone, Debug, Ord, PartialOrd, Eq, PartialEq, Hash, Logos)]
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
    #[error]
    #[regex(r"[ \t\f\n]+", logos::skip)]
    Error,
}

pub type Tokens = Vec<Token>;

