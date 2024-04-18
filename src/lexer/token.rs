use core::fmt;
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Token {
    Keyword(Keyword),
    Ident(String),
    Op(String),
    Newline,
}

impl Display for Token {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Token::Keyword(kw) => kw.to_string(),
                Token::Ident(ident) => ident.to_string(),
                Token::Op(s) => s.to_string(),
                Token::Newline => "end of line".to_string(),
            }
        )
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Copy)]
pub enum GroupKind {
    Parentheses,
    Brackets,
    Braces,
}

#[derive(Debug, Clone, PartialEq, Eq, Copy)]
pub enum Keyword {
    Start,
    Default,
}

impl Display for Keyword {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Keyword::Start => "start",
                Keyword::Default => "default",
            }
        )
    }
}
