use chumsky::{primitive::just, select, IterParser, Parser};

use crate::lexer::{
    token::Token,
    util::{Extra, ParserInput},
};

pub fn tape_parser<'token, 'src: 'token>(
) -> impl Parser<'src, ParserInput<'src>, Vec<String>, Extra<'src>> {
    let ident = select! {
        Token::Ident(ident) => ident,
    };
    ident.repeated().collect().then_ignore(just(Token::Newline))
}
