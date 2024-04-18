use chumsky::{primitive::just, Parser};

use crate::lexer::{
    token::Token,
    util::{Extra, ParserInput},
};

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum Direction {
    Left,
    Right,
}

pub fn direction_parser<'token, 'src: 'token>(
) -> impl Parser<'src, ParserInput<'src>, Direction, Extra<'src>> {
    let left = just(Token::Op("<-".to_string())).map(|_| Direction::Left);
    let right = just(Token::Op("->".to_string())).map(|_| Direction::Right);
    left.or(right).labelled("direction")
}
