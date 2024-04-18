use chumsky::{primitive::just, select, Parser};

use crate::lexer::{
    token::{Keyword, Token},
    util::{Extra, ParserInput},
};

pub fn start_parser<'token, 'src: 'token>(
) -> impl Parser<'src, ParserInput<'src>, String, Extra<'src>> {
    let ident = select! {
        Token::Ident(ident) => ident,
    };
    just(Token::Keyword(Keyword::Start)).ignore_then(ident)
}
