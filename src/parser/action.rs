use chumsky::Parser;

use crate::{
    lexer::util::{Extra, ParserInput},
    parser::{
        condition::{condition_parser, Condition},
        direction::Direction,
    },
};

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct Action {
    pub move_direction: Direction,
    pub change: Condition,
}

pub fn action_parser<'token, 'src: 'token>(
) -> impl Parser<'src, ParserInput<'src>, Action, Extra<'src>> {
    let move_direction = crate::parser::direction::direction_parser();
    move_direction
        .then(condition_parser())
        .map(|(move_direction, condition)| Action {
            move_direction,
            change: condition,
        })
}
