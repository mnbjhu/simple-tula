use std::collections::HashMap;

use chumsky::{primitive::just, IterParser, Parser};

use crate::{
    lexer::{
        token::Token,
        util::{Extra, ParserInput},
    },
    parser::{
        action::{action_parser, Action},
        condition::{condition_parser, Condition},
    },
};

pub fn machine_parser<'token, 'src: 'token>(
) -> impl Parser<'src, ParserInput<'src>, HashMap<Condition, Action>, Extra<'src>> {
    condition_parser()
        .then(action_parser())
        .separated_by(just(Token::Newline))
        .allow_trailing()
        .collect()
}
