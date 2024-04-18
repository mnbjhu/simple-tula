use chumsky::{select, Parser};

use crate::lexer::{
    token::Token,
    util::{Extra, ParserInput},
};

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct Condition {
    pub state: String,
    pub symbol: String,
}

pub fn condition_parser<'token, 'src: 'token>(
) -> impl Parser<'src, ParserInput<'src>, Condition, Extra<'src>> {
    let ident = select! {
        Token::Ident(ident) => ident,
    };
    ident
        .labelled("state")
        .then(ident.labelled("symbol"))
        .map(|(state, symbol)| Condition { state, symbol })
}
