use std::collections::HashMap;

use chumsky::{primitive::just, Parser};

use crate::{
    lexer::{
        token::Token,
        util::{Extra, ParserInput},
    },
    parser::{
        action::Action, condition::Condition, default::default_parser, machine::machine_parser,
        start::start_parser,
    },
};

pub struct File {
    pub start: String,
    pub default: String,
    pub machine: HashMap<Condition, Action>,
}

pub fn file_parser<'token, 'src: 'token>() -> impl Parser<'src, ParserInput<'src>, File, Extra<'src>>
{
    start_parser()
        .then_ignore(just(Token::Newline))
        .then(default_parser())
        .then_ignore(just(Token::Newline))
        .then(machine_parser())
        .map(|((start, default), machine)| File {
            start,
            default,
            machine,
        })
}
